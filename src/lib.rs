#![feature(asm)]
#![no_std]

pub use self::arch::*;
pub use self::call::*;
pub use self::data::*;
pub use self::error::*;
pub use self::flag::*;
pub use self::number::*;
pub use self::scheme::*;

#[cfg(target_arch = "arm")]
#[path="arch/arm.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path="arch/x86.rs"]
mod arch;

#[cfg(target_arch = "x86_64")]
#[path="arch/x86_64.rs"]
mod arch;

/// Function definitions
pub mod call;

/// Complex structures that are used for some system calls
pub mod data;

/// All errors that can be generated by a system call
pub mod error;

/// Flags used as an argument to many system calls
pub mod flag;

/// Call numbers used by each system call
pub mod number;

/// A trait useful for scheme handlers
pub mod scheme;
