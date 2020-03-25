//! Schaffer, J. David (1984). Some experiments in machine learning using vector evaluated genetic
//! algorithms (artificial intelligence, optimization, adaptation, pattern recognition)

use crate::Problem;
use core::ops::{Range, RangeInclusive};
use mop::blocks::{
  mph::{Mph, MphDefinitionsBuilder},
  ObjDirection,
};

type Constrain = ();
type Objective = (ObjDirection, fn(&Solution) -> f64);
type Solution = [f64; 1];
type SolutionDomain = [RangeInclusive<f64>; 1];

fn f1(s: &Solution) -> f64 {
  match s[0] {
    x if x <= 1.0 => -x,
    x if x > 1.0 && x <= 3.0 => x - 2.0,
    x if x > 3.0 && x <= 4.0 => 4.0 - x,
    _ => s[0] - 4.0,
  }
}

fn f2(s: &Solution) -> f64 {
  s[0].powi(2) - 10.0 * s[0] + 25.0
}

pub struct SchafferFunction2;

impl Problem<Constrain, Objective, f64, Solution, SolutionDomain> for SchafferFunction2 {
  fn graph_ranges(&self) -> [Range<f64>; 2] {
    [-1.0..1.0, 0.0..16.0]
  }

  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain, Objective, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("Schaffer function 2")
        .push_hard_cstr(())
        .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
        .push_obj((ObjDirection::Min, f2))
        .solution_domain([-5.0..=10.0])
        .build(),
      results_num,
    )
  }
}
