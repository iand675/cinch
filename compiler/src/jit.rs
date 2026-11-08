//! Embedded LLVM ORC JIT (optional `jit` feature).
//!
//! The backend consumes LLVM IR text produced by [`crate::codegen`]. LLVM's
//! C API provides LLJIT, so this module does not need a C++ shim or an
//! `llvm-sys` dependency. The LLVM shared library is linked only when the
//! feature is enabled; a build without it keeps the ordinary clang path.

use crate::interface::{parse_json, JsonValue};
use crate::{Error, Result};
use std::collections::BTreeMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::Once;

type LLVMErrorRef = *mut c_void;
type LLVMContextRef = *mut c_void;
type LLVMModuleRef = *mut c_void;
type LLVMMemoryBufferRef = *mut c_void;
type LLVMOrcLLJITRef = *mut c_void;
type LLVMOrcThreadSafeContextRef = *mut c_void;
type LLVMOrcThreadSafeModuleRef = *mut c_void;
type LLVMOrcExecutionSessionRef = *mut c_void;
type LLVMOrcJITDylibRef = *mut c_void;
type LLVMOrcDefinitionGeneratorRef = *mut c_void;

#[allow(dead_code)]
extern "C" {
    fn LLVMInitializeAArch64TargetInfo();
    fn LLVMInitializeAArch64Target();
    fn LLVMInitializeAArch64TargetMC();
    fn LLVMInitializeAArch64AsmPrinter();
    fn LLVMInitializeAArch64AsmParser();
    fn LLVMInitializeX86TargetInfo();
    fn LLVMInitializeX86Target();
    fn LLVMInitializeX86TargetMC();
    fn LLVMInitializeX86AsmPrinter();
    fn LLVMInitializeX86AsmParser();

    fn LLVMContextCreate() -> LLVMContextRef;
    fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
        data: *const c_char,
        len: usize,
        name: *const c_char,
    ) -> LLVMMemoryBufferRef;
    fn LLVMCreateMemoryBufferWithContentsOfFile(
        path: *const c_char,
        out: *mut LLVMMemoryBufferRef,
        message: *mut *mut c_char,
    ) -> c_int;
    fn LLVMParseIRInContext(
        context: LLVMContextRef,
        buffer: LLVMMemoryBufferRef,
        module: *mut LLVMModuleRef,
        message: *mut *mut c_char,
    ) -> c_int;
    fn LLVMDisposeMessage(message: *mut c_char);

    fn LLVMOrcCreateLLJIT(result: *mut LLVMOrcLLJITRef, builder: *mut c_void) -> LLVMErrorRef;
    fn LLVMOrcDisposeLLJIT(jit: LLVMOrcLLJITRef) -> LLVMErrorRef;
    fn LLVMOrcLLJITGetExecutionSession(jit: LLVMOrcLLJITRef) -> LLVMOrcExecutionSessionRef;
    fn LLVMOrcLLJITGetMainJITDylib(jit: LLVMOrcLLJITRef) -> LLVMOrcJITDylibRef;
    fn LLVMOrcLLJITGetGlobalPrefix(jit: LLVMOrcLLJITRef) -> c_char;
    fn LLVMOrcCreateNewThreadSafeContextFromLLVMContext(
        context: LLVMContextRef,
    ) -> LLVMOrcThreadSafeContextRef;
    fn LLVMOrcDisposeThreadSafeContext(context: LLVMOrcThreadSafeContextRef);
    fn LLVMOrcCreateNewThreadSafeModule(
        module: LLVMModuleRef,
        context: LLVMOrcThreadSafeContextRef,
    ) -> LLVMOrcThreadSafeModuleRef;
    fn LLVMOrcLLJITAddLLVMIRModule(
        jit: LLVMOrcLLJITRef,
        dylib: LLVMOrcJITDylibRef,
        module: LLVMOrcThreadSafeModuleRef,
    ) -> LLVMErrorRef;
    fn LLVMOrcLLJITAddObjectFile(
        jit: LLVMOrcLLJITRef,
        dylib: LLVMOrcJITDylibRef,
        buffer: LLVMMemoryBufferRef,
    ) -> LLVMErrorRef;
    fn LLVMOrcLLJITLookup(
        jit: LLVMOrcLLJITRef,
        result: *mut u64,
        name: *const c_char,
    ) -> LLVMErrorRef;
    fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
        result: *mut LLVMOrcDefinitionGeneratorRef,
        global_prefix: c_char,
        filter: *mut c_void,
        filter_ctx: *mut c_void,
    ) -> LLVMErrorRef;
    fn LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(
        result: *mut LLVMOrcDefinitionGeneratorRef,
        file_name: *const c_char,
        global_prefix: c_char,
        filter: *mut c_void,
        filter_ctx: *mut c_void,
    ) -> LLVMErrorRef;
    fn LLVMOrcJITDylibAddGenerator(
        dylib: LLVMOrcJITDylibRef,
        generator: LLVMOrcDefinitionGeneratorRef,
    );

    fn LLVMGetErrorMessage(error: LLVMErrorRef) -> *mut c_char;
    fn LLVMDisposeErrorMessage(message: *mut c_char);
}

