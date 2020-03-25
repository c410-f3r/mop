use crate::{genetic_algorithm::operators::mutation::Mutation, utils::two_dist_rnd_num};
use mop_blocks::{mph::MphOrs, Pct, Solution};
use rand::{rngs::StdRng, SeedableRng};

#[derive(Clone, Debug)]
pub struct Swap {
  genes_num: usize,
  probability: Pct,
}

impl Swap {
  pub fn new(genes_num: usize, probability: Pct) -> Self {
    Swap { genes_num, probability }
  }
}
impl<M, OR, S> Mutation<M, MphOrs<OR, S>> for Swap
where
  S: Solution,
{
  fn mutation(&self, _: &M, source: &mut MphOrs<OR, S>) {
    let mut rng = StdRng::from_entropy();
    for mut individual in source.iter_mut() {
      let len = individual.solution_mut().len();
      if self.probability.is_in_rnd_pbty(&mut rng) || len < 2 {
        for _ in 0..self.genes_num {
          let [f_var_idx, s_var_idx] = two_dist_rnd_num(&mut rng, 0..len);
          individual.solution_mut().intra_swap(f_var_idx, s_var_idx);
        }
      }
    }
  }
}
