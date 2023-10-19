#[derive(Clone, Debug)]

pub struct GeneticAlgorithmParams<CO, M, MS> {
  pub crossover: CO,

  pub mating_selection: MS,

  pub mutation: M,
}
