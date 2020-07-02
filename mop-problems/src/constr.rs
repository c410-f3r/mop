//! Deb, Kalyanmoy (2002) Multiobjective optimization using evolutionary algorithms

use crate::Problem;
use core::{
  cmp::Ordering,
  ops::{Range, RangeInclusive},
};
use mop::blocks::{
  ObjDirection, {Cstr, Obj},
};

type Domain = [RangeInclusive<f64>; 2];
type Solution = [f64; 2];

fn f1(s: &Solution) -> f64 {
  s[0]
}

fn f2(s: &Solution) -> f64 {
  (1.0 + s[1]) / s[0]
}

fn g1(s: &Solution) -> usize {
  let lhs = s[1] + 9.0 * s[0];
  match lhs.partial_cmp(&6.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = -s[1] + 9.0 * s[0];
  match lhs.partial_cmp(&1.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

#[derive(Debug)]
pub struct Constr;

impl<'a>
  Problem<
    'a,
    Domain,
    [&'a (dyn Cstr<Solution> + Send + Sync); 2],
    [&'a (dyn Obj<f64, Solution> + Send + Sync); 2],
    Solution,
  > for Constr
{
  const GRAPH_RANGES: [Range<f64>; 2] = [0.0..2.0, 0.0..12.0];
  const NAME: &'static str = "Constr";

  fn domain() -> Domain {
    [0.1..=1.0, 0.0..=5.0]
  }

  fn hcs() -> [&'a (dyn Cstr<Solution> + Send + Sync); 2] {
    [&(g1 as fn(&Solution) -> usize), &(g2 as fn(&Solution) -> usize)]
  }

  fn objs() -> [&'a (dyn Obj<f64, Solution> + Send + Sync); 2] {
    [
      &(ObjDirection::Min, f1 as fn(&Solution) -> f64),
      &(ObjDirection::Min, f2 as fn(&Solution) -> f64),
    ]
  }
}
