use crate::opt::{OptFacadeBuilder, OptHooks};
use alloc::{string::String, vec::Vec};
use core::{
  fmt::Debug,
  marker::PhantomData,
  ops::{Div, Sub},
};
use mop_blocks::{
  dr_matrix::DrMatrixVec,
  mph::{Mph, MphOrsEvaluators},
  Cstr, Obj, ObjDirection, Pct, SolutionDomain,
};
use mop_common_defs::{Solver, TraitCfg};
use num_traits::{NumCast, One, Zero};

#[derive(Debug)]
pub struct OptFacade<C, O, OH, OR, S, SD> {
  current_iteration: usize,
  last_fitness: OR,
  max_iterations: usize,
  objs_goals: Vec<OR>,
  opt_hooks: OH,
  stagnation_lower_bound: OR,
  stagnation_percentage: Pct,
  stagnation_threshold_counter: usize,
  stagnation_threshold: usize,
  phantom: PhantomData<(C, O, S, SD)>,
  stagnation_upper_bound: OR,
}

impl<C, O, OH, OR, S, SD> OptFacade<C, O, OH, OR, S, SD>
where
  OR: Copy + NumCast + One + Sub<OR, Output = OR> + Zero,
{
  pub fn new(
    max_iterations: usize,
    objs_goals: Vec<OR>,
    opt_hooks: OH,
    stagnation_percentage: Pct,
    stagnation_threshold: usize,
  ) -> Self {
    let sp = NumCast::from(*stagnation_percentage).unwrap();
    OptFacade {
      current_iteration: 0,
      last_fitness: OR::zero(),
      max_iterations,
      objs_goals,
      phantom: PhantomData,
      opt_hooks,
      stagnation_lower_bound: OR::one() - sp,
      stagnation_percentage,
      stagnation_threshold_counter: 0,
      stagnation_threshold,
      stagnation_upper_bound: OR::one() + sp,
    }
  }

  pub fn into_builder<I>(self) -> OptFacadeBuilder<OH, OR> {
    OptFacadeBuilder {
      max_iterations: Some(self.max_iterations),
      objs_goals: self.objs_goals,
      opt_hooks: Some(self.opt_hooks),
      stagnation_percentage: Some(self.stagnation_percentage),
      stagnation_threshold: Some(self.stagnation_threshold),
    }
  }
}

impl<C, O, OH, OR, S, SD> OptFacade<C, O, OH, OR, S, SD>
where
  C: Cstr<S>,
  O: Obj<OR, S>,
  OH: OptHooks<S>,
  OR: Copy
    + Div<OR, Output = OR>
    + NumCast
    + One
    + PartialOrd
    + Sub<OR, Output = OR>
    + Zero
    + TraitCfg,
  SD: SolutionDomain<S>,
{
  pub fn solve_problem_with<SOLVER>(
    mut self,
    problem: &mut Mph<C, O, OR, S, SD>,
    mut solver: SOLVER,
  ) -> Self
  where
    SOLVER: Solver<Problem = Mph<C, O, OR, S, SD>>,
  {
    solver.init(problem);
    self.opt_hooks.init();
    loop {
      {
        problem
          .results_mut()
          .iter_mut()
          .for_each(|mut x| self.opt_hooks.before_iter(x.solution_mut()));
      }
      solver.before_iter(problem);
      if self.iterations_number_has_extrapolated()
        || self.objs_are_not_converging(problem)
        || self.were_all_specified_goals_achieved(problem)
      {
        break;
      }
      solver.after_iter(problem);
      {
        problem
          .results_mut()
          .iter_mut()
          .for_each(|mut x| self.opt_hooks.after_iter(*x.solution_mut()));
      }
    }
    solver.finished(problem);
    self.opt_hooks.finished();
    self
  }

  fn objs_are_not_converging(&mut self, problem: &Mph<C, O, OR, S, SD>) -> bool {
    let fitness = **problem.results().best().unwrap().objs_avg();
    if fitness >= self.last_fitness * self.stagnation_lower_bound
      && fitness <= self.last_fitness * self.stagnation_upper_bound
    {
      self.stagnation_threshold_counter += 1;
    } else {
      self.stagnation_threshold_counter = 0;
      self.last_fitness = fitness;
    }
    self.stagnation_threshold_counter >= self.stagnation_threshold
  }

  fn iterations_number_has_extrapolated(&mut self) -> bool {
    self.current_iteration += 1;
    self.current_iteration >= self.max_iterations
  }

  fn were_all_specified_goals_achieved(&self, problem: &Mph<C, O, OR, S, SD>) -> bool
  where
    O: Obj<OR, S>,
  {
    let (defs, results) = problem.parts();
    let best = results.best().unwrap();
    let curr_objs = best.objs();
    let objs_defs = defs.objs();
    let mut num_of_achieved_objs = 0;
    for (obj_def, (curr_obj_val, specif_obj_goal)) in
      objs_defs.iter().zip(curr_objs.iter().zip(&self.objs_goals))
    {
      match obj_def.obj_direction() {
        ObjDirection::Max => {
          if curr_obj_val >= specif_obj_goal {
            num_of_achieved_objs += 1;
          }
        }
        ObjDirection::Min => {
          if curr_obj_val <= specif_obj_goal {
            num_of_achieved_objs += 1;
          }
        }
      }
    }
    num_of_achieved_objs == self.objs_goals.len() && !self.objs_goals.is_empty()
  }
}

impl<C, O, OH, OR, S, SD> OptFacade<C, O, OH, OR, S, SD>
where
  C: Cstr<S> + TraitCfg,
  O: Obj<OR, S> + TraitCfg,
  SD: SolutionDomain<S> + TraitCfg,
  OH: OptHooks<S>,
  OR: Copy + Debug + Div<OR, Output = OR> + NumCast + TraitCfg + Zero,
  S: Clone + TraitCfg,
{
  pub fn cstrs_reasons(&mut self, problem: &Mph<C, O, OR, S, SD>) -> DrMatrixVec<String> {
    let (defs, results) = problem.parts();
    MphOrsEvaluators::cstrs_reasons(defs, results)
  }
}
