//! Binh; A multiobjective evolutionary algorithm. The study cases

use crate::Problem;
use core::{
  cmp::Ordering,
  ops::{Range, RangeInclusive},
};
use mop::{
  blocks::{
    mph::{Mph, MphDefinitionsBuilder},
    ObjDirection,
  },
  facades::{initial_solutions::RandomInitialSolutions, opt::OptFacade},
};

const SOLUTION_DOMAIN: SolutionDomain = [-7.0..=4.0, -7.0..=4.0];

type Constrain = fn(&Solution) -> usize;
type Objective = (ObjDirection, fn(&Solution) -> f64);
type Solution = [f64; 2];
type SolutionDomain = [RangeInclusive<f64>; 2];

fn f1(s: &Solution) -> f64 {
  s[0] * s[0] - s[1]
}

fn f2(s: &Solution) -> f64 {
  -(0.5 * s[0]) - s[1] - 1.0
}

fn g1(s: &Solution) -> usize {
  let lhs = 6.5 - (s[0] / 6.0) - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = 7.5 - (0.5 * s[0]) - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

fn g3(s: &Solution) -> usize {
  let lhs = 30.0 - (5.0 * s[0]) - s[1];
  match lhs.partial_cmp(&0.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

pub struct TestFunction4;

impl Problem<Constrain, Objective, (), f64, Solution, SolutionDomain> for TestFunction4 {
  fn facade(
    &self,
    facade: OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain>,
    problem: &mut Mph<Constrain, Objective, f64, Solution, SolutionDomain>,
  ) -> OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain> {
    facade.initial_solutions(RandomInitialSolutions::default(), problem)
  }

  fn graph_ranges(&self) -> [Range<f64>; 2] {
    [-3.0..13.0, -8.0..-4.0]
  }
  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain, Objective, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("Test function 4")
        .push_hard_cstr(g1 as fn(&Solution) -> usize)
        .push_hard_cstr(g2)
        .push_hard_cstr(g3)
        .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
        .push_obj((ObjDirection::Min, f2))
        .solution_domain(SOLUTION_DOMAIN)
        .build(),
      results_num,
    )
  }
}
