use crate::{
  gp::{MpDefinitions, MpVec, MphDefinitions, MphVec},
  Obj, ObjDirection,
};
use core::{cmp::Ordering, ops::RangeInclusive};

type DummyMpTy =
  MpVec<[RangeInclusive<f64>; 2], (ObjDirection, fn(&[f64; 2]) -> f64), f64, [f64; 2]>;
type DummyMphTy = MphVec<
  [RangeInclusive<f64>; 2],
  fn(&[f64; 2]) -> usize,
  (ObjDirection, fn(&[f64; 2]) -> f64),
  f64,
  [f64; 2],
>;

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
    match od.is_better(first, second) {
      Some(Ordering::Greater) => {
        has_at_least_one_better_value = true;
      }
      Some(Ordering::Less) => return Ordering::Less,
      Some(_) | None => {}
    };
  }
  if has_at_least_one_better_value {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
}

fn obj(_: &[f64; 2]) -> f64 {
  0.0
}

fn hc(_: &[f64; 2]) -> usize {
  0
}

pub fn dummy_mp() -> DummyMpTy {
  MpVec::with_capacity(
    MpDefinitions {
      domain: [0.0..=0.0, 0.0..=0.0],
      hard_cstrs: Default::default(),
      name: Default::default(),
      objs: alloc::vec![(ObjDirection::Min, obj), (ObjDirection::Min, obj),],
      soft_cstrs: Default::default(),
    },
    4,
  )
}

pub fn dummy_mp_with_solutions() -> DummyMpTy {
  let mut problem = dummy_mp();
  problem
    .rslts_mut()
    .constructor()
    .or_os_iter([4.0, 8.0].iter().cloned(), [2.0, 2.0])
    .or_os_iter([8.0, 8.0].iter().cloned(), [3.0, 3.0])
    .or_os_iter([8.0, 12.0].iter().cloned(), [4.0, 4.0])
    .or_os_iter([12.0, 12.0].iter().cloned(), [5.0, 5.0]);
  problem
}

pub fn dummy_mph() -> DummyMphTy {
  MphVec::with_capacity(
    MphDefinitions {
      domain: [0.0..=0.0, 0.0..=0.0],
      hard_cstrs: alloc::vec![hc, hc],
      name: Default::default(),
      objs: alloc::vec![(ObjDirection::Min, obj), (ObjDirection::Min, obj)],
      soft_cstrs: Default::default(),
    },
    4,
  )
}

pub fn dummy_mph_with_solutions() -> DummyMphTy {
  let mut problem = dummy_mph();
  problem
    .rslts_mut()
    .constructor()
    .or_hcos_iter([1, 2].iter().cloned(), [4.0, 8.0].iter().cloned(), [2.0, 2.0])
    .or_hcos_iter([1, 2].iter().cloned(), [8.0, 8.0].iter().cloned(), [3.0, 3.0])
    .or_hcos_iter([1, 2].iter().cloned(), [8.0, 12.0].iter().cloned(), [4.0, 4.0])
    .or_hcos_iter([1, 2].iter().cloned(), [12.0, 12.0].iter().cloned(), [5.0, 5.0]);
  problem
}
