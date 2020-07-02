#![allow(clippy::as_conversions, trivial_casts)]

pub mod binh_and_korn;
pub mod constr;
pub mod cvrp;
pub mod rastrigin;
pub mod schaffer_function_2;
pub mod test_function_4;

use cl_traits::Array;
use core::ops::Range;
use mop::blocks::{Cstr, Obj};

pub trait Problem<'a, D, H, O, S>
where
  H: Array<Item = &'a (dyn Cstr<S> + Send + Sync)>,
  O: Array<Item = &'a (dyn Obj<f64, S> + Send + Sync)>,
  S: 'a,
{
  const GRAPH_RANGES: [Range<f64>; 2];
  const NAME: &'static str;

  fn domain() -> D;
  fn hcs() -> H;
  fn objs() -> O;
}
