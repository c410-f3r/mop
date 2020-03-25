use crate::genetic_algorithm::operators::mutation::Mutation;
use mop_blocks::{mph::MphOrs, Pct, Solution, SolutionDomain};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Debug)]
pub struct RandomDomainAssignments {
  genes_num: usize,
  probability: Pct,
}

impl RandomDomainAssignments {
  pub fn new(genes_num: usize, probability: Pct) -> Self {
    RandomDomainAssignments { genes_num, probability }
  }
}
impl<OR, S, SD> Mutation<SD, MphOrs<OR, S>> for RandomDomainAssignments
where
  S: Solution,
  SD: SolutionDomain<S>,
{
  fn mutation(&self, sd: &SD, source: &mut MphOrs<OR, S>) {
    let mut rng = StdRng::from_entropy();
    for mut result in source.iter_mut() {
      if self.probability.is_in_rnd_pbty(&mut rng) {
        for _ in 0..self.genes_num {
          let var_idx = rng.gen_range(0, result.solution().len());
          sd.set_rnd_solution_domain(result.solution_mut(), var_idx, &mut rng);
        }
      }
    }
  }
}
