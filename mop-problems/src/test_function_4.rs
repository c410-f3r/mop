//! Binh; A multiobjective evolutionary algorithm. The study cases

use crate::Problem;
use core::{
  cmp::Ordering,
  ops::{Range, RangeInclusive},
};
use mop::blocks::{Cstr, Obj, ObjDirection};

type Solution = [f64; 2];
type Domain = [RangeInclusive<f64>; 2];

fn f1(s: &Solution) -> f64 {
  s[0].powi(2) - s[1]
}

fn f2(s: &Solution) -> f64 {
  -0.5 * s[0] - s[1] - 1.0
}

fn g1(s: &Solution) -> usize {
  let lhs = 6.5 - s[0] / 6.0 - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = 7.5 - 0.5 * s[0] - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

fn g3(s: &Solution) -> usize {
  let lhs = 30.0 - 5.0 * s[0] - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    None | Some(_) => 1,
  }
}

#[derive(Debug)]
pub struct TestFunction4;

impl Problem<Domain, Solution, 3, 2> for TestFunction4 {
  const GRAPH_RANGES: [Range<f64>; 2] = [-3.0..13.0, -8.0..-4.0];
  const NAME: &'static str = "Test Function 4";

  fn domain() -> Domain {
    [-7.0..=4.0, -7.0..=4.0]
  }

  fn hcs<'a>() -> [&'a (dyn Cstr<Solution> + Send + Sync); 3] {
    [
      &(g1 as fn(&Solution) -> usize),
      &(g2 as fn(&Solution) -> usize),
      &(g3 as fn(&Solution) -> usize),
    ]
  }

  fn objs<'a>() -> [&'a (dyn Obj<f64, Solution> + Send + Sync); 2] {
    [
      &(ObjDirection::Min, f1 as fn(&Solution) -> f64),
      &(ObjDirection::Min, f2 as fn(&Solution) -> f64),
    ]
  }
}