fn initialize_llvm() {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        #[cfg(target_arch = "aarch64")]
        {
            LLVMInitializeAArch64TargetInfo();
            LLVMInitializeAArch64Target();
            LLVMInitializeAArch64TargetMC();
            LLVMInitializeAArch64AsmPrinter();
            LLVMInitializeAArch64AsmParser();
        }
        #[cfg(target_arch = "x86_64")]
        {
            LLVMInitializeX86TargetInfo();
            LLVMInitializeX86Target();
            LLVMInitializeX86TargetMC();
            LLVMInitializeX86AsmPrinter();
            LLVMInitializeX86AsmParser();
        }
    });
}

unsafe fn take_error(error: LLVMErrorRef) -> Option<String> {
    if error.is_null() {
        return None;
    }
    let message = LLVMGetErrorMessage(error);
    let text = if message.is_null() {
        "LLVM ORC error".to_string()
    } else {
        let text = CStr::from_ptr(message).to_string_lossy().into_owned();
        LLVMDisposeErrorMessage(message);
        text
    };
    Some(text)
}

/// One embedded LLJIT instance. Code added to it stays alive until the
/// engine is dropped.
pub struct JitEngine {
    jit: LLVMOrcLLJITRef,
    context: LLVMOrcThreadSafeContextRef,
    llvm_context: LLVMContextRef,
    /// Whether a package object's recorded layout prefix is registered in
    /// this engine. The runtime accepts one combined table per process.
    package_layouts: bool,
}

