//! Schaffer, J. David (1984). Some experiments in machine learning using vector evaluated genetic
//! algorithms (artificial intelligence, optimization, adaptation, pattern recognition)

use crate::Problem;
use core::ops::{Range, RangeInclusive};
use mop::blocks::{Cstr, Obj, ObjDirection};

type Solution = [f64; 1];
type Domain = [RangeInclusive<f64>; 1];

fn f1(s: &Solution) -> f64 {
  if s[0] <= 1.0 {
    -s[0]
  } else if s[0] > 1.0 && s[0] <= 3.0 {
    s[0] - 2.0
  } else if s[0] > 3.0 && s[0] <= 4.0 {
    4.0 - s[0]
  } else {
    s[0] - 4.0
  }
}

fn f2(s: &Solution) -> f64 {
  s[0].powi(2) - 10.0 * s[0] + 25.0
}

#[derive(Debug)]
pub struct SchafferFunction2;

impl Problem<Domain, Solution, 0, 2> for SchafferFunction2 {
  const GRAPH_RANGES: [Range<f64>; 2] = [-2.0..10.0, 0.0..30.0];
  const NAME: &'static str = "Schaffer function 2";

  fn domain() -> Domain {
    [-5.0..=10.0]
  }

  fn hcs<'a>() -> [&'a (dyn Cstr<Solution> + Send + Sync); 0] {
    []
  }

  fn objs<'a>() -> [&'a (dyn Obj<f64, Solution> + Send + Sync); 2] {
    [
      &(ObjDirection::Min, f1 as fn(&Solution) -> f64),
      &(ObjDirection::Min, f2 as fn(&Solution) -> f64),
    ]
  }
}
