use crate::{
  dr_matrix::DrMatrixRowsConstructor,
  gp::{GpOrRef, GpOrsRef, NoCstrRslts},
};
use cl_traits::{Length, Push, Storage};
use core::iter::Extend;

pub type MpOrsConstructor<'a, ORS, SS> = GpOrsConstructor<'a, NoCstrRslts, ORS, NoCstrRslts, SS>;
pub type MphOrsConstructor<'a, HCRS, ORS, SS> = GpOrsConstructor<'a, HCRS, ORS, NoCstrRslts, SS>;
pub type MphsOrsConstructor<'a, HCRS, ORS, SCRS, SS> = GpOrsConstructor<'a, HCRS, ORS, SCRS, SS>;
pub type SpOrsConstructor<'a, ORS, SS> = GpOrsConstructor<'a, NoCstrRslts, ORS, NoCstrRslts, SS>;

/// Constructor for MPH-OR
///
/// # Types
///
/// * `ORS`: Objective Results Storage
/// * `S`: Solution
#[derive(Debug, PartialEq)]
pub struct GpOrsConstructor<'a, HCRS, ORS, SCRS, SS> {
  pub(crate) hard_cstr_rslts: DrMatrixRowsConstructor<'a, HCRS>,
  pub(crate) obj_rslts: DrMatrixRowsConstructor<'a, ORS>,
  pub(crate) soft_cstr_rslts: DrMatrixRowsConstructor<'a, SCRS>,
  pub(crate) solutions: &'a mut SS,
}

impl<'a, HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrsConstructor<'a, HCRS, ORS, SCRS, SS>
where
  HCR: Clone,
  HCRS: Extend<HCR> + Storage<Item = HCR>,
  OR: Clone,
  ORS: Extend<OR> + Storage<Item = OR>,
  SCR: Clone,
  SCRS: Extend<SCR> + Storage<Item = SCR>,
  SS: Storage<Item = S>,
{
  #[inline]
  pub fn or_ref(mut self, from: GpOrRef<'_, HCR, OR, S, SCR>) -> Option<Self>
  where
    S: Clone,
    SS: Push<Input = S>,
  {
    self.hard_cstr_rslts = self.hard_cstr_rslts.row_slice(from.hard_cstr_rslts)?;
    self.obj_rslts = self.obj_rslts.row_slice(from.obj_rslts)?;
    self.soft_cstr_rslts = self.soft_cstr_rslts.row_slice(from.soft_cstr_rslts)?;
    let _ = self.solutions.push(from.solution.clone());
    Some(self)
  }

  #[inline]
  pub fn ors_ref(mut self, other: GpOrsRef<'_, HCR, OR, S, SCR>) -> Option<Self>
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
    SS: Push<Input = S>,
  {
    for solution in si {
      self.hard_cstr_rslts = self.hard_cstr_rslts.fill_row(HCR::default());
      self.obj_rslts = self.obj_rslts.fill_row(OR::default());
      self.soft_cstr_rslts = self.soft_cstr_rslts.fill_row(SCR::default());
      let _ = self.solutions.push(solution.ok()?);
    }
    Some(self)
  }
}

impl<'a, HCR, HCRS, OR, ORS, S, SCR, SCRS, SS> GpOrsConstructor<'a, HCRS, ORS, SCRS, SS>
where
  OR: Default,
  ORS: Extend<OR> + Length + Storage<Item = OR>,
  SS: Push<Input = S> + Storage<Item = S>,
  HCR: Default,
  HCRS: Extend<HCR> + Length + Storage<Item = HCR>,
  SCR: Default,
  SCRS: Extend<SCR> + Length + Storage<Item = SCR>,
{
  #[inline]
  pub fn or_hcos_iter<HCRI, ORI>(self, hcri: HCRI, ori: ORI, solution: S) -> Self
  where
    HCRI: Iterator<Item = HCR>,
    ORI: Iterator<Item = OR>,
  {
    let scri = (0..self.soft_cstr_rslts.cols).map(|_| Default::default());
    self.or_hcossc_iter(hcri, ori, solution, scri)
  }

  #[inline]
  pub fn or_hcossc_iter<HCRI, ORI, SCRI>(mut self, hcri: HCRI, ori: ORI, s: S, scri: SCRI) -> Self
  where
    HCRI: Iterator<Item = HCR>,
    ORI: Iterator<Item = OR>,
    SCRI: Iterator<Item = SCR>,
  {
    self.hard_cstr_rslts = self.hard_cstr_rslts.row_iter(hcri);
    self.obj_rslts = self.obj_rslts.row_iter(ori);
    self.soft_cstr_rslts = self.soft_cstr_rslts.row_iter(scri);
    let _ = self.solutions.push(s);
    self
  }

  #[inline]
  pub fn or_os_iter<ORI>(self, ori: ORI, solution: S) -> Self
  where
    ORI: Iterator<Item = OR>,
  {
    let hcri = (0..self.hard_cstr_rslts.cols).map(|_| Default::default());
    let scri = (0..self.soft_cstr_rslts.cols).map(|_| Default::default());
    self.or_hcossc_iter(hcri, ori, solution, scri)
  }
}
