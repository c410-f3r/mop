use crate::{
  gp::{MpOrRef, MpOrs},
  quality_comparator::QualityComparator,
  solvers::genetic_algorithm::operators::mating_selection::MatingSelection,
};
use cl_aux::{Clear, Push, SingleTypeStorage, Truncate};
use rand::{rngs::OsRng, Rng};

#[derive(Clone, Debug, Default)]
pub struct Tournament<QC> {
  n: usize,
  quality_comparator: QC,
}

impl<QC> Tournament<QC> {
  #[inline]
  pub fn new(n: usize, quality_comparator: QC) -> Self {
    assert!(n > 0);
    Tournament { n, quality_comparator }
  }
}
impl<QC, O, OR, ORS, S, SS> MatingSelection<[O], MpOrs<ORS, SS>> for Tournament<QC>
where
  OR: Copy,
  ORS: AsRef<[OR]> + Clear + Extend<OR> + SingleTypeStorage<Item = OR> + Truncate<Input = usize>,
  QC: for<'a> QualityComparator<[O], MpOrRef<'a, OR, S>>,
  S: Clone,
  SS: AsRef<[S]> + Clear + Push<S> + SingleTypeStorage<Item = S> + Truncate<Input = usize>,
{
  type Error = crate::Error;

  #[inline]
  fn mating_selection(
    &self,
    objs: &[O],
    source: &mut MpOrs<ORS, SS>,
    destination: &mut MpOrs<ORS, SS>,
    filling_num: usize,
  ) -> Result<(), Self::Error> {
    destination.clear();
    let mut rng = OsRng;
    while destination.rslts_num() < filling_num {
      let winner_opt = source.get(rng.gen_range(0..source.rslts_num()));
      let mut winner = crate::Error::opt_rslt(winner_opt)?;
      for _ in 0..self.n {
        let current_opt = source.get(rng.gen_range(0..source.rslts_num()));
        let current = crate::Error::opt_rslt(current_opt)?;
        if self.quality_comparator.is_better(objs, &current, &winner) {
          winner = current;
        }
      }
      let _ = destination.constructor().or_ref(&winner);
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    quality_comparator::ObjsAvg,
    solvers::genetic_algorithm::operators::mating_selection::{MatingSelection, Tournament},
    utils::dummy_mp,
  };

  #[test]
  fn tournament() {
    let mut problem = dummy_mp();
    let (defs, source) = problem.parts_mut();
    let mut destination = source.clone();
    let _ = source.constructor().or_os_iter([4.0, 8.0].iter().cloned(), [2.0, 3.0]);
    let _ = source.constructor().or_os_iter([2.0, 4.0].iter().cloned(), [1.0, 2.0]);
    let mp = Tournament::new(999, ObjsAvg);
    mp.mating_selection(defs.objs(), source, &mut destination, 1).unwrap();
    assert_eq!(destination.get(0), source.get(1));
  }
}
