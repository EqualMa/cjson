#![no_std]
#![deny(clippy::missing_safety_doc)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod ser;
pub mod values;

mod utils;
pub mod r#const;
mod macros;

#[doc(hidden)]
pub mod __private;
