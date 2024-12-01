// Avoid including the documentation examples if testing without the "string" feature
#![cfg_attr(feature = "string", doc = include_str!("./Lib.md"))]
//!

#![no_std]
// I'm not ideologically opposed to allowing non-safe code in this crate, but
// IMO it needs really excellent justification. One likely place where this
// could show up is if and when we support a non-std alloc mode. In that case,
// we need some way to synchronize access to a PikeVM cache. That in turn will
// likely require rolling our own primitive spin-lock or similar structure.
#![forbid(unsafe_code)]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
// When the main features are disabled, squash dead code warnings. The
// alternative is to litter conditional compilation directives everywhere,
// which is super annoying.
#![cfg_attr(not(feature = "string"), allow(dead_code))]

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("not supported on non-{32,64}, please file an issue");

extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "string")]
pub use self::string::*;
pub use self::{error::Error, hir::escape};

mod error;
mod hir;
mod int;
mod interpolate;
mod nfa;
mod pikevm;
mod pool;
#[cfg(feature = "string")]
mod string;
mod utf8;
