use crate::{genetic_algorithm::operators::crossover::Crossover, utils::two_asc_rnd_num};
use mop_blocks::{mph::MphOrs, Pct, Solution};
use rand::{rngs::StdRng, SeedableRng};

#[derive(Clone, Debug)]
pub struct MultiPoint {
  points: usize,
  probability: Pct,
}

impl MultiPoint {
  /// Swap data of individuals
  fn swap_data_of_indvs<S>(&self, first: &mut S, second: &mut S)
  where
    S: Solution,
  {
    let chunk_len = first.len() / self.points;
    let mut should_include = false;
    for (switch_counter, var_idx) in (0..first.len()).enumerate() {
      if switch_counter % chunk_len == 0 {
        should_include = !should_include
      }
      if should_include && second.has_var(var_idx) {
        first.inter_swap(second, var_idx);
      }
    }
  }
}

impl MultiPoint {
  pub fn new(points: usize, probability: Pct) -> Self {
    MultiPoint { points, probability }
  }
}
impl<OR, S> Crossover<MphOrs<OR, S>> for MultiPoint
where
  OR: Copy,
  S: Clone + Solution,
{
  fn crossover(
    &self,
    source: &mut MphOrs<OR, S>,
    destination: &mut MphOrs<OR, S>,
    filling_num: usize,
  ) {
    destination.clear();
    let mut rng = StdRng::from_entropy();
    while destination.len() < filling_num {
      {
        let [a, b] = two_asc_rnd_num(&mut rng, 0..source.len());
        let first = source.get(a);
        destination
          .constructor()
          .copy_result(&first)
          .commit(**first.objs_avg(), (*first.solution()).clone());
        let second = source.get(b);
        destination
          .constructor()
          .copy_result(&second)
          .commit(**second.objs_avg(), (*second.solution()).clone());
      }
      {
        if self.probability.is_in_rnd_pbty(&mut rng) {
          let a = destination.len() - 2;
          let b = destination.len() - 1;
          let [mut first, mut second] = destination.get_two_mut(a, b);
          self.swap_data_of_indvs(*first.solution_mut(), *second.solution_mut());
        }
      }
    }
    destination.truncate(filling_num);
  }
}
