//! Schaffer, J. David (1984). Some experiments in machine learning using vector evaluated genetic
//! algorithms (artificial intelligence, optimization, adaptation, pattern recognition)

mod common;

use common::Problem;
use core::ops::{Range, RangeInclusive};
use mop::ObjDirection;

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

  type Hcs = fn(&[f64; 1]) -> usize;
  type Objs = (ObjDirection, fn(&[f64; 1]) -> f64);

  #[inline]
  fn domain() -> Domain {
    [-5.0..=10.0]
  }

  #[inline]
  fn hcs() -> [Self::Hcs; 0] {
    []
  }

  #[inline]
  fn objs() -> [Self::Objs; 2] {
    [(ObjDirection::Min, f1), (ObjDirection::Min, f2)]
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  exec!("schaffer-function-2", SchafferFunction2);
  Ok(())
}
