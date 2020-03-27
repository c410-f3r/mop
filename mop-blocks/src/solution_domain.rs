#[cfg(feature = "with_rand")]
use {
  core::ops::RangeInclusive,
  rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
  },
};

pub trait SolutionDomain<S> {
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn len(&self) -> usize;

  #[cfg(feature = "with_rand")]
  fn new_random_solution<R>(&self, rng: &mut R) -> S
  where
    R: Rng;

  #[cfg(feature = "with_rand")]
  fn set_rnd_solution_domain<R>(&self, s: &mut S, idx: usize, rng: &mut R)
  where
    R: Rng;
}

#[cfg(feature = "with_rand")]
macro_rules! array_impls {
  ($($N:expr),+) => {
    $(
      impl<T> SolutionDomain<[T; $N]> for [RangeInclusive<T>; $N]
      where
        T: Copy + SampleUniform,
      {
        fn len(&self) -> usize {
          $N
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> [T; $N]
        where
          R: Rng,
        {
          cl_traits::create_array(|idx| {
            Uniform::from(*self[idx].start()..=*self[idx].end()).sample(rng)
          })
        }

        fn set_rnd_solution_domain<R>(&self, s: &mut [T; $N], idx: usize, rng: &mut R)
        where
          R: Rng,
        {
          let domain = &self[idx];
          let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
          s[idx] = domain_value;
        }
      }

      #[cfg(feature = "with_arrayvec")]
      impl<T> SolutionDomain<arrayvec::ArrayVec<[T; $N]>> for arrayvec::ArrayVec<[RangeInclusive<T>; $N]>
      where
        T: Copy + SampleUniform,
      {
        fn len(&self) -> usize {
          self.len()
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> arrayvec::ArrayVec<[T; $N]>
        where
          R: Rng,
        {
          let mut s = arrayvec::ArrayVec::new();
          for domain in self.iter() {
            s.push(Uniform::from(*domain.start()..=*domain.end()).sample(rng));
          }
          s
        }

        fn set_rnd_solution_domain<R>(&self, s: &mut arrayvec::ArrayVec<[T; $N]>, idx: usize, rng: &mut R)
        where
          R: Rng,
        {
          let domain = &self[idx];
          let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
          s[idx] = domain_value;
        }
      }

      #[cfg(feature = "with_ndsparse")]
      impl<DA, DATA, DS, IS, OS> SolutionDomain<ndsparse::csl::Csl<DA, DS, IS, OS>> for [RangeInclusive<DATA>; $N]
      where
        DA: cl_traits::Array<Item = usize> + Copy + Default,
        DATA: Copy + SampleUniform,
        DS: AsMut<[DATA]> + AsRef<[DATA]> + Default + cl_traits::Push<Input = DATA> + cl_traits::Storage<Item = DATA>,
        IS: AsMut<[usize]> + AsRef<[usize]> + Default + cl_traits::Push<Input = usize>,
        OS: AsMut<[usize]> + AsRef<[usize]> + Default + cl_traits::Push<Input = usize>,
        rand::distributions::Standard: rand::distributions::Distribution<DATA>
      {
        fn len(&self) -> usize {
          self.as_ref().len()
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> ndsparse::csl::Csl<DA, DS, IS, OS>
        where
          R: Rng,
        {
          let nnz = self.as_ref().len();
          let mut dims = cl_traits::ArrayWrapper::default();
          let iter = (&mut *dims as &mut DA).slice_mut().iter_mut();
          match nnz {
            0 => {}
            1 => iter.for_each(|dim| *dim = 1),
            _ => iter.for_each(|dim| *dim = rng.gen_range(1, nnz)),
          }
          ndsparse::csl::Csl::new_controlled_random_with_rand(dims, nnz, rng, |g, _| g.gen())
        }

        fn set_rnd_solution_domain<R>(&self, s: &mut ndsparse::csl::Csl<DA, DS, IS, OS>, idx: usize, rng: &mut R)
        where
          R: Rng,
        {
          let domain = &self[idx];
          let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
          s.data_mut()[idx] = domain_value;
        }
      }
    )+
  }
}

#[cfg(feature = "with_rand")]
array_impls!(
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32
);
