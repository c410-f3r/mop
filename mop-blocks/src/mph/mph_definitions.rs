use alloc::{string::String, vec::Vec};

/// Definitions for MPH
///
/// # Types
///
/// * `C`: Constraints Storage
/// * `O`: Objectives Storage
/// * `SD`: Solution Domain
#[derive(Clone, Debug)]
pub struct MphDefinitions<C, O, SD> {
  pub(crate) hard_cstrs: Vec<C>,
  pub(crate) name: String,
  pub(crate) objs: Vec<O>,
  pub(crate) solution_domain: SD,
}

impl<C, O, SD> MphDefinitions<C, O, SD> {
  /// Hard constraints
  pub fn hard_cstrs(&self) -> &[C] {
    &self.hard_cstrs
  }

  /// Name
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Objectives
  pub fn objs(&self) -> &[O] {
    &self.objs
  }

  pub fn solution_domain(&self) -> &SD {
    &self.solution_domain
  }

  pub fn solution_domain_mut(&mut self) -> &mut SD {
    &mut self.solution_domain
  }
}
