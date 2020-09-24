#![no_std]

#[cfg(all(std_crate, not(std_future_trait)))]
extern crate std;

#[cfg(any(not(has_build_script), std_future_trait))]
pub use core::{future, task};

/// Asynchronous values.
#[cfg(all(has_build_script, not(std_future_trait)))]
pub mod future;

#[cfg(all(has_build_script, not(std_future_trait)))]
mod poll;
#[cfg(all(has_build_script, not(std_future_trait)))]
mod wake;

#[cfg(all(has_build_script, not(std_future_trait)))]
pub mod task {
    /// Types and Traits for working with asynchronous tasks.
    pub use crate::poll::*;
    pub use crate::wake::*;
}
