use crate::{
  gp::MpOrs,
  solvers::{genetic_algorithm::operators::mutation::Mutation, utils::two_dist_rnd_num},
  Pct, Solution,
};
use cl_aux::SingleTypeStorage;
use rand::rngs::OsRng;

#[derive(Clone, Debug)]
pub struct Swap {
  times: usize,
  probability: Pct,
}

impl Swap {
  #[inline]
  pub fn new(times: usize, probability: Pct) -> Self {
    Swap { times, probability }
  }
}

impl<M, OR, ORS, S, SS> Mutation<M, MpOrs<ORS, SS>> for Swap
where
  ORS: AsMut<[OR]> + SingleTypeStorage<Item = OR>,
  SS: AsMut<[S]> + SingleTypeStorage<Item = S>,
  S: Solution,
{
  type Error = core::convert::Infallible;

  #[inline]
  fn mutation(&self, _: &M, source: &mut MpOrs<ORS, SS>) -> Result<(), Self::Error> {
    let mut rng = OsRng;
    for mut individual in source.iter_mut() {
      let len = individual.solution_mut().len();
      if self.probability.is_in_rnd_pbty(&mut rng) {
        for _ in 0..self.times {
          let [a, b] = two_dist_rnd_num(&mut rng, 0..len);
          individual.solution_mut().intra_swap(a, b);
        }
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    solvers::genetic_algorithm::operators::mutation::{Mutation, Swap},
    utils::dummy_mp,
    Pct,
  };

  #[test]
  fn swap() {
    let mut problem = dummy_mp();
    let (defs, source) = problem.parts_mut();
    let _ = source.constructor().or_os_iter([2.0, 4.0].iter().cloned(), [10.0, 20.0]);
    let rda = Swap::new(1, Pct::from_percent(100));
    rda.mutation(defs.domain(), source).unwrap();
    assert_eq!(*source.get(0).unwrap().solution().get(0).unwrap() as i32, 20);
    assert_eq!(*source.get(0).unwrap().solution().get(1).unwrap() as i32, 10);
  }
}
