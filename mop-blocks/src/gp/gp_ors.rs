//! ORH (Optimization *R*esults with *H*ard constraints and *O*bjectives)

use crate::{
  dr_matrix::DrMatrix,
  gp::{GpDefinitions, GpOrMut, GpOrRef, GpOrsConstructor, NoCstrRslts},
};
use alloc::vec::Vec;
use cl_traits::{Clear, Remove, Storage, Truncate, WithCapacity};

pub type GpOrsMut<'a, HCR, OR, S, SCR> =
  GpOrs<&'a mut [HCR], &'a mut [OR], &'a mut [SCR], &'a mut [S]>;
pub type GpOrsRef<'a, HCR, OR, S, SCR> = GpOrs<&'a [HCR], &'a [OR], &'a [SCR], &'a [S]>;

pub type MpOrs<ORS, SS> = GpOrs<NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type MpOrsMut<'a, OR, S> = GpOrsMut<'a, (), OR, S, ()>;
pub type MpOrsRef<'a, OR, S> = GpOrsRef<'a, (), OR, S, ()>;
pub type MpOrsVec<OR, S> = MpOrs<Vec<OR>, Vec<S>>;

pub type MphOrs<HCRS, ORS, SS> = GpOrs<HCRS, ORS, NoCstrRslts, SS>;
pub type MphOrsMut<'a, OR, S> = GpOrsMut<'a, usize, OR, S, ()>;
pub type MphOrsRef<'a, OR, S> = GpOrsRef<'a, usize, OR, S, ()>;
pub type MphOrsVec<OR, S> = MphOrs<Vec<usize>, Vec<OR>, Vec<S>>;

pub type MphsOrs<HCRS, ORS, SCRS, SS> = GpOrs<HCRS, ORS, SCRS, SS>;
pub type MphsOrsMut<'a, OR, S> = GpOrsMut<'a, usize, OR, S, ()>;
pub type MphsOrsRef<'a, OR, S> = GpOrsRef<'a, usize, OR, S, ()>;
pub type MphsOrsVec<OR, S> = MphsOrs<Vec<usize>, Vec<OR>, Vec<usize>, Vec<S>>;

pub type SpOrs<ORS, SS> = GpOrs<NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type SpOrsMut<'a, OR, S> = GpOrsRef<'a, (), OR, S, ()>;
pub type SpOrsRef<'a, OR, S> = GpOrsMut<'a, (), OR, S, ()>;
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
  HCRS: Storage<Item = HCR>,
  ORS: Storage<Item = OR>,
  SCRS: Storage<Item = SCR>,
  SS: Storage<Item = S>,
{
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
  HCRS: AsMut<[HCR]> + Storage<Item = HCR>,
  ORS: AsMut<[OR]> + Storage<Item = OR>,
  SCRS: AsMut<[SCR]> + Storage<Item = SCR>,
  SS: AsMut<[S]> + Storage<Item = S>,
{
  pub fn get_mut(&mut self, idx: usize) -> Option<GpOrMut<'_, HCR, OR, S, SCR>> {
    Some(GpOrMut {
      hard_cstr_rslts: self.hard_cstr_rslts.row_mut(idx)?,
      obj_rslts: self.obj_rslts.row_mut(idx)?,
      soft_cstr_rslts: self.soft_cstr_rslts.row_mut(idx)?,
      solution: self.solutions.as_mut().get_mut(idx)?,
    })
  }

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

  pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = GpOrMut<'a, HCR, OR, S, SCR>>
  where
    HCR: 'a,
    OR: 'a,
    S: 'a,
    SCR: 'a,
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

  pub fn remove(&mut self, idx: usize)
  where
    HCRS: Truncate<Input = usize>,
    SCRS: Truncate<Input = usize>,
    ORS: Truncate<Input = usize>,
    SS: Remove<Input = usize>,
  {
    self.hard_cstr_rslts.remove_row(idx);
    self.obj_rslts.remove_row(idx);
    self.soft_cstr_rslts.remove_row(idx);
    self.solutions.remove(idx);
  }

  pub fn swap(&mut self, a: usize, b: usize) {
    self.hard_cstr_rslts.swap_rows(a, b);
    self.obj_rslts.swap_rows(a, b);
    self.soft_cstr_rslts.swap_rows(a, b);
    self.solutions.as_mut().swap(a, b);
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: AsRef<[HCR]> + Storage<Item = HCR>,
  ORS: AsRef<[OR]> + Storage<Item = OR>,
  SCRS: AsRef<[SCR]> + Storage<Item = SCR>,
  SS: AsRef<[S]> + Storage<Item = S>,
{
  pub fn as_ref(&self) -> GpOrsRef<'_, HCR, OR, S, SCR> {
    GpOrsRef {
      hard_cstr_rslts: self.hard_cstr_rslts.as_ref(),
      soft_cstr_rslts: self.soft_cstr_rslts.as_ref(),
      obj_rslts: self.obj_rslts.as_ref(),
      solutions: self.solutions.as_ref(),
    }
  }

  pub fn get(&self, idx: usize) -> Option<GpOrRef<'_, HCR, OR, S, SCR>> {
    Some(GpOrRef {
      hard_cstr_rslts: self.hard_cstr_rslts.row(idx)?,
      obj_rslts: self.obj_rslts.row(idx)?,
      soft_cstr_rslts: self.soft_cstr_rslts.row(idx)?,
      solution: self.solutions.as_ref().get(idx)?,
    })
  }

  pub fn iter<'a>(&'a self) -> impl Iterator<Item = GpOrRef<'a, HCR, OR, S, SCR>>
  where
    HCR: 'a,
    OR: 'a,
    S: 'a,
    SCR: 'a,
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
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrs<HCRS, ORS, SCRS, SS>
where
  HCRS: Storage<Item = HCR> + WithCapacity<Input = usize>,
  ORS: Storage<Item = OR> + WithCapacity<Input = usize>,
  SCRS: Storage<Item = SCR> + WithCapacity<Input = usize>,
  SS: Storage<Item = S> + WithCapacity<Input = usize>,
{
  pub fn with_capacity<HC, HCS, D, O, OS, SC, SCS>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts_num: usize,
  ) -> Self
  where
    HCS: AsRef<[HC]> + Storage<Item = HC>,
    OS: AsRef<[O]> + Storage<Item = O>,
    SCS: AsRef<[SC]> + Storage<Item = SC>,
  {
    let hard_cstr_rslts = DrMatrix::with_capacity(rslts_num, defs.hard_cstrs().len());
    let obj_rslts = DrMatrix::with_capacity(rslts_num, defs.objs().len());
    let soft_cstr_rslts = DrMatrix::with_capacity(rslts_num, defs.soft_cstrs().len());
    let solutions = SS::with_capacity(rslts_num);
    Self { hard_cstr_rslts, obj_rslts, soft_cstr_rslts, solutions }
  }
}
