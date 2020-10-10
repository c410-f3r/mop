//! Mop (Many Optimizations)

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_const_generics)]

extern crate alloc;

mod criteria;
pub mod doc_tests;
mod domain;
pub mod dr_matrix;
mod error;
pub mod gp;
mod obj_direction;
pub mod objs;
mod pct;
pub mod quality_comparator;
mod solution;
pub mod utils;

pub use crate::{
  criteria::{cstr::*, obj::*},
  domain::*,
  error::*,
  obj_direction::*,
  pct::*,
  solution::*,
};

pub type Result<T> = core::result::Result<T, error::Error>;