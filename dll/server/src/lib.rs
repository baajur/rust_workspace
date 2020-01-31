#![cfg(windows)]

mod bridge;
pub mod config;
mod engine_functions;
mod hooks;
mod param;
mod utils;

use tnf_common::dll_main;

dll_main!({}, {});

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SERVER() {
    // FOnline needs this to check if this is correct dll for server
}
