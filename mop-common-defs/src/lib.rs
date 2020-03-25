#![cfg_attr(not(feature = "std"), no_std)]
#![deny(rust_2018_idioms)]
#![doc(test(attr(forbid(
  unused_variables,
  unused_assignments,
  unused_mut,
  unused_attributes,
  dead_code
))))]
#![forbid(missing_debug_implementations)]

mod solver;
mod trait_cfg;
pub mod utils;

pub use crate::{solver::Solver, trait_cfg::TraitCfg};
