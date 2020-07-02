//! A fast and elitist multiobjective genetic algorithm: NSGA-II
//!
//! Applies the constrained comparator described in NSGA-II.

use crate::{
  gp::MphOrRef, quality_comparator::QualityComparator, utils::verify_pareto_dominance, Obj,
};
use core::cmp::Ordering;

#[derive(Debug)]
pub struct Nsga2;

impl Nsga2 {
  fn do_is_best<F, OR, S>(
    a: &MphOrRef<'_, OR, S>,
    b: &MphOrRef<'_, OR, S>,
    has_better_objs: F,
  ) -> bool
  where
    F: Fn(&MphOrRef<'_, OR, S>, &MphOrRef<'_, OR, S>) -> bool,
  {
    let a_violations: usize = a.hard_cstr_rslts().iter().sum();
    let b_violations: usize = b.hard_cstr_rslts().iter().sum();
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

impl<O, OR, S> QualityComparator<[O], MphOrRef<'_, OR, S>> for Nsga2
where
  O: Obj<OR, S>,
  OR: PartialOrd,
{
  fn is_better(&self, objs: &[O], a: &MphOrRef<'_, OR, S>, b: &MphOrRef<'_, OR, S>) -> bool {
    Self::do_is_best(&a, &b, |a, b| {
      verify_pareto_dominance(objs, a.obj_rslts(), b.obj_rslts()) == Ordering::Greater
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    quality_comparator::{Nsga2, QualityComparator},
    utils::dummy_mph,
  };

  #[test]
  fn nsga2_a_is_feasible_and_b_is_not() {
    let mut problem = dummy_mph();
    let (defs, a) = problem.parts_mut();
    a.constructor()
      .or_hcos_iter([0, 0].iter().cloned(), [0.0, 0.0].iter().cloned(), [10.0, 20.0])
      .unwrap()
      .or_hcos_iter([0, 1].iter().cloned(), [0.0, 0.0].iter().cloned(), [10.0, 20.0])
      .unwrap();
    assert_eq!(Nsga2.is_better(defs.objs(), &a.get(0).unwrap(), &a.get(1).unwrap()), true);
  }

  #[test]
  fn nsga2_both_are_infeasible_and_a_has_less_violations() {
    let mut problem = dummy_mph();
    let (defs, a) = problem.parts_mut();
    a.constructor()
      .or_hcos_iter([1, 1].iter().cloned(), [0.0, 0.0].iter().cloned(), [10.0, 20.0])
      .unwrap()
      .or_hcos_iter([2, 2].iter().cloned(), [0.0, 0.0].iter().cloned(), [10.0, 20.0])
      .unwrap();
    assert_eq!(Nsga2.is_better(defs.objs(), &a.get(0).unwrap(), &a.get(1).unwrap()), true);
  }

  #[test]
  fn nsga2_both_are_feasible_and_a_dominates_b() {
    let mut problem = dummy_mph();
    let (defs, a) = problem.parts_mut();
    a.constructor()
      .or_hcos_iter([0, 0].iter().cloned(), [0.0, 0.0].iter().cloned(), [10.0, 20.0])
      .unwrap()
      .or_hcos_iter([0, 0].iter().cloned(), [0.0, 2.0].iter().cloned(), [10.0, 20.0])
      .unwrap();
    assert_eq!(Nsga2.is_better(defs.objs(), &a.get(0).unwrap(), &a.get(1).unwrap()), true);
  }
}