impl JitEngine {
    /// Create an engine for the host. `runtime` is a shared runtime library
    /// whose exported `cinch_*` symbols should resolve in JIT'd code.
    pub fn new(runtime: Option<&Path>) -> Result<JitEngine> {
        initialize_llvm();
        unsafe {
            let mut jit = ptr::null_mut();
            let error = LLVMOrcCreateLLJIT(&mut jit, ptr::null_mut());
            if let Some(message) = take_error(error) {
                return Err(Error::plain(format!("create ORC LLJIT: {message}")));
            }
            let dylib = LLVMOrcLLJITGetMainJITDylib(jit);
            let prefix = LLVMOrcLLJITGetGlobalPrefix(jit);

            let mut process_generator = ptr::null_mut();
            let error = LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
                &mut process_generator,
                prefix,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            if let Some(message) = take_error(error) {
                let _ = LLVMOrcDisposeLLJIT(jit);
                return Err(Error::plain(format!("ORC process symbols: {message}")));
            }
            LLVMOrcJITDylibAddGenerator(dylib, process_generator);

            if let Some(path) = runtime {
                let path = CString::new(path.to_string_lossy().as_bytes())
                    .map_err(|_| Error::plain("runtime path contains a NUL byte"))?;
                let mut library_generator = ptr::null_mut();
                let error = LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(
                    &mut library_generator,
                    path.as_ptr(),
                    prefix,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if let Some(message) = take_error(error) {
                    let _ = LLVMOrcDisposeLLJIT(jit);
                    return Err(Error::plain(format!("ORC runtime symbols: {message}")));
                }
                LLVMOrcJITDylibAddGenerator(dylib, library_generator);
            }

            let llvm_context = LLVMContextCreate();
            let context = LLVMOrcCreateNewThreadSafeContextFromLLVMContext(llvm_context);
            Ok(JitEngine {
                jit,
                context,
                llvm_context,
                package_layouts: false,
            })
        }
    }

    /// Add one textual LLVM module to the engine.
    pub fn add_ir(&mut self, ir: &str, name: &str) -> Result<()> {
        unsafe {
            let module_name =
                CString::new(name).map_err(|_| Error::plain("module name contains a NUL byte"))?;
            let buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                ir.as_ptr().cast(),
                ir.len(),
                module_name.as_ptr(),
            );
            if buffer.is_null() {
                return Err(Error::plain("LLVM could not allocate the IR buffer"));
            }
            let mut module = ptr::null_mut();
            let mut message = ptr::null_mut();
            // The ThreadSafeContext owns the LLVMContext; its pointer remains
            // valid for parsing and for the ThreadSafeModule.
            let failed = LLVMParseIRInContext(self.llvm_context, buffer, &mut module, &mut message);
            if failed != 0 {
                let text = if message.is_null() {
                    "LLVM IR parse failed".to_string()
                } else {
                    let text = CStr::from_ptr(message).to_string_lossy().into_owned();
                    LLVMDisposeMessage(message);
                    text
                };
                return Err(Error::plain(format!("parse LLVM IR: {text}")));
            }
            let module = LLVMOrcCreateNewThreadSafeModule(module, self.context);
            let dylib = LLVMOrcLLJITGetMainJITDylib(self.jit);
            let error = LLVMOrcLLJITAddLLVMIRModule(self.jit, dylib, module);
            if let Some(message) = take_error(error) {
                return Err(Error::plain(format!("add JIT module: {message}")));
            }
            Ok(())
        }
    }

    /// Add one native object file to the engine's main JIT dylib.
    ///
    /// The bytes move into LLVM; this wrapper owns the memory buffer on
    /// every path and reports LLVM's own diagnostic when the file cannot be
    /// read or the object cannot be loaded (a foreign target, a malformed
    /// object, or a symbol that collides with one already in the dylib).
    pub fn add_object_file(&mut self, path: &Path) -> Result<()> {
        let c_path = CString::new(path.to_string_lossy().as_bytes())
            .map_err(|_| Error::plain(format!("{}: path contains a NUL byte", path.display())))?;
        unsafe {
            let mut buffer = ptr::null_mut();
            let mut message = ptr::null_mut();
            let failed = LLVMCreateMemoryBufferWithContentsOfFile(
                c_path.as_ptr(),
                &mut buffer,
                &mut message,
            );
            if failed != 0 {
                let text = if message.is_null() {
                    "cannot read the object file".to_string()
                } else {
                    let text = CStr::from_ptr(message).to_string_lossy().into_owned();
                    LLVMDisposeMessage(message);
                    text
                };
                return Err(Error::plain(format!("{}: {text}", path.display())));
            }
            // The buffer's ownership transfers to LLVM even when the add
            // fails, so neither branch disposes it here.
            let dylib = LLVMOrcLLJITGetMainJITDylib(self.jit);
            let error = LLVMOrcLLJITAddObjectFile(self.jit, dylib, buffer);
            if let Some(message) = take_error(error) {
                return Err(Error::plain(format!(
                    "{}: add object file: {message}",
                    path.display()
                )));
            }
            Ok(())
        }
    }

    /// Add one installed package object (`package.o` beside its
    /// `package.abi.json`) and register the layout prefix that manifest
    /// records.
    ///
    /// The manifest is validated against this host's native profile and the
    /// object bytes before anything enters the engine, and the recorded
    /// layout words are registered with the runtime as the process's one
    /// combined table (with `cinch_reclaim_enable` when the object was built
    /// with reclamation), exactly as a native consumer must before calling
    /// library code. The returned inventory names the units the object
    /// defines, with their physical arities.
    pub fn add_package_object(&mut self, object: &Path) -> Result<PackageObject> {
        if self.package_layouts {
            return Err(Error::plain(
                "this engine already registered a package layout prefix; one process registers one combined table",
            ));
        }
        let abi = read_package_abi(object)?;
        verify_package_interfaces(object, &abi.interfaces)?;
        let registration = layout_registration_ir(&abi.layout_words, abi.reclaim);
        self.add_ir(&registration, "package-layouts")?;
        self.add_object_file(object)?;
        self.call_void("cinch_jit_register_package_layouts")?;
        self.package_layouts = true;
        Ok(PackageObject {
            object: object.to_path_buf(),
            units: abi.units,
            layout_words: abi.layout_words.len(),
        })
    }

