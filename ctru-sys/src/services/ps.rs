/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PS_AESAlgorithm {
    PS_ALGORITHM_CBC_ENC = 0,
    PS_ALGORITHM_CBC_DEC = 1,
    PS_ALGORITHM_CTR_ENC = 2,
    PS_ALGORITHM_CTR_DEC = 3,
    PS_ALGORITHM_CCM_ENC = 4,
    PS_ALGORITHM_CCM_DEC = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PS_AESKeyType {
    PS_KEYSLOT_0D = 0,
    PS_KEYSLOT_2D = 1,
    PS_KEYSLOT_31 = 2,
    PS_KEYSLOT_38 = 3,
    PS_KEYSLOT_32 = 4,
    PS_KEYSLOT_39_DLP = 5,
    PS_KEYSLOT_2E = 6,
    PS_KEYSLOT_INVALID = 7,
    PS_KEYSLOT_36 = 8,
    PS_KEYSLOT_39_NFC = 9,
}
extern "C" {
    pub fn psInit() -> Result;
    pub fn psExit();
    pub fn PS_EncryptDecryptAes(size: u32_, in_: *mut u8_, out: *mut u8_,
                                aes_algo: PS_AESAlgorithm,
                                key_type: PS_AESKeyType, iv: *mut u8_)
     -> Result;
    pub fn PS_EncryptSignDecryptVerifyAesCcm(in_: *mut u8_, in_size: u32_,
                                             out: *mut u8_, out_size: u32_,
                                             data_len: u32_,
                                             mac_data_len: u32_,
                                             mac_len: u32_,
                                             aes_algo: PS_AESAlgorithm,
                                             key_type: PS_AESKeyType,
                                             nonce: *mut u8_) -> Result;
    pub fn PS_GetLocalFriendCodeSeed(seed: *mut u64_) -> Result;
    pub fn PS_GetDeviceId(device_id: *mut u32_) -> Result;
    pub fn PS_GenerateRandomBytes(out: *mut ::libc::c_void, len: size_t)
     -> Result;
}
use ::types::*;
