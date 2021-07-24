use crate::{gp::MpOrRef, quality_comparator::QualityComparator, Obj};
use core::cmp::Ordering;

#[derive(Debug)]
pub struct ObjsAvg;

impl<O, OR, S> QualityComparator<[O], MpOrRef<'_, OR, S>> for ObjsAvg
where
  O: Obj<OR, S>,
  OR: PartialOrd,
{
  #[inline]
  fn is_better(&self, objs: &[O], a: &MpOrRef<'_, OR, S>, b: &MpOrRef<'_, OR, S>) -> bool {
    let mut times_a_is_better_than_b = 0;
    let mut times_b_is_better_than_a = 0;
    for (obj, (first_or, second_or)) in objs.iter().zip(a.obj_rslts().iter().zip(b.obj_rslts())) {
      match obj.obj_direction().is_better(first_or, second_or) {
        Some(Ordering::Greater) => times_a_is_better_than_b += 1,
        Some(Ordering::Less) => times_b_is_better_than_a += 1,
        Some(_) | None => {}
      }
    }
    times_a_is_better_than_b > times_b_is_better_than_a
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    quality_comparator::{ObjsAvg, QualityComparator},
    utils::dummy_mp,
  };

  #[test]
  fn objs_avg() {
    let mut problem = dummy_mp();
    let (defs, a) = problem.parts_mut();
    let _ = a.constructor().or_os_iter([0.0, 2.0].iter().cloned(), [10.0, 20.0]);
    let _ = a.constructor().or_os_iter([2.0, 2.0].iter().cloned(), [10.0, 20.0]);
    assert_eq!(ObjsAvg.is_better(defs.objs(), &a.get(0).unwrap(), &a.get(1).unwrap()), true);
  }
}
