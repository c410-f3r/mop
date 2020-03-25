use crate::{
  genetic_algorithm::operators::mating_selection::MatingSelection,
  quality_comparator::QualityComparator,
};
use mop_blocks::{
  mph::{MphOrRef, MphOrs},
  Obj,
};
use num_traits::Zero;
use rand::{distributions::uniform::SampleUniform, rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Debug, Default)]
pub struct Tournament<QC> {
  n: usize,
  quality_comparator: QC,
}

impl<QC> Tournament<QC> {
  pub fn new(n: usize, quality_comparator: QC) -> Self {
    assert!(n > 0);
    Tournament { n, quality_comparator }
  }
}
impl<QC, O, OR, S> MatingSelection<[O], MphOrs<OR, S>> for Tournament<QC>
where
  QC: for<'a> QualityComparator<[O], MphOrRef<'a, OR, S>>,
  O: Obj<OR, S>,
  OR: Copy + PartialOrd + SampleUniform + Zero,
  S: Clone,
{
  fn mating_selection(
    &self,
    objs: &[O],
    source: &mut MphOrs<OR, S>,
    destination: &mut MphOrs<OR, S>,
    filling_num: usize,
  ) {
    destination.clear();
    let mut rng = StdRng::from_entropy();
    while destination.len() < filling_num {
      let mut winner = source.get(rng.gen_range(0, source.len()));
      for _ in 1..self.n {
        let current = source.get(rng.gen_range(0, source.len()));
        if self.quality_comparator.is_better(objs, &current, &winner) {
          winner = current;
        }
      }
      destination
        .constructor()
        .copy_result(&winner)
        .commit(**winner.objs_avg(), (*winner.solution()).clone());
    }
  }
}
