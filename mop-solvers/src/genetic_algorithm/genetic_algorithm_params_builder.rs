use crate::genetic_algorithm::GeneticAlgorithmParams;

#[derive(Debug)]
pub struct GeneticAlgorithmParamsBuilder<C, M, MS> {
  crossover: Option<C>,
  mating_selection: Option<MS>,
  mutation: Option<M>,
}

impl<C, M, MS> GeneticAlgorithmParamsBuilder<C, M, MS> {
  pub fn build(self) -> Result<GeneticAlgorithmParams<C, M, MS>, mop_blocks::Error> {
    Ok(GeneticAlgorithmParams {
      crossover: mop_blocks::Error::opt_rslt(self.crossover)?,
      mating_selection: mop_blocks::Error::opt_rslt(self.mating_selection)?,
      mutation: mop_blocks::Error::opt_rslt(self.mutation)?,
    })
  }

  pub fn crossover(mut self, crossover: C) -> Self {
    self.crossover = Some(crossover);
    self
  }

  pub fn mating_selection(mut self, mating_selection: MS) -> Self {
    self.mating_selection = Some(mating_selection);
    self
  }

  pub fn mutation(mut self, mutation: M) -> Self {
    self.mutation = Some(mutation);
    self
  }
}

impl<C, M, MS> Default for GeneticAlgorithmParamsBuilder<C, M, MS> {
  fn default() -> Self {
    GeneticAlgorithmParamsBuilder { crossover: None, mating_selection: None, mutation: None }
  }
}
