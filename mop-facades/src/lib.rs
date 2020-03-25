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

pub mod initial_solutions;
pub mod objs;
pub mod opt;
