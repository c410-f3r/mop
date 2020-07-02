use crate::{genetic_algorithm::operators::crossover::Crossover, utils::two_asc_rnd_num};
use cl_traits::{Clear, Push, Storage, Truncate};
use core::ops::Div;
use mop_blocks::{gp::MpOrs, Pct, Solution};
use rand::{rngs::StdRng, SeedableRng};

#[derive(Clone, Debug)]
pub struct MultiPoint {
  divisor: usize,
  probability: Pct,
}

impl MultiPoint {
  pub fn new(points: usize, probability: Pct) -> Self {
    MultiPoint { divisor: points.saturating_add(1), probability }
  }

  /// Swap data of individuals
  fn swap_data_of_indvs<S>(&self, first: &mut S, second: &mut S)
  where
    S: Solution,
  {
    let len = first.len();
    let chunk_len = len.div(self.divisor);
    for var_start in (0..self.divisor).map(|i| i * chunk_len).step_by(2) {
      for var_idx in var_start..var_start + chunk_len {
        first.inter_swap(second, var_idx);
      }
    }
  }
}

impl<OR, ORS, S, SS> Crossover<MpOrs<ORS, SS>> for MultiPoint
where
  OR: Copy,
  ORS:
    AsRef<[OR]> + AsMut<[OR]> + Clear + Extend<OR> + Storage<Item = OR> + Truncate<Input = usize>,
  S: Clone + Solution,
  SS:
    AsRef<[S]> + AsMut<[S]> + Clear + Push<Input = S> + Storage<Item = S> + Truncate<Input = usize>,
{
  type Error = mop_blocks::Error;

  fn crossover(
    &self,
    source: &mut MpOrs<ORS, SS>,
    destination: &mut MpOrs<ORS, SS>,
    filling_num: usize,
  ) -> Result<(), Self::Error> {
    destination.clear();
    let mut rng = StdRng::from_entropy();
    while destination.rslts_num() < filling_num {
      let [a, b] = two_asc_rnd_num(&mut rng, 0..source.rslts_num());
      let first = mop_blocks::Error::opt_rslt(source.get(a))?;
      let second = mop_blocks::Error::opt_rslt(source.get(b))?;
      destination.constructor().or_ref(first);
      destination.constructor().or_ref(second);
      if self.probability.is_in_rnd_pbty(&mut rng) {
        let a = destination.rslts_num() - 2;
        let b = destination.rslts_num() - 1;
        let [mut first, mut second] = mop_blocks::Error::opt_rslt(destination.get_two_mut(a, b))?;
        self.swap_data_of_indvs(*first.solution_mut(), *second.solution_mut());
      }
    }
    destination.truncate(filling_num);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::genetic_algorithm::operators::crossover::{Crossover, MultiPoint};
  use mop_blocks::{gp::MpOrRef, utils::dummy_mp, Pct};

  #[test]
  fn multi_point() {
    let mut problem = dummy_mp();
    let mut destination = problem.rslts_mut().clone();
    let source = problem.rslts_mut();
    source.constructor().or_os_iter([4.0, 8.0].iter().cloned(), [2.0, 3.0]);
    source.constructor().or_os_iter([2.0, 4.0].iter().cloned(), [1.0, 2.0]);
    let mp = MultiPoint::new(1, Pct::from_percent(100));
    mp.crossover(source, &mut destination, 2).unwrap();
    assert_eq!(destination.get(0).unwrap(), MpOrRef::new(&[], &[4.0, 8.0], &[], &[1.0, 3.0]));
    assert_eq!(destination.get(1).unwrap(), MpOrRef::new(&[], &[2.0, 4.0], &[], &[2.0, 2.0]));
  }
}
