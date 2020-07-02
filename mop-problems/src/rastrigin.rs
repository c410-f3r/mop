//! Rastrigin, L. A.; Systems of extremal control.

use crate::Problem;
use core::{
  f64::consts::PI,
  ops::{Range, RangeInclusive},
};
use mop::blocks::{Cstr, Obj, ObjDirection};

const N_USIZE: usize = 2;
const N_F64: f64 = 2.0;

type Solution = [f64; N_USIZE];
type Domain = [RangeInclusive<f64>; N_USIZE];

fn f1(s: &Solution) -> f64 {
  let sum = (0..N_USIZE).fold(0.0, |acc, idx| {
    let rslt = s[idx].powi(N_USIZE as i32) - 10.0 * (N_F64 * PI * s[idx]).cos();
    acc + rslt
  });
  10.0 * N_F64 + sum
}

#[derive(Debug)]
pub struct Rastrigin;

impl<'a>
  Problem<
    'a,
    Domain,
    [&'a (dyn Cstr<Solution> + Send + Sync); 0],
    [&'a (dyn Obj<f64, Solution> + Send + Sync); 1],
    Solution,
  > for Rastrigin
{
  const GRAPH_RANGES: [Range<f64>; 2] = [-6.0..6.0, -6.0..6.0];
  const NAME: &'static str = "Rastrigin";

  fn domain() -> Domain {
    [-5.12..=5.12, -5.12..=5.12]
  }

  fn hcs() -> [&'a (dyn Cstr<Solution> + Send + Sync); 0] {
    []
  }

  fn objs() -> [&'a (dyn Obj<f64, Solution> + Send + Sync); 1] {
    [&(ObjDirection::Min, f1 as fn(&Solution) -> f64)]
  }
}
