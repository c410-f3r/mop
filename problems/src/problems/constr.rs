//! Deb, Kalyanmoy (2002) Multiobjective optimization using evolutionary algorithms

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

const SOLUTION_DOMAIN: SolutionDomain = [0.1..=1.0, 0.0..=5.0];

type Constrain = fn(&Solution) -> usize;
type Objective = (ObjDirection, fn(&Solution) -> f64);
type Solution = [f64; 2];
type SolutionDomain = [RangeInclusive<f64>; 2];

fn f1(s: &Solution) -> f64 {
  s[0]
}

fn f2(s: &Solution) -> f64 {
  (1.0 + s[1]) / 1.0
}

fn g1(s: &Solution) -> usize {
  let lhs = s[1] + 9.0 * s[0];
  match lhs.partial_cmp(&6.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = -s[1] + 9.0 * s[0];
  match lhs.partial_cmp(&1.0) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

pub struct Constr;

impl Problem<Constrain, Objective, (), f64, Solution, SolutionDomain> for Constr {
  fn facade(
    &self,
    facade: OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain>,
    problem: &mut Mph<Constrain, Objective, f64, Solution, SolutionDomain>,
  ) -> OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain> {
    facade.initial_solutions(RandomInitialSolutions::default(), problem)
  }

  fn graph_ranges(&self) -> [Range<f64>; 2] {
    [0.3..1.0, 1.0..9.0]
  }

  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain, Objective, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("Constr")
        .push_hard_cstr(g1 as fn(&Solution) -> usize)
        .push_hard_cstr(g2)
        .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
        .push_obj((ObjDirection::Min, f2))
        .solution_domain(SOLUTION_DOMAIN)
        .build(),
      results_num,
    )
  }
}
