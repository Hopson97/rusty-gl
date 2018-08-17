//! # Rusty GL
//! `rusty_gl` is a wrapper over the C-binding for OpenGL for Rust, gl-rs
//! The aim of this crate is:
//!    1. Make OpenGL in Rust easier to use
//!
//!         This has been done by hiding weird (and unsafe!) casts from Rust code to the
//!         C interface, as well as using rust types over std::os::raw and std::ffi types. For example,
//!         some functions take in &str rather than std::ffi::CString.
//!     
//!    2. Make OpenGL in Rust safer to use
//!         
//!         One way I have done this is by enforcing correct enum types when passing to functions.
//!         The C-Interface allowed for any GLenum to pass into a function where it was needed, but
//!         it is very easy to pass the wrong one, causing `GL_INVALID_ENUM` errors.
//!         Instead, I have made specific enum types, and then enforce those types when calling rusty-gl
//!         functions
//!         
//!    3. Make OpenGL in Rust fit along nicely with other Rust code
//!
//!         The C interface had camelCase interface, as well as requiring `unsafe` blocks everywhere.
//!         Rusty GL functions use the more Rust accepted `snake_case` for functions and `PascalCase`
//!         for types. Also, none of the functions require `unsafe {}` blocks to be used.

extern crate gl;

pub mod buffers;
pub mod drawing;
pub mod enums;
pub mod shaders;
pub mod textures;
