use crate::quality_comparator::QualityComparator;
use core::cmp::Ordering;
use mop_blocks::{mph::MphOrRef, Obj};

#[derive(Debug)]
pub struct ObjsAvg;

impl<O, OR, S> QualityComparator<[O], MphOrRef<'_, OR, S>> for ObjsAvg
where
  O: Obj<OR, S>,
  OR: PartialOrd,
{
  fn is_better(&self, objs: &[O], a: &MphOrRef<'_, OR, S>, b: &MphOrRef<'_, OR, S>) -> bool {
    let mut times_a_is_better_than_b = 0;
    let mut times_b_is_better_than_a = 0;
    for (obj, (first_or, second_or)) in objs.iter().zip(a.objs().iter().zip(b.objs())) {
      match obj.obj_direction().partial_cmp(first_or, second_or).unwrap() {
        Ordering::Greater => times_a_is_better_than_b += 1,
        Ordering::Less => times_b_is_better_than_a += 1,
        _ => {}
      }
    }
    times_a_is_better_than_b > times_b_is_better_than_a
  }
}
