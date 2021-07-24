use crate::opt::OptHooks;
use alloc::vec::Vec;
use cl_traits::{Length, Storage};
use core::{
  fmt::Debug,
  iter::Sum,
  marker::PhantomData,
  ops::{Div, Sub},
};
use mop_blocks::{
  gp::{Gp, GpOrRef},
  quality_comparator::QualityComparator,
  Domain, Obj, ObjDirection, Pct, Solution,
};
use mop_common::Solver;
use num_traits::{NumCast, One, Zero};
#[cfg(feature = "std")]
use {core::time::Duration, std::time::Instant};

#[derive(Debug)]
pub struct OptFacade<D, HCRS, HCS, OH, OR, ORS, OS, QC, SCRS, SCS, SS> {
  best_idx_opt: Option<usize>,
  current_iteration: usize,
  max_iterations: usize,
  objs_goals: Vec<OR>,
  opt_hooks_opt: Option<OH>,
  phantom: PhantomData<(D, HCRS, HCS, ORS, OS, SCRS, SCS, SS)>,
  quality_comparator_opt: Option<QC>,
  stagnation_opt: Option<Stagnation<OR>>,
  #[cfg(feature = "std")]
  time_opt: Option<Time>,
}

impl<D, HCR, HCRS, HCS, O, OH, OR, ORS, OS, QC, S, SCR, SCRS, SCS, SS>
  OptFacade<D, HCRS, HCS, OH, OR, ORS, OS, QC, SCRS, SCS, SS>