    /// Add a package object whose layout prefix is already covered by the
    /// package object added later. Dependency objects are linked before the
    /// root so their external symbols resolve; only the root registers the
    /// combined layout prefix.
    pub fn add_package_object_dependency(&mut self, object: &Path) -> Result<PackageObject> {
        let abi = read_package_abi(object)?;
        verify_package_interfaces(object, &abi.interfaces)?;
        self.add_object_file(object)?;
        Ok(PackageObject {
            object: object.to_path_buf(),
            units: abi.units,
            layout_words: abi.layout_words.len(),
        })
    }

    /// Resolve a symbol in the main JIT dylib.
    pub fn lookup(&self, name: &str) -> Result<u64> {
        let name =
            CString::new(name).map_err(|_| Error::plain("symbol name contains a NUL byte"))?;
        unsafe {
            let mut address = 0u64;
            let error = LLVMOrcLLJITLookup(self.jit, &mut address, name.as_ptr());
            if let Some(message) = take_error(error) {
                return Err(Error::plain(format!("JIT lookup {name:?}: {message}")));
            }
            Ok(address)
        }
    }

    /// Call a JIT'd `() -> i64` function.
    pub fn call_i64(&self, name: &str) -> Result<i64> {
        let address = self.lookup(name)?;
        let function: extern "C" fn() -> i64 = unsafe { std::mem::transmute(address) };
        Ok(function())
    }

    /// Call a JIT'd `() -> ()` function.
    pub fn call_void(&self, name: &str) -> Result<()> {
        let address = self.lookup(name)?;
        let function: extern "C" fn() = unsafe { std::mem::transmute(address) };
        function();
        Ok(())
    }

    /// Call a JIT'd C `main`. The generated Cinch entry uses the ordinary
    /// C signature and captures its command line through `cinch_argv_init`.
    pub fn call_main(&self) -> Result<i32> {
        let address = self.lookup("main")?;
        let function: extern "C" fn(c_int, *const *const c_char) -> c_int =
            unsafe { std::mem::transmute(address) };
        Ok(function(0, ptr::null()))
    }
}

impl Drop for JitEngine {
    fn drop(&mut self) {
        unsafe {
            if !self.jit.is_null() {
                let _ = LLVMOrcDisposeLLJIT(self.jit);
            }
            if !self.context.is_null() {
                LLVMOrcDisposeThreadSafeContext(self.context);
            }
        }
    }
}

/// One installed package object added to a [`JitEngine`]: the units its
/// physical manifest defines, with their arities, and the size of the
/// layout prefix registered before them.
pub struct PackageObject {
    pub object: PathBuf,
    pub units: BTreeMap<String, usize>,
    pub layout_words: usize,
}

/// The parts of an installed object's physical manifest the JIT consumes.
struct PackageAbi {
    units: BTreeMap<String, usize>,
    interfaces: BTreeMap<String, String>,
    layout_words: Vec<u64>,
    reclaim: bool,
}

/// The ABI manifest `emit-object` writes beside an object (`package.o` has
/// `package.abi.json`).
fn package_abi_path(object: &Path) -> PathBuf {
    object.with_extension("abi.json")
}

fn manifest_error(path: &Path, detail: impl std::fmt::Display) -> Error {
    Error::plain(format!("{}: {detail}", path.display()))
}

