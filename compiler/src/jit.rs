//! Embedded LLVM ORC JIT (optional `jit` feature).
//!
//! The backend consumes LLVM IR text produced by [`crate::codegen`]. LLVM's
//! C API provides LLJIT, so this module does not need a C++ shim or an
//! `llvm-sys` dependency. The LLVM shared library is linked only when the
//! feature is enabled; a build without it keeps the ordinary clang path.

use crate::{Error, Result};
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::Path;
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
    fn LLVMParseIRInContext(
        context: LLVMContextRef,
        buffer: LLVMMemoryBufferRef,
        module: *mut LLVMModuleRef,
        message: *mut *mut c_char,
    ) -> c_int;
    fn LLVMDisposeMessage(message: *mut c_char);

    fn LLVMOrcCreateLLJIT(
        result: *mut LLVMOrcLLJITRef,
        builder: *mut c_void,
    ) -> LLVMErrorRef;
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
            Ok(JitEngine { jit, context, llvm_context })
        }
    }

    /// Add one textual LLVM module to the engine.
    pub fn add_ir(&mut self, ir: &str, name: &str) -> Result<()> {
        unsafe {
            let module_name = CString::new(name).map_err(|_| Error::plain("module name contains a NUL byte"))?;
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

    /// Resolve a symbol in the main JIT dylib.
    pub fn lookup(&self, name: &str) -> Result<u64> {
        let name = CString::new(name).map_err(|_| Error::plain("symbol name contains a NUL byte"))?;
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

    #[test]
    fn embedded_orc_runs_a_module() {
        let mut engine = JitEngine::new(None).expect("JIT engine");
        engine
            .add_ir("define i64 @answer() {\nentry:\n  ret i64 42\n}\n", "answer")
            .expect("add module");
        assert_eq!(engine.call_i64("answer").expect("call answer"), 42);
    }
}
