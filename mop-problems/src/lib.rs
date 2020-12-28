#![allow(
  // Function pointers
  clippy::as_conversions,
  trivial_casts
)]

pub mod binh_and_korn;
pub mod constr;
pub mod cvrp;
pub mod rastrigin;
pub mod schaffer_function_2;
pub mod test_function_4;

use core::ops::Range;
use mop::blocks::{Cstr, Obj};

pub trait Problem<D, S, const H: usize, const O: usize> {
  const GRAPH_RANGES: [Range<f64>; 2];
  const NAME: &'static str;

  fn domain() -> D;
  fn hcs<'a>() -> [&'a (dyn Cstr<S> + Send + Sync); H];
  fn objs<'a>() -> [&'a (dyn Obj<f64, S> + Send + Sync); O];
}