/// Read and validate an installed package object's ABI manifest.
///
/// The checks reject an object this host cannot execute: a different target
/// triple, runtime ABI, word size, endianness, or ownership mode. The
/// recorded object content hash must also match the bytes on disk, so a
/// stale manifest never describes newer code.
fn read_package_abi(object: &Path) -> Result<PackageAbi> {
    let manifest_path = package_abi_path(object);
    let text =
        std::fs::read_to_string(&manifest_path).map_err(|e| manifest_error(&manifest_path, e))?;
    let parsed = parse_json(&text).map_err(|e| manifest_error(&manifest_path, e))?;
    let fields = parsed
        .as_object()
        .map_err(|e| manifest_error(&manifest_path, e))?;
    let number = |key: &str| -> Result<u64> {
        let value = fields
            .get(key)
            .ok_or_else(|| manifest_error(&manifest_path, format!("missing {key}")))?;
        value
            .as_num()
            .map_err(|e| manifest_error(&manifest_path, format!("{key}: {e}")))
    };
    let string = |key: &str| -> Result<&str> {
        let value = fields
            .get(key)
            .ok_or_else(|| manifest_error(&manifest_path, format!("missing {key}")))?;
        value
            .as_str()
            .map_err(|e| manifest_error(&manifest_path, format!("{key}: {e}")))
    };
    let profile = crate::core::TargetProfile::native();
    if number("schema")? != 1 {
        return Err(manifest_error(
            &manifest_path,
            "unsupported object manifest schema",
        ));
    }
    if number("runtime_abi")? != u64::from(crate::interface::RUNTIME_ABI_VERSION) {
        return Err(manifest_error(
            &manifest_path,
            format!(
                "runtime ABI {} does not match this compiler's {}",
                number("runtime_abi")?,
                crate::interface::RUNTIME_ABI_VERSION
            ),
        ));
    }
    if string("target")? != profile.triple {
        return Err(manifest_error(
            &manifest_path,
            format!(
                "target {} is not this host's {}",
                string("target")?,
                profile.triple
            ),
        ));
    }
    if number("word_size")? != u64::from(profile.word_size) {
        return Err(manifest_error(
            &manifest_path,
            format!(
                "word size {} is not this host's {}",
                number("word_size")?,
                profile.word_size
            ),
        ));
    }
    if string("endian")? != profile.endian {
        return Err(manifest_error(
            &manifest_path,
            format!(
                "endianness {} is not this host's {}",
                string("endian")?,
                profile.endian
            ),
        ));
    }
    let reclaim = fields
        .get("reclaim")
        .ok_or_else(|| manifest_error(&manifest_path, "missing reclaim"))?
        .as_bool()
        .map_err(|e| manifest_error(&manifest_path, format!("reclaim: {e}")))?;
    if reclaim != profile.reclaim {
        return Err(manifest_error(
            &manifest_path,
            format!(
                "object reclamation mode ({reclaim}) does not match this profile's ({})",
                profile.reclaim
            ),
        ));
    }
    let bytes = std::fs::read(object).map_err(|e| manifest_error(object, e))?;
    let recorded: u64 = string("object_content_hash")?
        .parse()
        .map_err(|_| manifest_error(&manifest_path, "object_content_hash is not a number"))?;
    let actual: u64 = crate::interface::content_hash_bytes(&bytes)
        .parse()
        .expect("content hash is decimal");
    if recorded != actual {
        return Err(manifest_error(
            &manifest_path,
            "recorded object content hash does not match the object bytes",
        ));
    }
    let units = fields
        .get("units")
        .ok_or_else(|| manifest_error(&manifest_path, "missing units"))?
        .as_array()
        .map_err(|e| manifest_error(&manifest_path, format!("units: {e}")))?;
    let mut inventory = BTreeMap::new();
    for unit in units {
        let unit = unit
            .as_object()
            .map_err(|e| manifest_error(&manifest_path, format!("units: {e}")))?;
        let symbol = unit
            .get("symbol")
            .ok_or_else(|| manifest_error(&manifest_path, "unit without a symbol"))?
            .as_str()
            .map_err(|e| manifest_error(&manifest_path, format!("unit symbol: {e}")))?;
        let arity = unit
            .get("arity")
            .ok_or_else(|| manifest_error(&manifest_path, "unit without an arity"))?
            .as_num()
            .map_err(|e| manifest_error(&manifest_path, format!("unit arity: {e}")))?;
        let arity = usize::try_from(arity).map_err(|_| {
            manifest_error(&manifest_path, format!("unit {symbol}: arity is too large"))
        })?;
        inventory.insert(symbol.to_string(), arity);
    }
    let layout_words = fields
        .get("layout_words")
        .ok_or_else(|| manifest_error(&manifest_path, "missing layout_words"))?
        .as_array()
        .map_err(|e| manifest_error(&manifest_path, format!("layout_words: {e}")))?
        .iter()
        .map(|word| {
            word.as_num()
                .map_err(|e| manifest_error(&manifest_path, format!("layout word: {e}")))
        })
        .collect::<Result<Vec<u64>>>()?;
    let mut interfaces = BTreeMap::new();
    if let Some(rows) = fields.get("interfaces") {
        for row in rows
            .as_array()
            .map_err(|e| manifest_error(&manifest_path, format!("interfaces: {e}")))?
        {
            let row = row
                .as_object()
                .map_err(|e| manifest_error(&manifest_path, format!("interface: {e}")))?;
            let module = row
                .get("module")
                .ok_or_else(|| manifest_error(&manifest_path, "interface without a module"))?
                .as_str()
                .map_err(|e| manifest_error(&manifest_path, format!("interface module: {e}")))?;
            let hash = row
                .get("iface_hash")
                .ok_or_else(|| manifest_error(&manifest_path, "interface without iface_hash"))?
                .as_str()
                .map_err(|e| manifest_error(&manifest_path, format!("interface hash: {e}")))?;
            interfaces.insert(module.to_string(), hash.to_string());
        }
    }
    Ok(PackageAbi {
        units: inventory,
        interfaces,
        layout_words,
        reclaim,
    })
}

