//! Mop (Many Optimizations)

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

extern crate alloc;

mod criteria;
pub mod doc_tests;
pub mod dr_matrix;
pub mod mph;
mod obj_direction;
mod pct;
mod solution;
mod solution_domain;

pub use crate::{
  criteria::{cstr::*, obj::*},
  obj_direction::*,
  pct::*,
  solution::*,
  solution_domain::*,
};
