//! Drive a native compiler and an ORC engine for a local edit session.

use crate::core::TargetProfile;
use crate::jit::JitEngine;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::{c_char, c_int, CString};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::os::fd::AsRawFd;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, ExitStatus, Stdio};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// The dependency object may keep its relocation to __apply across edits. Keep
// that symbol at a stable address and tail-call the current program's version.
const APPLY_TRAMPOLINE: &str = r#"
@__cinch_native_apply_slot = global ptr null

define i64 @__apply(i64 %closure, i64 %argc, ptr %argv) {
  %function = load ptr, ptr @__cinch_native_apply_slot
  %result = musttail call i64 %function(i64 %closure, i64 %argc, ptr %argv)
  ret i64 %result
}
"#;

#[derive(Deserialize, Serialize)]
struct ObjectConfiguration {
    schema: u32,
    target: String,
    object: PathBuf,
    object_sha256: String,
    metadata: PathBuf,
    metadata_sha256: String,
}

struct Compiler {
    child: Child,
    replies: BufReader<ChildStdout>,
    reply: String,
    scratch: tempfile::TempDir,
}

impl Compiler {
    fn start(stage: &Path, source: &Path) -> Result<Self> {
        let stage = stage.canonicalize()?;
        let mut config: ObjectConfiguration =
            serde_json::from_slice(&fs::read(stage.join("base-objects.json"))?)?;
        if config.schema != 1 || config.target != TargetProfile::native().triple {
            return Err("native JIT requires a runtime object configuration for this host".into());
        }

        // Both engines use this private snapshot. In particular, a provider
        // replaced on disk after the compiler's digest checks cannot change
        // the object ORC links against.
        let scratch = tempfile::tempdir()?;
        let object = scratch.path().join("base.o");
        let package = scratch.path().join("base.cpk");
        fs::copy(&config.object, &object)?;
        fs::copy(stage.join("base.cpk"), &package)?;
        config.object = object;
        config.metadata = package;
        let configuration = scratch.path().join("objects.json");
        fs::write(&configuration, serde_json::to_vec(&config)?)?;
        let mut child = Command::new(stage.join("cinch"))
            .arg("--serve-ir")
            .arg("--interfaces")
            .arg(&config.metadata)
            .arg("--objects")
            .arg(configuration)
            .arg(source)
            .arg(scratch.path().join("program.ll"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let replies = BufReader::new(child.stdout.take().expect("compiler stdout is piped"));
        let mut compiler = Self {
            child,
            replies,
            reply: String::new(),
            scratch,
        };
        if compiler.read_reply()? != "ready\n" {
            return Err(format!(
                "native compiler did not admit the package\n{}",
                compiler.diagnostics()
            )
            .into());
        }
        Ok(compiler)
    }

    fn read_reply(&mut self) -> Result<&str> {
        self.reply.clear();
        self.replies.read_line(&mut self.reply)?;
        Ok(&self.reply)
    }

    fn diagnostics(&self) -> String {
        fs::read_to_string(self.scratch.path().join("program.ll.diagnostics")).unwrap_or_default()
    }

    fn pipes(&self) -> [c_int; 2] {
        [
            self.child
                .stdin
                .as_ref()
                .expect("compiler stdin is piped")
                .as_raw_fd(),
            self.replies.get_ref().as_raw_fd(),
        ]
    }

    fn request(&mut self) -> Result<i32> {
        let stdin = self.child.stdin.as_mut().expect("compiler stdin is piped");
        stdin.write_all(b"\n")?;
        let status = self
            .read_reply()?
            .trim()
            .parse::<i32>()
            .map_err(|_| "native compiler returned no valid build status")?;
        if status != 0 {
            eprint!("{}", self.diagnostics());
        }
        Ok(status)
    }

    fn finish(&mut self) -> Result<()> {
        drop(self.child.stdin.take());
        let status = self.child.wait()?;
        if !status.success() {
            return Err(format!("native compiler exited {status}").into());
        }
        Ok(())
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        if !matches!(self.child.try_wait(), Ok(Some(_))) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

struct Program {
    ir: String,
    entry: u64,
}

struct NativeEngine {
    engine: JitEngine,
    apply_slot: *mut usize,
    program: Option<Program>,
}

impl NativeEngine {
    fn start(runtime: &Path, object: &Path) -> Result<Self> {
        let mut engine = JitEngine::new(Some(runtime))?;
        engine.add_ir(APPLY_TRAMPOLINE, "cinch-native-dispatch")?;
        let apply_slot = engine.lookup("__cinch_native_apply_slot")? as *mut usize;
        engine.add_object_file(object)?;
        Ok(Self {
            engine,
            apply_slot,
            program: None,
        })
    }

    fn prepare(&mut self, ir: String) -> Result<u64> {
        if let Some(previous) = &self.program {
            if previous.ir == ir {
                return Ok(previous.entry);
            }
        }
        self.program = None;
        let (entry, apply) = self.engine.replace_entry(&ir)?;
        // ORC owns the pointer-sized global for the lifetime of this engine.
        // No program is running during replacement.
        unsafe { self.apply_slot.write(apply as usize) };
        self.program = Some(Program { ir, entry });
        Ok(entry)
    }
}

fn build_and_run(
    compiler: &mut Compiler,
    engine: &mut NativeEngine,
    arguments: &[*const c_char],
    input: Option<&File>,
) -> Result<i32> {
    let status = compiler.request()?;
    if status != 0 {
        return Ok(status);
    }
    let ir = fs::read_to_string(compiler.scratch.path().join("program.ll"))?;
    let entry = match engine.prepare(ir) {
        Ok(entry) => entry,
        Err(error) => {
            eprintln!("cinch jit: {error}");
            return Ok(1);
        }
    };
    run_in_child(entry, arguments, input, compiler.pipes())
}

fn single_threaded() -> Result<bool> {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn pthread_is_threaded_np() -> c_int;
        }
        Ok(unsafe { pthread_is_threaded_np() == 0 })
    }
    #[cfg(target_os = "linux")]
    {
        let mut threads = fs::read_dir("/proc/self/task")?;
        Ok(threads.next().transpose()?.is_some() && threads.next().transpose()?.is_none())
    }
}

fn run_in_child(
    entry: u64,
    arguments: &[*const c_char],
    input: Option<&File>,
    compiler_pipes: [c_int; 2],
) -> Result<i32> {
    // The resident CLI performs synchronous LLJIT materialization, and
    // never runs a program in the parent. Refuse to call non-async-signal-
    // safe runtime code after forking a multithreaded host.
    if !single_threaded()? {
        return Err("native JIT cannot fork from a multithreaded host".into());
    }
    let argc = c_int::try_from(arguments.len() - 1)?;
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if pid == 0 {
        // The edit protocol owns the terminal's stdin in serve mode.
        unsafe {
            for pipe in compiler_pipes {
                libc::close(pipe);
            }
            if let Some(input) = input {
                if libc::dup2(input.as_raw_fd(), libc::STDIN_FILENO) < 0 {
                    libc::_exit(125);
                }
            }
            // The native host emitter supplies this exact entry signature.
            let main: extern "C" fn(c_int, *const *const c_char) -> c_int =
                std::mem::transmute(entry);
            libc::exit(main(argc, arguments.as_ptr()));
        }
    }
    let mut raw = 0;
    loop {
        if unsafe { libc::waitpid(pid, &mut raw, 0) } == pid {
            let status = ExitStatus::from_raw(raw);
            return Ok(status
                .code()
                .unwrap_or_else(|| 128 + status.signal().unwrap_or(0)));
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error.into());
        }
    }
}

fn run(
    stage: &Path,
    source: &str,
    runtime: Option<&str>,
    serve: bool,
    args: &[String],
) -> Result<i32> {
    let runtime = match runtime {
        Some(path) => Path::new(path).canonicalize()?,
        None => crate::driver::ensure_runtime_dylib()?,
    };
    let mut compiler = Compiler::start(stage, Path::new(source))?;
    let mut engine = NativeEngine::start(&runtime, &compiler.scratch.path().join("base.o"))?;
    let words: Vec<_> = std::iter::once(source)
        .chain(args.iter().map(String::as_str))
        .map(CString::new)
        .collect::<std::result::Result<_, _>>()?;
    let argv: Vec<_> = words
        .iter()
        .map(|word| word.as_ptr())
        .chain([std::ptr::null()])
        .collect();
    if !serve {
        let status = build_and_run(&mut compiler, &mut engine, &argv, None)?;
        compiler.finish()?;
        return Ok(status);
    }

    let input = File::open("/dev/null")?;
    let mut line = String::new();
    eprintln!("cinch jit: ready");
    while io::stdin().read_line(&mut line)? != 0 {
        if line != "\n" {
            return Err("native JIT expects one empty line per request".into());
        }
        let status = build_and_run(&mut compiler, &mut engine, &argv, Some(&input))?;
        eprintln!("cinch jit: status {status}");
        line.clear();
    }
    compiler.finish()?;
    Ok(0)
}

/// Entry point for `cinch jit --native-stage`.
pub fn cli(args: &[String]) -> i32 {
    let usage = || {
        eprintln!("usage: cinch jit --native-stage STAGE [--runtime LIBRARY] [--serve] SOURCE [-- ARGS...]");
        2
    };
    let Some((stage, mut words)) = args.split_first() else {
        return usage();
    };
    let mut runtime = None;
    let mut serve = false;
    let mut source = None;
    while let Some((word, rest)) = words.split_first() {
        words = rest;
        match word.as_str() {
            "--runtime" if runtime.is_none() => {
                let Some((path, rest)) = words.split_first() else {
                    return usage();
                };
                runtime = Some(path.as_str());
                words = rest;
            }
            "--serve" if !serve => serve = true,
            other if other.starts_with("--") => return usage(),
            _ => {
                source = Some(word.as_str());
                break;
            }
        }
    }
    let Some(source) = source else { return usage() };
    let trailing = match words.split_first() {
        None => &[][..],
        Some((dash, trailing)) if dash == "--" => trailing,
        _ => return usage(),
    };
    match run(Path::new(stage), source, runtime, serve, trailing) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("cinch jit: {error}");
            1
        }
    }
}