where
  HCRS: AsMut<[HCR]> + AsRef<[HCR]> + Storage<Item = HCR>,
  D: Domain<S>,
  O: Obj<OR, S>,
  OH: OptHooks<Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>>,
  OR: Copy
    + Div<OR, Output = OR>
    + NumCast
    + One
    + PartialOrd
    + Sub<OR, Output = OR>
    + Zero
    + Sum<OR>,
  SCRS: AsMut<[SCR]> + AsRef<[SCR]> + Storage<Item = SCR>,
  ORS: AsMut<[OR]> + AsRef<[OR]> + Storage<Item = OR>,
  OS: AsMut<[O]> + AsRef<[O]> + Storage<Item = O>,
  QC: for<'a> QualityComparator<[O], GpOrRef<'a, HCR, OR, S, SCR>>,
  S: Solution,
  SS: AsMut<[S]> + AsRef<[S]> + Length + Storage<Item = S>,
{
  #[inline]
  pub fn new(max_iterations: usize) -> Self {
    OptFacade {
      best_idx_opt: None,
      current_iteration: 0,
      max_iterations,
      objs_goals: Vec::new(),
      opt_hooks_opt: None,
      phantom: PhantomData,
      quality_comparator_opt: None,
      stagnation_opt: None,
      #[cfg(feature = "std")]
      time_opt: None,
    }
  }

  #[inline]
  pub fn curr_best_idx(&self) -> Option<usize> {
    self.best_idx_opt
  }

  #[cfg(feature = "std")]
  #[inline]
  pub fn max_duration(&self) -> Option<Duration> {
    self.time_opt.as_ref().map(|t| t.max_duration)
  }

  #[inline]
  pub fn max_iterations(&self) -> usize {
    self.max_iterations
  }

  #[inline]
  pub fn objs_goals(&self) -> &[OR] {
    &self.objs_goals
  }

  #[inline]
  pub fn opt_hooks(&self) -> &Option<OH> {
    &self.opt_hooks_opt
  }

  #[inline]
  pub fn quality_comparator(&self) -> &Option<QC> {
    &self.quality_comparator_opt
  }

  #[inline]
  pub fn stagnation(&self) -> Option<(Pct, usize)> {
    self.stagnation_opt.as_ref().map(|s| (s.percentage, s.threshold))
  }

  #[cfg(feature = "std")]
  #[inline]
  pub fn set_max_duration(mut self, max_duration: Duration) -> Self {
    self.time_opt = Some(Time { current_duration: Instant::now(), max_duration });
    self
  }

  /// The maximum number of times the solver will process the solution
  #[inline]
  pub fn set_max_iterations(mut self, max_iterations: usize) -> Self {
    self.max_iterations = max_iterations;
    self
  }

  #[inline]
  pub fn set_objs_goals(mut self, objs_goals: Vec<OR>) -> Self {
    self.objs_goals = objs_goals;
    self
  }

  #[inline]
  pub fn set_opt_hooks(mut self, opt_hooks: OH) -> Self {
    self.opt_hooks_opt = Some(opt_hooks);
    self
  }

  /// The maximum number of times the solver will process the solution
  #[inline]
  pub fn set_quality_comparator(mut self, quality_comparator: QC) -> Self {
    self.quality_comparator_opt = Some(quality_comparator);
    self
  }

  /// Defines a stagnation stopping criteria.
  ///
  /// # Arguments
  ///
  /// * `percentage`: The variation (for more or less) which tells that the current solution
  /// is not converging.
  /// * `threshold`: Will stop processing if the solution is not converging for a certain
  /// number of times
  #[inline]
  pub fn set_stagnation(
    mut self,
    percentage: Pct,
    threshold: usize,
  ) -> Result<Self, mop_blocks::Error> {
    let casted_threshold = mop_blocks::Error::cast_rslt(*percentage)?;
    self.stagnation_opt = Some(Stagnation {
      last_fitness: OR::zero(),
      lower_bound: OR::one() - casted_threshold,
      percentage,
      threshold_counter: 0,
      threshold,
      upper_bound: OR::one() + casted_threshold,
    });
    Ok(self)
  }

  #[inline]
  pub async fn solve_problem_with<SOLVER>(
    mut self,
    problem: &mut Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>,
    mut solver: SOLVER,
  ) -> Result<Self, SOLVER::Error>
  where
    SOLVER: Solver<Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>>,
    SOLVER::Error: From<mop_blocks::Error>,
  {
    self.reset_aux_params();
    solver.init(problem);
    if let Some(oh) = self.opt_hooks_opt.as_mut() {
      oh.init();
    }
    loop {
      if let Some(oh) = self.opt_hooks_opt.as_mut() {
        oh.before_iter(problem);
      }
      solver.before_iter(problem).await?;
      // Ignore if best result couldn't be found
      let _ = self.manage_best_result(problem);
      let should_stop_partial = self.iterations_number_has_extrapolated()
        || mop_blocks::Error::opt_rslt(self.objs_are_not_converging(problem))?
        || mop_blocks::Error::opt_rslt(self.were_all_specified_goals_achieved(problem))?;
      #[cfg(feature = "std")]
      let should_stop = self.time_has_expired() || should_stop_partial;
      #[cfg(not(feature = "std"))]
      let should_stop = should_stop_partial;
      if should_stop {
        break;
      }
      if let Some(oh) = self.opt_hooks_opt.as_mut() {
        oh.after_iter(problem);
      }
      solver.after_iter(problem).await?;
    }
    solver.finished(problem);
    if let Some(oh) = self.opt_hooks_opt.as_mut() {
      oh.finished();
    }
    Ok(self)
  }

  fn iterations_number_has_extrapolated(&mut self) -> bool {
    self.current_iteration += 1;
    self.current_iteration >= self.max_iterations
  }

  fn manage_best_result(
    &mut self,
    problem: &Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>,
  ) -> Option<()> {
    let quality_comparator = self.quality_comparator_opt.as_ref()?;
    let mut best_idx = 0;
    let (defs, rslts) = problem.parts();
    for current_idx in 1..rslts.rslts_num() {
      let current_solution = rslts.get(current_idx)?;
      let best_solution = rslts.get(current_idx)?;
      if quality_comparator.is_better(defs.objs(), &current_solution, &best_solution) {
        best_idx = current_idx;
      }
    }
    self.best_idx_opt = Some(best_idx);
    Some(())
  }

  fn objs_are_not_converging(
    &mut self,
    problem: &mut Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>,
  ) -> Option<bool> {
    let (best_idx_opt, stagnation_opt) = (self.best_idx_opt, self.stagnation_opt.as_mut());
    if let Some(stagnation) = stagnation_opt {
      if let Some(best_idx) = best_idx_opt {
        let fitness = problem.rslts().get(best_idx)?.obj_rslts().iter().copied().sum::<OR>();
        if fitness >= stagnation.last_fitness * stagnation.lower_bound
          && fitness <= stagnation.last_fitness * stagnation.upper_bound
        {
          stagnation.threshold_counter += 1;
        } else {
          stagnation.threshold_counter = 0;
          stagnation.last_fitness = fitness;
        }
        Some(stagnation.threshold_counter >= stagnation.threshold)
      } else {
        Some(false)
      }
    } else {
      Some(false)
    }
  }

  fn reset_aux_params(&mut self) {
    self.current_iteration = 0;
    if let Some(stagnation) = self.stagnation_opt.as_mut() {
      stagnation.last_fitness = OR::zero();
      stagnation.threshold_counter = 0;
    }
    #[cfg(feature = "std")]
    {
      if let Some(time) = self.time_opt.as_mut() {
        time.current_duration = Instant::now();
      }
    }
  }

  #[cfg(feature = "std")]
  fn time_has_expired(&self) -> bool {
    if let Some(t) = self.time_opt.as_ref() {
      t.current_duration.elapsed() > t.max_duration
    } else {
      false
    }
  }

  fn were_all_specified_goals_achieved(
    &self,
    problem: &Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>,
  ) -> Option<bool>
  where
    O: Obj<OR, S>,
  {
    if let Some(best_idx) = self.best_idx_opt {
      let (defs, rslts) = problem.parts();
      let best = rslts.get(best_idx)?;
      let mut num_of_achieved_objs = 0;
      for (obj_def, (curr_obj_rslt, specif_obj_goal)) in
        defs.objs().iter().zip(best.obj_rslts().iter().zip(&self.objs_goals))
      {
        match obj_def.obj_direction() {
          ObjDirection::Max => {
            if curr_obj_rslt >= specif_obj_goal {
              num_of_achieved_objs += 1;
            }
          }
          ObjDirection::Min => {
            if curr_obj_rslt <= specif_obj_goal {
              num_of_achieved_objs += 1;
            }
          }
        }
      }
      Some(num_of_achieved_objs == self.objs_goals.len() && !self.objs_goals.is_empty())
    } else {
      Some(false)
    }
  }
}

#[derive(Debug)]
struct Stagnation<OR> {
  last_fitness: OR,
  lower_bound: OR,
  percentage: Pct,
  threshold_counter: usize,
  threshold: usize,
  upper_bound: OR,
}

#[derive(Debug)]
#[cfg(feature = "std")]
struct Time {
  current_duration: Instant,
  max_duration: Duration,
}
