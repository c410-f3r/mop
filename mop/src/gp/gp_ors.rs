//! ORH (Optimization *R*esults with *H*ard constraints and *O*bjectives)

use crate::{
  dr_matrix::DrMatrix,
  gp::{GpDefinitions, GpOrMut, GpOrRef, GpOrsConstructor, NoCstrRslts},
};
use alloc::vec::Vec;
use cl_aux::{Clear, Remove, SingleTypeStorage, Truncate, WithCapacity};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub type GpOrsMut<'any, HCR, OR, S, SCR> =
  GpOrs<&'any mut [HCR], &'any mut [OR], &'any mut [SCR], &'any mut [S]>;
pub type GpOrsRef<'any, HCR, OR, S, SCR> = GpOrs<&'any [HCR], &'any [OR], &'any [SCR], &'any [S]>;

pub type MpOrs<ORS, SS> = GpOrs<NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type MpOrsMut<'any, OR, S> = GpOrsMut<'any, (), OR, S, ()>;
pub type MpOrsRef<'any, OR, S> = GpOrsRef<'any, (), OR, S, ()>;
pub type MpOrsVec<OR, S> = MpOrs<Vec<OR>, Vec<S>>;

pub type MphOrs<HCRS, ORS, SS> = GpOrs<HCRS, ORS, NoCstrRslts, SS>;
pub type MphOrsMut<'any, OR, S> = GpOrsMut<'any, usize, OR, S, ()>;
pub type MphOrsRef<'any, OR, S> = GpOrsRef<'any, usize, OR, S, ()>;
pub type MphOrsVec<OR, S> = MphOrs<Vec<usize>, Vec<OR>, Vec<S>>;

pub type MphsOrs<HCRS, ORS, SCRS, SS> = GpOrs<HCRS, ORS, SCRS, SS>;
pub type MphsOrsMut<'any, OR, S> = GpOrsMut<'any, usize, OR, S, ()>;
pub type MphsOrsRef<'any, OR, S> = GpOrsRef<'any, usize, OR, S, ()>;
pub type MphsOrsVec<OR, S> = MphsOrs<Vec<usize>, Vec<OR>, Vec<usize>, Vec<S>>;

pub type SpOrs<ORS, SS> = GpOrs<NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type SpOrsMut<'any, OR, S> = GpOrsRef<'any, (), OR, S, ()>;
pub type SpOrsRef<'any, OR, S> = GpOrsMut<'any, (), OR, S, ()>;
pub type SpOrsVec<OR, S> = SpOrs<Vec<OR>, Vec<S>>;

/// MPH-ORS (Multi-objective Problem with Hard constraint - Optimization ResultS)
///
/// This structure is generic over single or multi objective problems, constrained or not.
///
/// # Types
///
/// * `CRS`: Constraint Results Storage
/// * `ORS`: Objectives Results Storage
/// * `ORAS`: Objective Results Average Storage
/// * `SS`: Solutions Storage
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GpOrs<HCRS, ORS, SCRS, SS> {
  pub(crate) hard_cstr_rslts: DrMatrix<HCRS>,
  pub(crate) obj_rslts: DrMatrix<ORS>,
  pub(crate) soft_cstr_rslts: DrMatrix<SCRS>,
  pub(crate) solutions: SS,
}

impl<HCRS, ORS, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS> {
  #[inline]
  pub fn clear(&mut self)
  where
    HCRS: Clear,
    ORS: Clear,
    SCRS: Clear,
    SS: Clear,
  {
    self.hard_cstr_rslts.clear();
    self.obj_rslts.clear();
    self.soft_cstr_rslts.clear();
    self.solutions.clear();
  }

  #[inline]
  pub fn truncate(&mut self, until_idx: usize)
  where
    HCRS: Truncate<Input = usize>,
    ORS: Truncate<Input = usize>,
    SCRS: Truncate<Input = usize>,
    SS: Truncate<Input = usize>,
  {
    self.hard_cstr_rslts.truncate(until_idx);
    self.obj_rslts.truncate(until_idx);
    self.soft_cstr_rslts.truncate(until_idx);
    self.solutions.truncate(until_idx);
  }

