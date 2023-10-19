use crate::solvers::genetic_algorithm::GeneticAlgorithmParams;

#[derive(Debug)]
pub struct GeneticAlgorithmParamsBuilder<C, M, MS> {
  crossover: Option<C>,
  mating_selection: Option<MS>,
  mutation: Option<M>,
}

impl<C, M, MS> GeneticAlgorithmParamsBuilder<C, M, MS> {
  #[inline]
  pub fn build(self) -> Result<GeneticAlgorithmParams<C, M, MS>, crate::Error> {
    Ok(GeneticAlgorithmParams {
      crossover: crate::Error::opt_rslt(self.crossover)?,
      mating_selection: crate::Error::opt_rslt(self.mating_selection)?,
      mutation: crate::Error::opt_rslt(self.mutation)?,
    })
  }

  #[inline]
  pub fn crossover(mut self, crossover: C) -> Self {
    self.crossover = Some(crossover);
    self
  }

  #[inline]
  pub fn mating_selection(mut self, mating_selection: MS) -> Self {
    self.mating_selection = Some(mating_selection);
    self
  }

  #[inline]
  pub fn mutation(mut self, mutation: M) -> Self {
    self.mutation = Some(mutation);
    self
  }
}

impl<C, M, MS> Default for GeneticAlgorithmParamsBuilder<C, M, MS> {
  #[inline]
  fn default() -> Self {
    GeneticAlgorithmParamsBuilder { crossover: None, mating_selection: None, mutation: None }
  }
}
