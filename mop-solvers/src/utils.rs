use core::ops::{Mul, Range, Sub};
use num_traits::{NumCast, Pow, Zero};
use rand::{distributions::uniform::SampleUniform, Rng};

pub fn euclidean_distance<T>(first: &[T], second: &[T]) -> Option<T>
where
  T: Copy + Mul<T, Output = T> + NumCast + Pow<T, Output = T> + Sub<T, Output = T> + Zero,
{
  let mut distance = T::zero();
  for (a, b) in first.iter().copied().zip(second.iter().copied()) {
    let diff = b - a;
    distance = distance + diff * diff;
  }
  Some(distance.pow(NumCast::from(0.5)?))
}

/// Two ascending random numbers
pub fn two_asc_rnd_num<R, T>(rng: &mut R, range: Range<T>) -> [T; 2]
where
  R: Rng,
  T: Copy + SampleUniform + PartialOrd,
{
  let [first, second] = two_dist_rnd_num(rng, range);
  if first < second {
    [first, second]
  } else {
    [second, first]
  }
}

/// Two distinct random numbers
pub fn two_dist_rnd_num<R, T>(rng: &mut R, range: Range<T>) -> [T; 2]
where
  R: Rng,
  T: Copy + SampleUniform + PartialOrd,
{
  let first = rng.gen_range(range.start, range.end);
  let mut second = rng.gen_range(range.start, range.end);
  while first == second {
    second = rng.gen_range(range.start, range.end);
  }
  [first, second]
}
