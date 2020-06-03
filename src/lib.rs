#![allow(non_camel_case_types)]
#![no_std]


#![allow(unused)]
mod generic;
pub use self::generic::*;

#[cfg(feature = "atsame70q21b")]
pub mod atsame70q21b;