use core::{
  cmp::Ordering,
  ops::{Mul, Range, Sub},
};
use mop_blocks::Obj;
use num_traits::{NumCast, Pow, Zero};
use rand::{distributions::uniform::SampleUniform, Rng};

pub fn euclidean_distance<T>(first: &[T], second: &[T]) -> T
where
  T: Copy + Mul<T, Output = T> + NumCast + Pow<T, Output = T> + Sub<T, Output = T> + Zero,
{
  let mut distance = T::zero();
  for (a, b) in first.iter().copied().zip(second.iter().copied()) {
    let diff = b - a;
    distance = distance + diff * diff;
  }
  distance.pow(T::from(0.5).unwrap())
}

/// Verifies if the set of the first values dominates the set of the second values.
pub fn verify_pareto_dominance<O, OR, S, T>(
  objs_defs: &[O],
  first_values: &[T],
  second_values: &[T],
) -> Ordering
where
  O: Obj<OR, S>,
  T: PartialOrd,
{
  let iter = first_values
    .iter()
    .zip(second_values)
    .enumerate()
    .map(|(idx, (a, b))| (objs_defs[idx].obj_direction(), a, b));
  let mut has_at_least_one_better_value = false;
  for (od, first, second) in iter {
    match od.partial_cmp(first, second).unwrap() {
      Ordering::Greater => {
        has_at_least_one_better_value = true;
      }
      Ordering::Less => return Ordering::Less,
      _ => {}
    };
  }
  if has_at_least_one_better_value {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
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
