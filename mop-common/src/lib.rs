//! MOP - Commons

#![no_std]

extern crate alloc;

mod solver;
mod trait_cfg;

pub use crate::{solver::*, trait_cfg::*};
