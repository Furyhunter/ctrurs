#![feature(alloc)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(alloc_system)]
#![feature(allow_internal_unstable)]
#![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
#![feature(collections)]
#![feature(collections_range)]
#![feature(core_float)]
#![feature(const_fn)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(char_escape_debug)]
#![feature(dropck_eyepatch)]
#![feature(dropck_parametricity)]
#![feature(float_extras)]
#![feature(fn_traits)]
#![feature(fnbox)]
#![feature(fused)]
#![feature(generic_param_attrs)]
#![feature(heap_api)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(needs_drop)]
#![feature(needs_panic_runtime)]
#![feature(oom)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(placement_new_protocol)]
#![feature(prelude_import)]
#![feature(raw)]
#![feature(rand)]
#![feature(shared)]
#![feature(sip_hash_13)]
#![feature(slice_concat_ext)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(str_internals)]
#![feature(thread_local)]
#![feature(try_from)]
#![feature(unboxed_closures)]
#![feature(unicode)]
#![feature(unique)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(zero_one)]
#![allow(non_camel_case_types, dead_code, unused_features)]
#![no_std]

#![needs_panic_runtime]

#![cfg_attr(not(stage0), default_lib_allocator)]
#![stable(feature = "rust1", since = "1.0.0")]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

#[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
                 debug_assert_ne, unreachable, unimplemented, write, writeln, try)]
extern crate core as __core;

#[macro_use]
#[macro_reexport(vec, format)]
extern crate alloc;
extern crate std_unicode;
extern crate alloc_system;
extern crate libc;

// compiler-rt intrinsics
extern crate compiler_builtins;

// 3ds-specific dependencies
extern crate ctru_sys as libctru;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and reexports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use std_unicode::char;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod fs;
pub mod collections;
pub mod error;
pub mod ffi;
pub mod io;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod sync;
pub mod time;
pub mod heap;

// Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

// Private support modules
mod panicking;
mod memchr;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;
