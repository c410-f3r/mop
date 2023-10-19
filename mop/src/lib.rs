//! Mop (Many Optimizations)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod criteria;
pub mod doc_tests;
mod domain;
pub mod dr_matrix;
mod either;
mod error;
#[cfg(all(not(feature = "rayon"), feature = "wasm-bindgen"))]
pub mod wasm_bindgen;

pub mod gp;
mod obj_direction;
pub mod objs;
pub mod opt;
mod par_bounds;
mod pct;
pub mod quality_comparator;
mod solution;
mod solver;
#[cfg(feature = "solvers")]
pub mod solvers;
pub mod utils;

pub use criteria::{cstr::*, obj::*};
pub use domain::*;
pub use either::Either;
pub use error::*;
pub use obj_direction::*;
pub use par_bounds::ParBounds;
pub use pct::*;
pub use solution::*;
pub use solver::Solver;

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(feature = "rayon")]
#[derive(Debug)]
pub struct ParallelIteratorWrapper<I>(pub(crate) I);

#[cfg(feature = "rayon")]
/// Parallel producer for Rayon implementation. This is mostly an internal detail.
#[derive(Debug)]
pub struct ParallelProducerWrapper<I>(pub(crate) I);
