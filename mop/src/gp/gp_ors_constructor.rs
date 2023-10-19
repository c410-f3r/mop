use crate::{
  dr_matrix::DrMatrixRowsConstructor,
  gp::{GpOrRef, GpOrsRef, NoCstrRslts},
};
use cl_aux::{Length, Push, SingleTypeStorage};
use core::iter::Extend;

pub type MpOrsConstructor<'any, ORS, SS> =
  GpOrsConstructor<'any, NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type MphOrsConstructor<'any, HCRS, ORS, SS> =
  GpOrsConstructor<'any, HCRS, ORS, NoCstrRslts, SS>;
pub type MphsOrsConstructor<'any, HCRS, ORS, SCRS, SS> =
  GpOrsConstructor<'any, HCRS, ORS, SCRS, SS>;
pub type SpOrsConstructor<'any, ORS, SS> =
  GpOrsConstructor<'any, NoCstrRslts, ORS, NoCstrRslts, SS>;

/// Constructor for MPH-OR
///
/// # Types
///
/// * `ORS`: Objective Results Storage
/// * `S`: Solution
#[derive(Debug, PartialEq)]
pub struct GpOrsConstructor<'any, HCRS, ORS, SCRS, SS> {
  pub(crate) hard_cstr_rslts: DrMatrixRowsConstructor<'any, HCRS>,
  pub(crate) obj_rslts: DrMatrixRowsConstructor<'any, ORS>,
  pub(crate) soft_cstr_rslts: DrMatrixRowsConstructor<'any, SCRS>,
  pub(crate) solutions: &'any mut SS,
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrsConstructor<'_, HCRS, ORS, SCRS, SS>
where
  HCR: Clone,
  HCRS: Extend<HCR> + SingleTypeStorage<Item = HCR>,
  OR: Clone,
  ORS: Extend<OR> + SingleTypeStorage<Item = OR>,
  SCR: Clone,
  SCRS: Extend<SCR> + SingleTypeStorage<Item = SCR>,
  SS: SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn or_ref(mut self, from: &GpOrRef<'_, HCR, OR, S, SCR>) -> Option<Self>
  where
    S: Clone,
    SS: Push<S>,
  {
    self.hard_cstr_rslts = self.hard_cstr_rslts.row_slice(from.hard_cstr_rslts)?;
    self.obj_rslts = self.obj_rslts.row_slice(from.obj_rslts)?;
    self.soft_cstr_rslts = self.soft_cstr_rslts.row_slice(from.soft_cstr_rslts)?;
    self.solutions.push(from.solution.clone()).ok()?;
    Some(self)
  }

  #[inline]
  pub fn ors_ref(mut self, other: &GpOrsRef<'_, HCR, OR, S, SCR>) -> Option<Self>
  where
    S: Clone,
    SS: Extend<S>,
  {
    self.hard_cstr_rslts = self.hard_cstr_rslts.matrix_ref(other.hard_cstr_rslts)?;
    self.obj_rslts = self.obj_rslts.matrix_ref(other.obj_rslts)?;
    self.soft_cstr_rslts = self.soft_cstr_rslts.matrix_ref(other.soft_cstr_rslts)?;
    self.solutions.extend(other.solutions.iter().cloned());
    Some(self)
  }

  #[inline]
  pub fn ors_s_iter<E, SI>(mut self, si: SI) -> Option<Self>
  where
    HCR: Default,
    OR: Default,
    SCR: Default,
    SI: Iterator<Item = Result<S, E>>,
    SS: Push<S>,
  {
    for solution in si {
      self.hard_cstr_rslts = self.hard_cstr_rslts.fill_row(HCR::default());
      self.obj_rslts = self.obj_rslts.fill_row(OR::default());
      self.soft_cstr_rslts = self.soft_cstr_rslts.fill_row(SCR::default());
      self.solutions.push(solution.ok()?).ok()?;
    }
    Some(self)
  }
}

impl<HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrsConstructor<'_, HCRS, ORS, SCRS, SS>
where
  OR: Default,
  ORS: Extend<OR> + Length + SingleTypeStorage<Item = OR>,
  SS: Push<S> + SingleTypeStorage<Item = S>,
  HCR: Default,
  HCRS: Extend<HCR> + Length + SingleTypeStorage<Item = HCR>,
  SCR: Default,
  SCRS: Extend<SCR> + Length + SingleTypeStorage<Item = SCR>,
{
  #[inline]
  pub fn or_hcos_iter<HCRI, ORI>(&mut self, hcri: HCRI, ori: ORI, solution: S) -> &mut Self
  where
    HCRI: Iterator<Item = HCR>,
    ORI: Iterator<Item = OR>,
  {
    let scri = (0..self.soft_cstr_rslts.cols).map(|_| <_>::default());
    self.or_hcossc_iter(hcri, ori, solution, scri)
  }

  #[inline]
  pub fn or_hcossc_iter<HCRI, ORI, SCRI>(
    &mut self,
    hcri: HCRI,
    ori: ORI,
    s: S,
    scri: SCRI,
  ) -> &mut Self
  where
    HCRI: Iterator<Item = HCR>,
    ORI: Iterator<Item = OR>,
    SCRI: Iterator<Item = SCR>,
  {
    let _ = self.hard_cstr_rslts.row_iter(hcri);
    let _ = self.obj_rslts.row_iter(ori);
    let _ = self.soft_cstr_rslts.row_iter(scri);
    let _rslt = self.solutions.push(s);
    self
  }

  #[inline]
  pub fn or_os_iter<ORI>(&mut self, ori: ORI, solution: S) -> &mut Self
  where
    ORI: Iterator<Item = OR>,
  {
    let hcri = (0..self.hard_cstr_rslts.cols).map(|_| <_>::default());
    let scri = (0..self.soft_cstr_rslts.cols).map(|_| <_>::default());
    self.or_hcossc_iter(hcri, ori, solution, scri)
  }
}
