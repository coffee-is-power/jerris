//! A JVM written in rust
//!
//! **WARNING:** Don't expect this to work first time, this crate doesn't have all the functionality of a JVM yet, so this is **NOT** production ready
//!
//! This crate implements the specifications at https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
#[macro_use]
extern crate num_derive;

pub mod big_endian;
pub mod class;
pub mod constant_pool;
pub mod access_flags;
pub mod field;
pub mod method;
pub mod attribute;