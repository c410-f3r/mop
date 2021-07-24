use crate::genetic_algorithm::operators::mutation::Mutation;
use cl_traits::Storage;
use mop_blocks::{gp::MpOrs, Domain, Pct, Solution};
use rand::{rngs::OsRng, Rng};

#[derive(Clone, Debug)]
pub struct RandomDomainAssignments {
  times: usize,
  probability: Pct,
}

impl RandomDomainAssignments {
  #[inline]
  pub fn new(times: usize, probability: Pct) -> Self {
    RandomDomainAssignments { times, probability }
  }
}
impl<D, OR, ORS, S, SS> Mutation<D, MpOrs<ORS, SS>> for RandomDomainAssignments
where
  D: Domain<S>,
  S: Solution,

  ORS: AsMut<[OR]> + Storage<Item = OR>,
  SS: AsMut<[S]> + Storage<Item = S>,
{
  type Error = core::convert::Infallible;

  #[inline]
  fn mutation(&self, sd: &D, source: &mut MpOrs<ORS, SS>) -> Result<(), Self::Error> {
    let mut rng = OsRng;
    for mut result in source.iter_mut() {
      if self.probability.is_in_rnd_pbty(&mut rng) {
        for _ in 0..self.times {
          let var_idx = rng.gen_range(0..result.solution().len());
          sd.set_rnd_domain(result.solution_mut(), var_idx, &mut rng);
        }
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::genetic_algorithm::operators::mutation::{Mutation, RandomDomainAssignments};
  use mop_blocks::{utils::dummy_mp, Pct};

  #[test]
  fn random_domain_assignment() {
    let mut problem = dummy_mp();
    let (defs, source) = problem.parts_mut();
    let _ = source.constructor().or_os_iter([2.0, 4.0].iter().cloned(), [1.0, 2.0]);
    let rda = RandomDomainAssignments::new(2, Pct::from_percent(100));
    rda.mutation(defs.domain(), source).unwrap();
    let solution = *source.get(0).unwrap().solution();
    assert_ne!([*solution.get(0).unwrap() as i32, *solution.get(0).unwrap() as i32], [1, 2]);
  }
}
