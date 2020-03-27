//! Rastrigin, L. A.; Systems of extremal control.

use crate::Problem;
use core::{
  f64::consts::PI,
  ops::{Range, RangeInclusive},
};
use mop::{
  blocks::{
    mph::{Mph, MphDefinitionsBuilder},
    ObjDirection,
  },
  facades::{initial_solutions::RandomInitialSolutions, opt::OptFacade},
};

const N_USIZE: usize = 2;
const N_F64: f64 = 2.0;
const SOLUTION_DOMAIN: SolutionDomain = [-5.12..=5.12, -5.12..=5.12];

type Constrain = ();
type Objective = (ObjDirection, fn(&Solution) -> f64);
type Solution = [f64; N_USIZE];
type SolutionDomain = [RangeInclusive<f64>; N_USIZE];

fn f1(s: &Solution) -> f64 {
  let sum = (0..N_USIZE).fold(0.0, |acc, idx| {
    let rslt = s[idx].powi(N_USIZE as i32) - 10.0 * (N_F64 * PI * s[idx]).cos();
    acc + rslt
  });
  10.0 * N_F64 + sum
}

pub struct Rastrigin;

impl Problem<Constrain, Objective, (), f64, Solution, SolutionDomain> for Rastrigin {
  fn facade(
    &self,
    facade: OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain>,
    problem: &mut Mph<Constrain, Objective, f64, Solution, SolutionDomain>,
  ) -> OptFacade<Constrain, Objective, (), f64, Solution, SolutionDomain> {
    facade.initial_solutions(RandomInitialSolutions::default(), problem)
  }

  fn graph_ranges(&self) -> [Range<f64>; N_USIZE] {
    [-6.0..6.0, -6.0..6.0]
  }

  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain, Objective, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("Rastrigin")
        .push_hard_cstr(())
        .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
        .solution_domain(SOLUTION_DOMAIN)
        .build(),
      results_num,
    )
  }
}
