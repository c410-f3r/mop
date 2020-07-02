#[cfg(all(feature = "with-ndsparse", feature = "with-rand"))]
use cl_traits::{Array, ArrayWrapper, Push, Storage};
#[cfg(feature = "with-rand")]
use {
  cl_traits::create_array,
  core::ops::RangeInclusive,
  rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
  },
};

pub trait Domain<S> {
  type Error;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn len(&self) -> usize;

  #[cfg(feature = "with-rand")]
  fn new_random_solution<R>(&self, rng: &mut R) -> Result<S, Self::Error>
  where
    R: Rng;

  #[cfg(feature = "with-rand")]
  fn set_rnd_domain<R>(&self, s: &mut S, idx: usize, rng: &mut R)
  where
    R: Rng;
}

#[cfg(feature = "with-rand")]
macro_rules! array_impls {
  ($($N:expr),+) => {
    $(
      impl<T> Domain<[T; $N]> for [RangeInclusive<T>; $N]
      where
        T: Copy + SampleUniform,
      {
        type Error = core::convert::Infallible;

        fn len(&self) -> usize {
          $N
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> Result<[T; $N], Self::Error>
        where
          R: Rng,
        {
          Ok(create_array(|idx| {
            Uniform::from(*self[idx].start()..=*self[idx].end()).sample(rng)
          }))
        }

        fn set_rnd_domain<R>(&self, s: &mut [T; $N], idx: usize, rng: &mut R)
        where
          R: Rng,
        {
          let domain = &self[idx];
          let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
          s[idx] = domain_value;
        }
      }

      impl<T> Domain<arrayvec::ArrayVec<[T; $N]>> for arrayvec::ArrayVec<[RangeInclusive<T>; $N]>
      where
        T: Copy + SampleUniform,
      {
        type Error = core::convert::Infallible;

        fn len(&self) -> usize {
          self.len()
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> Result<arrayvec::ArrayVec<[T; $N]>, Self::Error>
        where
          R: Rng,
        {
          let mut s = arrayvec::ArrayVec::new();
          for domain in self.iter() {
            s.push(Uniform::from(*domain.start()..=*domain.end()).sample(rng));
          }
          Ok(s)
        }

        fn set_rnd_domain<R>(&self, s: &mut arrayvec::ArrayVec<[T; $N]>, idx: usize, rng: &mut R)
        where
          R: Rng,
        {
          let domain = &self[idx];
          let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
          s[idx] = domain_value;
        }
      }

      #[cfg(feature = "with-ndsparse")]
      impl<DA, DATA, DS, IS, OS> Domain<ndsparse::csl::Csl<DA, DS, IS, OS>> for [RangeInclusive<DATA>; $N]
      where
        DA: Array<Item = usize> + Copy + Default,
        DATA: Copy + SampleUniform,
        DS: AsMut<[DATA]> + AsRef<[DATA]> + Default + Push<Input = DATA> + Storage<Item = DATA>,
        IS: AsMut<[usize]> + AsRef<[usize]> + Default + Push<Input = usize>,
        OS: AsMut<[usize]> + AsRef<[usize]> + Default + Push<Input = usize>,
        rand::distributions::Standard: Distribution<DATA>
      {
        type Error = crate::Error;

        fn len(&self) -> usize {
          self.as_ref().len()
        }

        fn new_random_solution<R>(&self, rng: &mut R) -> Result<ndsparse::csl::Csl<DA, DS, IS, OS>, Self::Error>
        where
          R: Rng,
        {
          let nnz = self.as_ref().len();
          let dims = ArrayWrapper::default();
          let mut array: DA = *dims;
          let iter = array.slice_mut().iter_mut();
          match nnz {
            0 => {}
            1 => iter.for_each(|dim| *dim = 1),
            _ => iter.for_each(|dim| *dim = rng.gen_range(1, nnz)),
          }
          Ok(ndsparse::csl::Csl::new_controlled_random_rand(dims, nnz, rng, |g, _| g.gen()).map_err(|_| crate::Error::Other("Error"))?)
        }

        fn set_rnd_domain<R>(&self, s: &mut ndsparse::csl::Csl<DA, DS, IS, OS>, idx: usize, rng: &mut R)
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

#[cfg(feature = "with-rand")]
array_impls!(
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32
);
