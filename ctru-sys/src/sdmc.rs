/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use ::types::*;
use services::fs::FS_DirectoryEntry;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdmc_dir_t {
    pub magic: u32_,
    pub fd: Handle,
    pub index: ssize_t,
    pub size: size_t,
    pub entry_data: [FS_DirectoryEntry; 32usize],
}
impl ::core::default::Default for sdmc_dir_t {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
extern "C" {
    pub fn sdmcInit() -> Result;
    pub fn sdmcWriteSafe(enable: u8);
    pub fn sdmcExit() -> Result;
    pub fn sdmc_getmtime(name: *const ::libc::c_char, mtime: *mut u64_)
     -> Result;
}
