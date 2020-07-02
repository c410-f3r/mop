#![cfg_attr(not(feature = "std"), no_std)]
#![doc(test(attr(forbid(
  unused_variables,
  unused_assignments,
  unused_mut,
  unused_attributes,
  dead_code
))))]

extern crate alloc;

pub mod opt;