  #[inline]
  pub fn rslts_num(&self) -> usize {
    self.obj_rslts.rows()
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: SingleTypeStorage<Item = HCR>,
  ORS: SingleTypeStorage<Item = OR>,
  SCRS: SingleTypeStorage<Item = SCR>,
  SS: SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn constructor(&mut self) -> GpOrsConstructor<'_, HCRS, ORS, SCRS, SS> {
    GpOrsConstructor {
      hard_cstr_rslts: self.hard_cstr_rslts.constructor(),
      obj_rslts: self.obj_rslts.constructor(),
      soft_cstr_rslts: self.soft_cstr_rslts.constructor(),
      solutions: &mut self.solutions,
    }
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: AsMut<[HCR]> + SingleTypeStorage<Item = HCR>,
  ORS: AsMut<[OR]> + SingleTypeStorage<Item = OR>,
  SCRS: AsMut<[SCR]> + SingleTypeStorage<Item = SCR>,
  SS: AsMut<[S]> + SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn get_mut(&mut self, idx: usize) -> Option<GpOrMut<'_, HCR, OR, S, SCR>> {
    Some(GpOrMut {
      hard_cstr_rslts: self.hard_cstr_rslts.row_mut(idx)?,
      obj_rslts: self.obj_rslts.row_mut(idx)?,
      soft_cstr_rslts: self.soft_cstr_rslts.row_mut(idx)?,
      solution: self.solutions.as_mut().get_mut(idx)?,
    })
  }

  #[inline]
  pub fn get_two_mut(&mut self, a: usize, b: usize) -> Option<[GpOrMut<'_, HCR, OR, S, SCR>; 2]> {
    let [first_os, second_os] = self.obj_rslts.two_rows_mut(a, b)?;
    let [first_hcs, second_hcs] = self.hard_cstr_rslts.two_rows_mut(a, b)?;
    let [first_scs, second_scs] = self.soft_cstr_rslts.two_rows_mut(a, b)?;
    let [first_s, second_s] = {
      let (first, second) = self.solutions.as_mut().split_at_mut(b);
      [&mut first[a], &mut second[0]]
    };
    Some([
      GpOrMut {
        hard_cstr_rslts: first_hcs,
        obj_rslts: first_os,
        soft_cstr_rslts: first_scs,
        solution: first_s,
      },
      GpOrMut {
        hard_cstr_rslts: second_hcs,
        obj_rslts: second_os,
        soft_cstr_rslts: second_scs,
        solution: second_s,
      },
    ])
  }

