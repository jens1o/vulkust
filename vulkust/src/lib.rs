#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]
#![feature(duration_as_u128)]
#![feature(concat_idents)]

pub extern crate cgmath as math;
pub extern crate gltf;
pub extern crate image;
pub extern crate libc;
pub extern crate num_cpus;
pub extern crate rusttype;

#[cfg(apple_os)]
#[macro_use]
pub extern crate objc;

#[cfg(unix_based_os)]
#[macro_use]
pub extern crate bitflags;

#[cfg(target_os = "windows")]
pub extern crate winapi;

#[macro_use]
pub mod macros;
// pub mod audio;
pub mod core;
pub mod physics;
pub mod render;
pub mod system;
pub mod vulkan;
