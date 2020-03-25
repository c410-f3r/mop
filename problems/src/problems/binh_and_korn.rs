//! Binh  and  U.  Korn; MOBES:  A  multiobjective  evolution  strategy for constrained optimization problems

use crate::Problem;
use core::{
  cmp::Ordering,
  ops::{Range, RangeInclusive},
};
use mop::blocks::{
  mph::{Mph, MphDefinitionsBuilder},
  ObjDirection,
};

type Constrain = fn(&Solution) -> usize;
type Objective = (ObjDirection, fn(&Solution) -> f64);
type Solution = [f64; 2];
type SolutionDomain = [RangeInclusive<f64>; 2];

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
    _ => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = (s[0].powi(2) - 16.0 * s[0] + 64.0) + (s[1].powi(2) + 6.0 * s[1] + 9.0);
  match lhs.partial_cmp(&7.7) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

pub struct BinhAndKorn;

impl Problem<Constrain, Objective, f64, Solution, SolutionDomain> for BinhAndKorn {
  fn graph_ranges(&self) -> [Range<f64>; 2] {
    [0.0..140.0, 0.0..50.0]
  }

  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain, Objective, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("Binh and Korn")
        .push_hard_cstr(g1 as fn(&Solution) -> usize)
        .push_hard_cstr(g2)
        .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
        .push_obj((ObjDirection::Min, f2))
        .solution_domain([0.0..=5.0, 0.0..=3.0])
        .build(),
      results_num,
    )
  }
}
