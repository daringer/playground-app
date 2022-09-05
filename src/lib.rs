//! Playground and test client for Trussed
//!

#![cfg_attr(not(test), no_std)]
// #![warn(missing_docs)]

#[macro_use]
extern crate delog;
generate_macros!();

use trussed::{client, syscall, types::Message, Client as TrussedClient};

mod playground;
pub use playground::App;