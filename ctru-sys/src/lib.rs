/*
 * C bindings generation: 
 * bindgen --match=file.h --use-core --ctypes-prefix=libc -- --sysroot=$DEVKITARM/arm-none-eabi -I$CTRULIB/include $CTRULIB/include/3ds.h
 *
 */

#![no_std]
#![allow(non_camel_case_types, non_snake_case, overflowing_literals)]
#![feature(untagged_unions)]

pub mod applets;
pub mod console;
pub mod env;
pub mod gfx;
pub mod gpu;
pub mod ipc;
pub mod ndsp;
pub mod os;
pub mod romfs;
pub mod sdmc;
pub mod services;
pub mod svc;
pub mod srv;
pub mod sys;
pub mod synchronization;
pub mod thread;
pub mod types;

pub use self::sys::*;
pub use self::types::*;
