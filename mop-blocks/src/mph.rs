//! MP (Multi-objective Problem)

mod mph_definitions;
mod mph_definitions_builder;
mod mph_or;
mod mph_or_constructor;
mod mph_ors;
mod mph_ors_evaluators;

pub use mph_definitions::*;
pub use mph_definitions_builder::*;
pub use mph_or::*;
pub use mph_or_constructor::*;
pub use mph_ors::*;
pub use mph_ors_evaluators::*;

/// MPH (Multi-objective Problem with Hard constraints)
///
/// # Types
///
/// * `C`: Constraint
/// * `O`: Objective
/// * `OR`: Objective Result
/// * `SD`: Solution Domain
#[derive(Clone, Debug)]
pub struct Mph<C, O, OR, S, SD> {
  pub(crate) definitions: MphDefinitions<C, O, SD>,
  pub(crate) ors: MphOrs<OR, S>,
}

impl<C, O, OR, S, SD> Mph<C, O, OR, S, SD> {
  pub fn new(definitions: MphDefinitions<C, O, SD>, ors: MphOrs<OR, S>) -> Self {
    Self { definitions, ors }
  }

  pub fn with_capacity(definitions: MphDefinitions<C, O, SD>, results_num: usize) -> Self {
    let ors = MphOrs::with_capacity(&definitions, results_num);
    Self { definitions, ors }
  }

  pub fn definitions(&self) -> &MphDefinitions<C, O, SD> {
    &self.definitions
  }

  pub fn into_parts(self) -> (MphDefinitions<C, O, SD>, MphOrs<OR, S>) {
    (self.definitions, self.ors)
  }

  pub fn parts(&self) -> (&MphDefinitions<C, O, SD>, &MphOrs<OR, S>) {
    (&self.definitions, &self.ors)
  }

  pub fn parts_mut(&mut self) -> (&mut MphDefinitions<C, O, SD>, &mut MphOrs<OR, S>) {
    (&mut self.definitions, &mut self.ors)
  }

  pub fn results(&self) -> &MphOrs<OR, S> {
    &self.ors
  }

  pub fn results_mut(&mut self) -> &mut MphOrs<OR, S> {
    &mut self.ors
  }
}
