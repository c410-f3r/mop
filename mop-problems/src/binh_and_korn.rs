//! Binh  and  U.  Korn; MOBES:  A  multiobjective  evolution  strategy for constrained optimization problems

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
  4.0 * s[0].powi(2) + 4.0 * s[1].powi(2)
}

fn f2(s: &Solution) -> f64 {
  (s[0].powi(2) - 10.0 * s[0] + 25.0) + (s[1].powi(2) - 10.0 * s[1] + 25.0)
}

fn g1(s: &Solution) -> usize {
  let lhs = (s[0].powi(2) - 10.0 * s[0] + 25.0) + s[1].powi(2);
  match lhs.partial_cmp(&25.0) {
    Some(Ordering::Equal) | Some(Ordering::Less) => 0,
    None | Some(_) => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = (s[0].powi(2) - 16.0 * s[0] + 64.0) + (s[1].powi(2) + 6.0 * s[1] + 9.0);
  match lhs.partial_cmp(&7.7) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

#[derive(Debug)]
pub struct BinhAndKorn;

impl Problem<Domain, Solution, 2, 2> for BinhAndKorn {
  const GRAPH_RANGES: [Range<f64>; 2] = [0.0..140.0, 0.0..50.0];
  const NAME: &'static str = "Binh and Korn";

  fn domain() -> Domain {
    [0.0..=5.0, 0.0..=3.0]
  }

  fn hcs<'a>() -> [&'a (dyn Cstr<Solution> + Send + Sync); 2] {
    [&(g1 as fn(&Solution) -> usize), &(g2 as fn(&Solution) -> usize)]
  }

  fn objs<'a>() -> [&'a (dyn Obj<f64, Solution> + Send + Sync); 2] {
    [
      &(ObjDirection::Min, f1 as fn(&Solution) -> f64),
      &(ObjDirection::Min, f2 as fn(&Solution) -> f64),
    ]
  }
}
