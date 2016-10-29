/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
extern "C" {
    pub fn srvPmInit() -> Result;
    pub fn srvPmExit();
    pub fn SRVPM_PublishToProcess(notificationId: u32_, process: Handle)
     -> Result;
    pub fn SRVPM_PublishToAll(notificationId: u32_) -> Result;
    pub fn SRVPM_RegisterProcess(procid: u32_, count: u32_,
                                 serviceaccesscontrol: *mut ::libc::c_void)
     -> Result;
    pub fn SRVPM_UnregisterProcess(procid: u32_) -> Result;
}
use ::types::*;
