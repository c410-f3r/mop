//! ORH (Optimization *R*esults with *H*ard constraints and *O*bjectives)

use crate::{
  dr_matrix::DrMatrixVec,
  mph::{MphDefinitions, MphOrConstructor, MphOrMut, MphOrRef},
};
use alloc::vec::Vec;

/// MPH-ORS (Multi-objective Problem with Hard constraint - Optimization ResultS)
///
/// # Types
///
/// * `OR`: Objective Result
/// * `S`: Solution
#[derive(Clone, Debug)]
pub struct MphOrs<OR, S> {
  pub(crate) best_idx: Option<usize>,
  pub(crate) hard_cstrs: DrMatrixVec<usize>,
  pub(crate) objs_avg: Vec<OR>,
  pub(crate) objs: DrMatrixVec<OR>,
  pub(crate) results_num: usize,
  pub(crate) solutions: Vec<S>,
}

impl<OR, S> MphOrs<OR, S> {
  pub fn with_capacity<C, O, SD>(defs: &MphDefinitions<C, O, SD>, results_num: usize) -> Self {
    let hard_cstrs = DrMatrixVec::with_capacity(results_num, defs.hard_cstrs().len());
    let objs = DrMatrixVec::with_capacity(results_num, defs.objs().len());
    let objs_avg = Vec::with_capacity(results_num);
    let solutions = Vec::with_capacity(results_num);
    Self { best_idx: None, hard_cstrs, objs, objs_avg, results_num, solutions }
  }

  pub fn best(&self) -> Option<MphOrRef<'_, OR, S>> {
    self.best_idx.map(|x| MphOrRef {
      hard_cstrs: self.hard_cstrs.row(x),
      objs: self.objs.row(x),
      objs_avg: &self.objs_avg[x],
      solution: &self.solutions[x],
    })
  }

  pub fn best_objs(&self) -> Option<&[OR]> {
    self.best_idx.map(|x| self.objs.row(x))
  }

  pub fn clear(&mut self) {
    self.hard_cstrs.clear();
    self.objs.clear();
    self.objs_avg.clear();
    self.solutions.clear();
  }

  pub fn constructor(&mut self) -> MphOrConstructor<'_, OR, S> {
    MphOrConstructor {
      hard_cstrs: self.hard_cstrs.row_constructor(),
      objs: self.objs.row_constructor(),
      objs_avg: &mut self.objs_avg,
      solutions: &mut self.solutions,
    }
  }

  pub fn extend(&mut self, other: &Self)
  where
    OR: Copy,
    S: Clone,
  {
    self.hard_cstrs.extend(&other.hard_cstrs);
    self.objs.extend(&other.objs);
    self.objs_avg.extend(&other.objs_avg);
    self.solutions.extend(other.solutions.iter().cloned());
  }

  pub fn get(&self, idx: usize) -> MphOrRef<'_, OR, S> {
    MphOrRef {
      hard_cstrs: self.hard_cstrs.row(idx),
      objs: self.objs.row(idx),
      objs_avg: &self.objs_avg[idx],
      solution: &self.solutions[idx],
    }
  }

  pub fn get_mut(&mut self, idx: usize) -> MphOrMut<'_, OR, S> {
    MphOrMut {
      hard_cstrs: self.hard_cstrs.row_mut(idx),
      objs: self.objs.row_mut(idx),
      objs_avg: &mut self.objs_avg[idx],
      solution: &mut self.solutions[idx],
    }
  }

  pub fn get_two_mut(&mut self, smaller_idx: usize, bigger_idx: usize) -> [MphOrMut<'_, OR, S>; 2] {
    assert!(smaller_idx < bigger_idx && smaller_idx < self.len() && bigger_idx < self.len());
    let [first_o, second_o] = self.objs.two_rows_mut(smaller_idx, bigger_idx);
    let [first_oa, second_oa] = {
      let (first, second) = self.objs_avg.split_at_mut(bigger_idx);
      [&mut first[smaller_idx], &mut second[0]]
    };
    let [first_hcres, second_hcres] = self.hard_cstrs.two_rows_mut(smaller_idx, bigger_idx);
    let [first_s, second_s] = {
      let (first, second) = self.solutions.split_at_mut(bigger_idx);
      [&mut first[smaller_idx], &mut second[0]]
    };
    [
      MphOrMut { hard_cstrs: first_hcres, objs: first_o, objs_avg: first_oa, solution: first_s },
      MphOrMut {
        hard_cstrs: second_hcres,
        objs: second_o,
        objs_avg: second_oa,
        solution: second_s,
      },
    ]
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn iter(&self) -> impl Iterator<Item = MphOrRef<'_, OR, S>> {
    self
      .hard_cstrs
      .row_iter()
      .zip(self.objs.row_iter().zip(self.objs_avg.iter().zip(self.solutions.iter())))
      .map(|(hard_cstrs, (objs, (objs_avg, solution)))| MphOrRef {
        hard_cstrs,
        objs,
        objs_avg,
        solution,
      })
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = MphOrMut<'_, OR, S>> {
    self
      .hard_cstrs
      .row_iter_mut()
      .zip(self.objs.row_iter_mut().zip(self.objs_avg.iter_mut().zip(self.solutions.iter_mut())))
      .map(|(hard_cstrs, (objs, (objs_avg, solution)))| MphOrMut {
        hard_cstrs,
        objs,
        objs_avg,
        solution,
      })
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.solutions.len()
  }

  pub fn remove(&mut self, idx: usize) {
    self.hard_cstrs.remove_row(idx);
    self.objs.remove_row(idx);
    self.objs_avg.remove(idx);
    self.solutions.remove(idx);
  }

  pub fn results_num(&self) -> usize {
    self.results_num
  }

  pub fn set_best(&mut self, index: usize) {
    self.best_idx = Some(index);
  }

  pub fn swap(&mut self, a: usize, b: usize) {
    self.hard_cstrs.swap_rows(a, b);
    self.objs.swap_rows(a, b);
    self.objs_avg.swap(a, b);
    self.solutions.swap(a, b);
  }

  pub fn truncate(&mut self, until_idx: usize) {
    self.hard_cstrs.truncate(until_idx);
    self.objs.truncate(until_idx);
    self.objs_avg.truncate(until_idx);
    self.solutions.truncate(until_idx);
  }
}