/// The installed `.cinchif` files beside the object are the checked contracts
/// the object was built from. Check the ones the install materialized; the
/// embedded prelude interfaces are part of the object's physical manifest but
/// are not copied into a package's `build/` directory.
fn verify_package_interfaces(object: &Path, expected: &BTreeMap<String, String>) -> Result<()> {
    let Some(build) = object.parent().map(|parent| parent.join("build")) else {
        return Ok(());
    };
    for (module, hash) in expected {
        let path = build.join(format!("{module}.cinchif"));
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let iface = crate::interface::Iface::from_json(&text)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        if iface.module != *module {
            return Err(Error::plain(format!(
                "{} names module {}, expected {module}",
                path.display(),
                iface.module
            )));
        }
        if iface.iface_hash() != *hash {
            return Err(Error::plain(format!(
                "{}: interface hash does not match the object manifest",
                path.display()
            )));
        }
    }
    Ok(())
}

/// The consumer's `__apply`: the closure dispatch `codegen::APPLY_DEF`
/// emits into every ordinary executable. A package object deliberately
/// leaves this symbol to its consumer, so a JIT session that loads one
/// defines the same stub here before the object enters the dylib.
const APPLY_DEF: &str = "\
define i64 @__apply(i64 %f, i64 %argc, ptr %argv) {
entry:
  %self = inttoptr i64 %f to ptr
  %cp = getelementptr inbounds i8, ptr %self, i64 0
  %cw = load i64, ptr %cp
  %code = inttoptr i64 %cw to ptr
  %r = call i64 %code(ptr %self, i64 %argc, ptr %argv)
  ret i64 %r
}

";

