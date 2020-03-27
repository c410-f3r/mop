use crate::opt::OptFacade;
use alloc::vec::Vec;
use core::{fmt::Debug, ops::Sub};
use mop_blocks::Pct;
use num_traits::{NumCast, One, Zero};

#[derive(Debug)]
pub struct OptFacadeBuilder<OH, OR> {
  pub(crate) max_iterations: Option<usize>,
  pub(crate) objs_goals: Vec<OR>,
  pub(crate) opt_hooks: Option<OH>,
  pub(crate) stagnation_percentage: Option<Pct>,
  pub(crate) stagnation_threshold: Option<usize>,
}

impl<OH, OR> OptFacadeBuilder<OH, OR>
where
  OR: Copy + NumCast + One + Sub<OR, Output = OR> + Zero,
{
  pub fn build<C, O, S, SD>(self) -> OptFacade<C, O, OH, OR, S, SD> {
    OptFacade::new(
      self.max_iterations.unwrap(),
      self.objs_goals,
      self.opt_hooks.unwrap(),
      self.stagnation_percentage.unwrap(),
      self.stagnation_threshold.unwrap(),
    )
  }

  /// The maximum number of times the solver will process the solution
  pub fn max_iterations(mut self, max_iterations: usize) -> Self {
    self.max_iterations = Some(max_iterations);
    self
  }

  pub fn objs_goals(mut self, objs_goals: Vec<OR>) -> Self {
    self.objs_goals = objs_goals;
    self
  }

  pub fn opt_hooks(mut self, opt_hooks: OH) -> Self {
    self.opt_hooks = Some(opt_hooks);
    self
  }

  /// The percentage variation (for more or less) which tells that the current solution is not converging
  pub fn stagnation_percentage(mut self, stagnation_percentage: Pct) -> Self {
    self.stagnation_percentage = Some(stagnation_percentage);
    self
  }

  /// Will stop processing if the solution is not converging for N consecutive times
  pub fn stagnation_threshold(mut self, stagnation_threshold: usize) -> Self {
    self.stagnation_threshold = Some(stagnation_threshold);
    self
  }
}

impl<OH, OR> Default for OptFacadeBuilder<OH, OR> {
  fn default() -> Self {
    Self {
      max_iterations: None,
      objs_goals: Vec::new(),
      opt_hooks: None,
      stagnation_percentage: None,
      stagnation_threshold: None,
    }
  }
}
