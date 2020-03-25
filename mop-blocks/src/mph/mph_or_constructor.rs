use crate::{dr_matrix::DrMatrixRowConstructorVec, mph::MphOrRef};
use alloc::vec::Vec;

/// Constructor for MPH-OR
///
/// # Types
///
/// * `ORS`: Objective Results Storage
/// * `S`: Solution
#[derive(Debug, PartialEq)]
pub struct MphOrConstructor<'a, OR, S> {
  pub(crate) hard_cstrs: DrMatrixRowConstructorVec<'a, usize>,
  pub(crate) objs: DrMatrixRowConstructorVec<'a, OR>,
  pub(crate) objs_avg: &'a mut Vec<OR>,
  pub(crate) solutions: &'a mut Vec<S>,
}

impl<'a, OR, S> MphOrConstructor<'a, OR, S> {
  pub fn commit(self, objs_avg: OR, solution: S) {
    self.hard_cstrs.commit();
    self.objs.commit();
    self.objs_avg.push(objs_avg);
    self.solutions.push(solution);
  }

  pub fn copy_result(mut self, from: &MphOrRef<'_, OR, S>) -> Self
  where
    OR: Copy,
  {
    self.hard_cstrs = self.hard_cstrs.copy_values_from_row(from.hard_cstrs);
    self.objs = self.objs.copy_values_from_row(from.objs);
    self
  }

  pub fn push_hard_cstr(mut self, result: usize) -> Self {
    self.hard_cstrs = self.hard_cstrs.push_value(result);
    self
  }

  pub fn push_obj(mut self, value: OR) -> Self {
    self.objs = self.objs.push_value(value);
    self
  }
}
