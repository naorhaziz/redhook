extern crate libc;

#[cfg(target_env = "gnu")]
pub mod ld_preload;

#[cfg(target_env = "gnu")]
pub mod reentrancy_guard;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod dyld_insert_libraries;