/// The module a JIT session must provide beside a package object: the
/// consumer's `__apply`, and the object's recorded layout prefix registered
/// with the runtime. The prefix is immutable, so registering exactly it is
/// the valid combined table for a process that adds no consumer layouts.
fn layout_registration_ir(words: &[u64], reclaim: bool) -> String {
    let table = if words.is_empty() {
        "[0 x i64] zeroinitializer".to_string()
    } else {
        format!(
            "[{} x i64] [{}]",
            words.len(),
            words
                .iter()
                .map(|word| format!("i64 {}", *word as i64))
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let enable = if reclaim {
        "  call void @cinch_reclaim_enable()\n"
    } else {
        ""
    };
    format!(
        "{apply}@__cinch_package_layouts = internal constant {table}\n\
         declare void @cinch_layouts_register(ptr, i64)\n\
         declare void @cinch_reclaim_enable()\n\
         define void @cinch_jit_register_package_layouts() {{\n\
         entry:\n{enable}  call void @cinch_layouts_register(ptr @__cinch_package_layouts, i64 {count})\n  ret void\n}}\n",
        apply = APPLY_DEF,
        count = words.len()
    )
}

/// Run a textual LLVM module's `main` against a runtime dylib.
#[cfg(feature = "jit")]
pub fn run_main(ir: &str, runtime: &Path) -> Result<i32> {
    // Executable IR carries a private `__cinch_layouts_init`, and ORC does
    // not run `llvm.global_ctors` automatically. Expose a tiny external
    // installer and call it once, before any generated constructor or drop.
    let install = ir.contains("@__cinch_layouts_init");
    let ir = if install {
        format!(
            "{ir}\ndefine void @cinch_jit_install_layouts() {{\nentry:\n  call void @__cinch_layouts_init()\n  ret void\n}}\n"
        )
    } else {
        ir.to_string()
    };
    let mut engine = JitEngine::new(Some(runtime))?;
    engine.add_ir(&ir, "cinch-jit")?;
    if install {
        engine.call_void("cinch_jit_install_layouts")?;
    }
    engine.call_main()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn embedded_orc_runs_a_module() {
        let mut engine = JitEngine::new(None).expect("JIT engine");
        engine
            .add_ir(
                "define i64 @answer() {\nentry:\n  ret i64 42\n}\n",
                "answer",
            )
            .expect("add module");
        assert_eq!(engine.call_i64("answer").expect("call answer"), 42);
    }

    /// Standard base64, which is how a registry bundle carries its files.
    fn base64(bytes: &[u8]) -> String {
        const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [
                chunk[0],
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
            ];
            let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32);
            out.push(A[(n >> 18) as usize & 63] as char);
            out.push(A[(n >> 12) as usize & 63] as char);
            out.push(if chunk.len() > 1 {
                A[(n >> 6) as usize & 63] as char
            } else {
                '='
            });
            out.push(if chunk.len() > 2 {
                A[n as usize & 63] as char
            } else {
                '='
            });
        }
        out
    }

    /// The package identity `install_bundle` rechecks: SHA-256 over the
    /// canonical manifest and the sorted file rows.
    fn package_identity(manifest: &serde_json::Value, source: &[u8]) -> String {
        package_identity_files(manifest, &[("Library.cinch", source)])
    }

    fn package_identity_files(manifest: &serde_json::Value, files: &[(&str, &[u8])]) -> String {
        let text = serde_json::to_string(manifest).expect("manifest JSON");
        let manifest = crate::interface::parse_json(&text)
            .expect("canonical manifest")
            .to_canonical_string();
        let mut rows = files
            .iter()
            .map(|(path, source)| {
                let mut file = Sha256::new();
                file.update(source);
                let hash: String = file
                    .finalize()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
                format!("{path}\t{hash}")
            })
            .collect::<Vec<_>>();
        rows.sort();
        let mut identity = Sha256::new();
        identity.update(format!("package\n{manifest}\n{}", rows.join("\n")).as_bytes());
        let digest: String = identity
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        format!("sha256:{digest}")
    }

    #[test]
    fn installed_package_object_loads_and_runs_a_unit() {
        // The install path drives the ordinary checker and code generator,
        // which recurse deeply on the embedded prelude.
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(|| {
                use serde_json::json;
                // The registry install path is what puts `package.o` and its
                // `package.abi.json` in a project, so build the object the
                // same way `cinch pkg add` does rather than fabricating one.
                let project = tempfile::tempdir().expect("project");
                // `boxed` allocates a constructor, so the call proves the
                // recorded layout prefix was registered, not merely read.
                let source = b"module Library where\n  data Box where\n    Box :: Int -> Box\n  value :: Int\n  value = 42\n  boxed :: Box\n  boxed = Box 42\n";
                let manifest = json!({
                    "schema": 2,
                    "name": "acme/library",
                    "version": "0.1.0",
                    "modules": [{"name": "Library", "path": "Library.cinch"}],
                    "order": ["Library"],
                });
                let package_id = package_identity(&manifest, source);
                let bundle = json!({
                    "schema": 1,
                    "manifest": manifest,
                    "files": {"Library.cinch": base64(source)},
                    "visibility": "public",
                });
                let installed = crate::pkg::install_bundle(
                    project.path(),
                    "http://registry.local",
                    &package_id,
                    &bundle,
                    None,
                    None,
                )
                .expect("install package bundle");
                assert!(installed.object.exists(), "install writes package.o");
                assert!(installed.abi.exists(), "install writes package.abi.json");

                let runtime = crate::driver::ensure_runtime_dylib().expect("runtime dylib");
                let mut engine = JitEngine::new(Some(&runtime)).expect("JIT engine");
                let package = engine
                    .add_package_object(&installed.object)
                    .expect("load installed package object");
                assert_eq!(package.units.get("Library.value"), Some(&0));
                assert!(package.layout_words > 0, "the object records a layout prefix");
                // `value :: Int` is the runtime word `42 << 1 | 1`; looking up
                // and calling the unit is the ABI's zero-arity form.
                assert_eq!(engine.call_i64("Library.value").expect("call unit"), 85);
                // A constructor block's word addresses its payload, whose
                // first word here is the same immediate. Allocation consults
                // the registered descriptor table, so this call fails if the
                // recorded prefix was not installed first.
                let boxed = engine.call_i64("Library.boxed").expect("call allocating unit") as u64;
                assert_eq!(unsafe { *(boxed as *const u64) }, 85);
            })
            .expect("test thread")
            .join()
            .expect("test thread result");
    }

    #[test]
    fn cross_object_references_resolve_in_one_jit_engine() {
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(|| {
                use serde_json::json;
                let project = tempfile::tempdir().expect("project");
                let base_source = b"module Base where\n  value :: Int\n  value = 7\n";
                let base_manifest = json!({
                    "schema": 2,
                    "name": "acme/base",
                    "version": "0.1.0",
                    "modules": [{"name": "Base", "path": "Base.cinch"}],
                    "order": ["Base"],
                });
                let base_id =
                    package_identity_files(&base_manifest, &[("Base.cinch", base_source)]);
                let base_bundle = json!({
                    "schema": 1,
                    "manifest": base_manifest,
                    "files": {"Base.cinch": base64(base_source)},
                    "visibility": "public",
                });
                let base_installed = crate::pkg::install_bundle(
                    project.path(),
                    "http://registry.local",
                    &base_id,
                    &base_bundle,
                    None,
                    None,
                )
                .expect("install base");
                let top_source =
                    b"module Top where\n  import Base\n  answer :: Int\n  answer = value + 1\n";
                let top_manifest = json!({
                    "schema": 2,
                    "name": "acme/top",
                    "version": "0.1.0",
                    "modules": [{"name": "Top", "path": "Top.cinch"}],
                    "order": ["Top"],
                });
                let top_id = package_identity_files(&top_manifest, &[("Top.cinch", top_source)]);
                let top_bundle = json!({
                    "schema": 1,
                    "manifest": top_manifest,
                    "files": {"Top.cinch": base64(top_source)},
                    "visibility": "public",
                });
                let top_installed = crate::pkg::install_bundle(
                    project.path(),
                    "http://registry.local",
                    &top_id,
                    &top_bundle,
                    None,
                    None,
                )
                .expect("install top");
                let runtime = crate::driver::ensure_runtime_dylib().expect("runtime dylib");
                let mut engine = JitEngine::new(Some(&runtime)).expect("JIT engine");
                let base_object = engine
                    .add_package_object_dependency(&base_installed.object)
                    .expect("load base object");
                assert!(base_object.units.contains_key("Base.value"));
                let top_object = engine
                    .add_package_object(&top_installed.object)
                    .expect("load top object");
                assert!(top_object.units.contains_key("Top.answer"));
                assert!(
                    top_object
                        .units
                        .keys()
                        .all(|symbol| symbol.starts_with("Top.") || symbol.starts_with("cinch.")),
                    "top object must not bundle Base: {:?}",
                    top_object.units
                );
                assert_eq!(engine.call_i64("Top.answer").expect("call Top.answer"), 17);
            })
            .expect("test thread")
            .join()
            .expect("test thread result");
    }
}
