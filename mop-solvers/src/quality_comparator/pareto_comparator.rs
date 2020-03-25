use crate::{quality_comparator::QualityComparator, utils::verify_pareto_dominance};
use core::{cmp::Ordering, marker::PhantomData};
use mop_blocks::{mph::MphOrRef, Obj, ObjDirection};

#[derive(Debug)]
pub struct ParetoComparator<M, T>
where
  M: ?Sized,
{
  phantom: PhantomData<(T, M)>,
}

impl<O, OR, S> ParetoComparator<[O], MphOrRef<'_, OR, S>>
where
  O: Obj<OR, S>,
{
  fn all_objs_have_the_same_direction(objs: &[O]) -> (bool, ObjDirection) {
    let od = objs.first().unwrap().obj_direction();
    let mut has_single_direction = true;
    for obj in objs.iter().skip(1) {
      if obj.obj_direction() != od {
        has_single_direction = false;
        break;
      }
    }
    (has_single_direction, od)
  }

  fn do_is_best<F>(a: &MphOrRef<'_, OR, S>, b: &MphOrRef<'_, OR, S>, has_better_objs: F) -> bool
  where
    F: Fn(&MphOrRef<'_, OR, S>, &MphOrRef<'_, OR, S>) -> bool,
  {
    let a_violations: usize = a.hard_cstrs().iter().sum();
    let b_violations: usize = b.hard_cstrs().iter().sum();
    let a_is_feasible_and_b_is_not = b_violations > 0 && a_violations == 0;
    let both_are_infeasible_and_a_has_less_violations = || {
      let both_are_infeasible = a_violations > 0 && b_violations > 0;
      both_are_infeasible && a_violations < b_violations
    };
    let both_are_feasible_and_a_dominates_b = || {
      let both_are_feasible = a_violations == 0 && b_violations == 0;
      both_are_feasible && has_better_objs(a, b)
    };
    a_is_feasible_and_b_is_not
      || both_are_infeasible_and_a_has_less_violations()
      || both_are_feasible_and_a_dominates_b()
  }
}

impl<O, OR, S> QualityComparator<[O], MphOrRef<'_, OR, S>>
  for ParetoComparator<[O], MphOrRef<'_, OR, S>>
where
  O: Obj<OR, S>,
  OR: PartialOrd,
{
  fn is_better(&self, objs: &[O], a: &MphOrRef<'_, OR, S>, b: &MphOrRef<'_, OR, S>) -> bool {
    let (has_same_direction, od) = Self::all_objs_have_the_same_direction(objs);
    if has_same_direction {
      Self::do_is_best(&a, &b, |a, b| {
        od.partial_cmp(a.objs_avg(), b.objs_avg()).unwrap() == Ordering::Greater
      })
    } else {
      Self::do_is_best(&a, &b, |a, b| {
        verify_pareto_dominance(objs, a.objs(), b.objs()) == Ordering::Greater
      })
    }
  }
}

impl<M, T> Default for ParetoComparator<M, T>
where
  M: ?Sized,
{
  fn default() -> Self {
    Self { phantom: PhantomData }
  }
}
