//! Deb, Kalyanmoy (2002) Multiobjective optimization using evolutionary algorithms

mod common;

use common::Problem;
use core::{
  cmp::Ordering,
  ops::{Range, RangeInclusive},
};
use mop::ObjDirection;

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

impl Problem<Domain, Solution, 2, 2> for Constr {
  const GRAPH_RANGES: [Range<f64>; 2] = [0.0..2.0, 0.0..12.0];
  const NAME: &'static str = "Constr";

  type Hcs = fn(&[f64; 2]) -> usize;
  type Objs = (ObjDirection, fn(&[f64; 2]) -> f64);

  #[inline]
  fn domain() -> Domain {
    [0.1..=1.0, 0.0..=5.0]
  }

  #[inline]
  fn hcs() -> [Self::Hcs; 2] {
    [g1, g2]
  }

  #[inline]
  fn objs() -> [Self::Objs; 2] {
    [(ObjDirection::Min, f1), (ObjDirection::Min, f2)]
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  exec!("constr", Constr);
  Ok(())
}
