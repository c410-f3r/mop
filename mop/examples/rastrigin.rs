//! Rastrigin, L. A.; Systems of extremal control.

mod common;

use common::Problem;
use core::{
  f64::consts::PI,
  ops::{Range, RangeInclusive},
};
use mop::ObjDirection;

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

impl Problem<Domain, Solution, 0, 1> for Rastrigin {
  const GRAPH_RANGES: [Range<f64>; 2] = [-6.0..6.0, -6.0..6.0];
  const NAME: &'static str = "Rastrigin";

  type Hcs = fn(&[f64; 2]) -> usize;
  type Objs = (ObjDirection, fn(&[f64; 2]) -> f64);

  #[inline]
  fn domain() -> Domain {
    [-5.12..=5.12, -5.12..=5.12]
  }

  #[inline]
  fn hcs() -> [Self::Hcs; 0] {
    []
  }

  #[inline]
  fn objs() -> [Self::Objs; 1] {
    [(ObjDirection::Min, f1)]
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  exec!("rastrigin", Rastrigin);
  Ok(())
}
