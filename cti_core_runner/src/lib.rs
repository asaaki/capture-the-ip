// SAFETY: we control both ends
#![allow(unsafe_code)]

#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

pub use cti_types::RunMode;
pub use std::process::ExitCode;

extern "C" {
    // shared lib: cti_core
    fn run(mode: RunMode) -> u8;
}

pub fn core_run(mode: RunMode) -> ExitCode {
    let code = unsafe { run(mode) };
    ExitCode::from(code)
}

/*
Alternative approach with:
libloading = "0.7.4"

use libloading::{Library, Symbol, library_filename};

type CoreLibFn = unsafe extern "C" fn(RunMode) -> u8;

const CTI_CORE_LIB: &str = "cti_core";
const CTI_CORE_RUN: &[u8; 4] = b"run\0";

fn call_core_run(mode: RunMode) -> u8 {
    #[allow(unsafe_code)]
    // SAFETY: we control both ends
    unsafe {
        let lib = Library::new(library_filename(CTI_CORE_LIB)).expect("cti_core lib loading issue");
        let run_fn: Symbol<'_, CoreLibFn> = lib.get(CTI_CORE_RUN).expect("cti_core fn symbol issue");
        run_fn(mode)
    }
}
*/