  #[inline]
  pub fn iter_mut<'any>(&'any mut self) -> impl Iterator<Item = GpOrMut<'any, HCR, OR, S, SCR>>
  where
    HCR: 'any,
    OR: 'any,
    S: 'any,
    SCR: 'any,
  {
    self
      .hard_cstr_rslts
      .row_iter_mut()
      .zip(
        self
          .obj_rslts
          .row_iter_mut()
          .zip(self.soft_cstr_rslts.row_iter_mut().zip(self.solutions.as_mut().iter_mut())),
      )
      .map(|(hard_cstr_rslts, (obj_rslts, (soft_cstr_rslts, solution)))| GpOrMut {
        hard_cstr_rslts,
        obj_rslts,
        soft_cstr_rslts,
        solution,
      })
  }

  #[inline]
  #[cfg(feature = "rayon")]
  pub fn par_iter_mut<'any>(
    &'any mut self,
  ) -> impl IndexedParallelIterator<Item = GpOrMut<'any, HCR, OR, S, SCR>>
  where
    HCR: Send + Sync + 'any,
    OR: Send + Sync + 'any,
    S: Send + Sync + 'any,
    SCR: Send + Sync + 'any,
  {
    self
      .hard_cstr_rslts
      .row_par_iter_mut()
      .zip_eq(self.obj_rslts.row_par_iter_mut().zip_eq(
        self.soft_cstr_rslts.row_par_iter_mut().zip(self.solutions.as_mut().par_iter_mut()),
      ))
      .map(|(hard_cstr_rslts, (obj_rslts, (soft_cstr_rslts, solution)))| GpOrMut {
        hard_cstr_rslts,
        obj_rslts,
        soft_cstr_rslts,
        solution,
      })
  }

  #[inline]
  #[must_use]
  pub fn remove(&mut self, idx: usize) -> bool
  where
    HCRS: Truncate<Input = usize>,
    SCRS: Truncate<Input = usize>,
    ORS: Truncate<Input = usize>,
    SS: Remove<Input = usize>,
  {
    if idx >= self.rslts_num() {
      return false;
    }
    let _ = self.hard_cstr_rslts.remove_row(idx);
    let _ = self.obj_rslts.remove_row(idx);
    let _ = self.soft_cstr_rslts.remove_row(idx);
    let _rslt = self.solutions.remove(idx);
    true
  }

  #[inline]
  #[must_use]
  pub fn swap(&mut self, a: usize, b: usize) -> bool {
    if a >= self.rslts_num() || b >= self.rslts_num() {
      return false;
    }
    let _ = self.hard_cstr_rslts.swap_rows(a, b);
    let _ = self.obj_rslts.swap_rows(a, b);
    let _ = self.soft_cstr_rslts.swap_rows(a, b);
    let _ = self.solutions.as_mut().swap(a, b);
    true
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: AsRef<[HCR]> + SingleTypeStorage<Item = HCR>,
  ORS: AsRef<[OR]> + SingleTypeStorage<Item = OR>,
  SCRS: AsRef<[SCR]> + SingleTypeStorage<Item = SCR>,
  SS: AsRef<[S]> + SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn as_ref(&self) -> GpOrsRef<'_, HCR, OR, S, SCR> {
    GpOrsRef {
      hard_cstr_rslts: self.hard_cstr_rslts.as_ref(),
      soft_cstr_rslts: self.soft_cstr_rslts.as_ref(),
      obj_rslts: self.obj_rslts.as_ref(),
      solutions: self.solutions.as_ref(),
    }
  }

  #[inline]
  pub fn get(&self, idx: usize) -> Option<GpOrRef<'_, HCR, OR, S, SCR>> {
    Some(GpOrRef {
      hard_cstr_rslts: self.hard_cstr_rslts.row(idx)?,
      obj_rslts: self.obj_rslts.row(idx)?,
      soft_cstr_rslts: self.soft_cstr_rslts.row(idx)?,
      solution: self.solutions.as_ref().get(idx)?,
    })
  }

  #[inline]
  pub fn iter<'any>(&'any self) -> impl Iterator<Item = GpOrRef<'any, HCR, OR, S, SCR>>
  where
    HCR: 'any,
    OR: 'any,
    S: 'any,
    SCR: 'any,
  {
    self
      .hard_cstr_rslts
      .row_iter()
      .zip(
        self
          .obj_rslts
          .row_iter()
          .zip(self.soft_cstr_rslts.row_iter().zip(self.solutions.as_ref().iter())),
      )
      .map(|(hard_cstr_rslts, (obj_rslts, (soft_cstr_rslts, solution)))| GpOrRef {
        hard_cstr_rslts,
        obj_rslts,
        soft_cstr_rslts,
        solution,
      })
  }

  #[inline]
  #[cfg(feature = "rayon")]
  pub fn par_iter<'any>(
    &'any self,
  ) -> impl IndexedParallelIterator<Item = GpOrRef<'any, HCR, OR, S, SCR>>
  where
    HCR: Send + Sync + 'any,
    OR: Send + Sync + 'any,
    S: Send + Sync + 'any,
    SCR: Send + Sync + 'any,
  {
    self
      .hard_cstr_rslts
      .row_par_iter()
      .zip_eq(
        self
          .obj_rslts
          .row_par_iter()
          .zip_eq(self.soft_cstr_rslts.row_par_iter().zip(self.solutions.as_ref().par_iter())),
      )
      .map(|(hard_cstr_rslts, (obj_rslts, (soft_cstr_rslts, solution)))| GpOrRef {
        hard_cstr_rslts,
        obj_rslts,
        soft_cstr_rslts,
        solution,
      })
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: SingleTypeStorage<Item = HCR> + WithCapacity<Input = usize>,
  ORS: SingleTypeStorage<Item = OR> + WithCapacity<Input = usize>,
  SCRS: SingleTypeStorage<Item = SCR> + WithCapacity<Input = usize>,
  SS: SingleTypeStorage<Item = S> + WithCapacity<Input = usize>,
{
  #[inline]
  pub fn with_capacity<HC, HCS, D, O, OS, SC, SCS>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts_num: usize,
  ) -> Self
  where
    HCS: AsRef<[HC]> + SingleTypeStorage<Item = HC>,
    OS: AsRef<[O]> + SingleTypeStorage<Item = O>,
    SCS: AsRef<[SC]> + SingleTypeStorage<Item = SC>,
  {
    let hard_cstr_rslts = DrMatrix::with_capacity(rslts_num, defs.hard_cstrs().len());
    let obj_rslts = DrMatrix::with_capacity(rslts_num, defs.objs().len());
    let soft_cstr_rslts = DrMatrix::with_capacity(rslts_num, defs.soft_cstrs().len());
    let solutions = SS::with_capacity(rslts_num);
    Self { hard_cstr_rslts, obj_rslts, soft_cstr_rslts, solutions }
  }
}
