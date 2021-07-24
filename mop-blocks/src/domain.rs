#[cfg(all(feature = "with-ndsparse", feature = "with-rand"))]
use cl_traits::{Push, Storage};
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

  #[inline]
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
impl<T, const N: usize> Domain<[T; N]> for [RangeInclusive<T>; N]
where
  T: Copy + SampleUniform,
{
  type Error = core::convert::Infallible;

  #[inline]
  fn len(&self) -> usize {
    N
  }

  #[inline]
  fn new_random_solution<R>(&self, rng: &mut R) -> Result<[T; N], Self::Error>
  where
    R: Rng,
  {
    Ok(create_array(|idx| Uniform::from(*self[idx].start()..=*self[idx].end()).sample(rng)))
  }

  #[inline]
  fn set_rnd_domain<R>(&self, s: &mut [T; N], idx: usize, rng: &mut R)
  where
    R: Rng,
  {
    let domain = &self[idx];
    let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
    s[idx] = domain_value;
  }
}

#[cfg(feature = "with-rand")]
impl<T, const N: usize> Domain<arrayvec::ArrayVec<T, N>>
  for arrayvec::ArrayVec<RangeInclusive<T>, N>
where
  T: Copy + SampleUniform,
{
  type Error = core::convert::Infallible;

  #[inline]
  fn len(&self) -> usize {
    self.len()
  }

  #[inline]
  fn new_random_solution<R>(&self, rng: &mut R) -> Result<arrayvec::ArrayVec<T, N>, Self::Error>
  where
    R: Rng,
  {
    let mut s = arrayvec::ArrayVec::new();
    for domain in self.iter() {
      s.push(Uniform::from(*domain.start()..=*domain.end()).sample(rng));
    }
    Ok(s)
  }

  #[inline]
  fn set_rnd_domain<R>(&self, s: &mut arrayvec::ArrayVec<T, N>, idx: usize, rng: &mut R)
  where
    R: Rng,
  {
    let domain = &self[idx];
    let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
    s[idx] = domain_value;
  }
}

#[cfg(all(feature = "with-ndsparse", feature = "with-rand"))]
impl<DATA, DS, IS, OS, const D: usize, const N: usize> Domain<ndsparse::csl::Csl<DS, IS, OS, D>>
  for [RangeInclusive<DATA>; N]
where
  DATA: Copy + SampleUniform,
  DS: AsMut<[DATA]> + AsRef<[DATA]> + Default + Push<Input = DATA> + Storage<Item = DATA>,
  IS: AsMut<[usize]> + AsRef<[usize]> + Default + Push<Input = usize>,
  OS: AsMut<[usize]> + AsRef<[usize]> + Default + Push<Input = usize>,
  rand::distributions::Standard: Distribution<DATA>,
{
  type Error = crate::Error;

  #[inline]
  fn len(&self) -> usize {
    self.as_ref().len()
  }

  #[inline]
  fn new_random_solution<R>(
    &self,
    rng: &mut R,
  ) -> Result<ndsparse::csl::Csl<DS, IS, OS, D>, Self::Error>
  where
    R: Rng,
  {
    let nnz = self.as_ref().len();
    let dims = cl_traits::default_array();
    let mut array: [usize; D] = dims;
    let iter = array.iter_mut();
    match nnz {
      0 => {}
      1 => iter.for_each(|dim| *dim = 1),
      _ => iter.for_each(|dim| *dim = rng.gen_range(1..nnz)),
    }
    ndsparse::csl::Csl::new_controlled_random_rand(dims, nnz, rng, |g, _| g.gen())
      .map_err(crate::Error::NdsparseError)
  }

  #[inline]
  fn set_rnd_domain<R>(&self, s: &mut ndsparse::csl::Csl<DS, IS, OS, D>, idx: usize, rng: &mut R)
  where
    R: Rng,
  {
    let domain = &self[idx];
    let domain_value = Uniform::from(*domain.start()..=*domain.end()).sample(rng);
    s.data_mut()[idx] = domain_value;
  }
}
