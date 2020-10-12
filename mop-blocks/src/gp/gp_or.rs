use crate::gp::{NoCstrRslts, OneObj};
use alloc::vec::Vec;
use cl_traits::Storage;

pub type GpOrMut<'a, HCR, OR, S, SCR> = GpOr<&'a mut [HCR], &'a mut [OR], &'a mut S, &'a mut [SCR]>;
pub type GpOrRef<'a, HCR, OR, S, SCR> = GpOr<&'a [HCR], &'a [OR], &'a S, &'a [SCR]>;
pub type GpOrVec<HCR, OR, S, SCR> = GpOr<Vec<HCR>, Vec<OR>, S, Vec<SCR>>;

pub type MpOr<ORS, S> = GpOr<NoCstrRslts, ORS, S, NoCstrRslts>;
pub type MpOrMut<'a, OR, S> = GpOrMut<'a, (), OR, S, ()>;
pub type MpOrRef<'a, OR, S> = GpOrRef<'a, (), OR, S, ()>;
pub type MpOrVec<OR, S> = MpOr<Vec<OR>, S>;

pub type MphOr<HCRS, ORS, S> = GpOr<HCRS, ORS, S, NoCstrRslts>;
pub type MphOrMut<'a, OR, S> = GpOrMut<'a, usize, OR, S, ()>;
pub type MphOrRef<'a, OR, S> = GpOrRef<'a, usize, OR, S, ()>;
pub type MphOrVec<OR, S> = MphOr<Vec<usize>, Vec<OR>, S>;

pub type MphsOr<HCRS, ORS, S, SCRS> = GpOr<HCRS, ORS, S, SCRS>;
pub type MphsOrMut<'a, OR, S> = GpOrMut<'a, usize, OR, S, usize>;
pub type MphsOrRef<'a, OR, S> = GpOrRef<'a, usize, OR, S, usize>;
pub type MphsOrVec<OR, S> = MphsOr<Vec<usize>, Vec<OR>, S, Vec<usize>>;

pub type SpOr<OR, S> = GpOr<NoCstrRslts, OneObj<OR>, S, NoCstrRslts>;
pub type SpOrMut<'a, OR, S> = GpOrMut<'a, (), OR, S, ()>;
pub type SpOrRef<'a, OR, S> = GpOrRef<'a, (), OR, S, ()>;

/// GP-OR (Generic Problem - Optimization Result)
///
/// This structure is generic over single or multi objective problems, constrained or not.
///
/// # Types
///
/// * `CRS`: Constraint Results Storage
/// * `ORS`: Objective Results Storage
/// * `S`: Solution
#[cfg_attr(feature = "with-serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct GpOr<HCRS, ORS, S, SCRS> {
  pub(crate) hard_cstr_rslts: HCRS,
  pub(crate) obj_rslts: ORS,
  pub(crate) soft_cstr_rslts: SCRS,
  pub(crate) solution: S,
}

impl<HCRS, ORS, S, SCRS> GpOr<HCRS, ORS, S, SCRS> {
  #[inline]
  pub fn new(hard_cstr_rslts: HCRS, obj_rslts: ORS, soft_cstr_rslts: SCRS, solution: S) -> Self {
    Self { hard_cstr_rslts, obj_rslts, soft_cstr_rslts, solution }
  }

  #[inline]
  pub fn solution(&self) -> &S {
    &self.solution
  }

  #[inline]
  pub fn solution_mut(&mut self) -> &mut S {
    &mut self.solution
  }
}

impl<HCRS, ORS, S, SCRS> GpOr<HCRS, ORS, S, SCRS>
where
  HCRS: AsRef<[usize]>,
{
  #[inline]
  pub fn hard_cstr_rslts(&self) -> &[usize] {
    self.hard_cstr_rslts.as_ref()
  }
}

impl<HCRS, OR, S, SCRS> GpOr<HCRS, [OR; 1], S, SCRS> {
  #[inline]
  pub fn obj_rslt(&self) -> &OR {
    &self.obj_rslts[0]
  }
}

impl<HCRS, OR, ORS, S, SCRS> GpOr<HCRS, ORS, S, SCRS>
where
  ORS: AsRef<[OR]> + Storage<Item = OR>,
{
  #[inline]
  pub fn obj_rslts(&self) -> &[OR] {
    self.obj_rslts.as_ref()
  }
}

impl<HCRS, ORS, S, SCRS> GpOr<HCRS, ORS, S, SCRS>
where
  SCRS: AsRef<[usize]>,
{
  #[inline]
  pub fn soft_cstr_rslts(&self) -> &[usize] {
    self.soft_cstr_rslts.as_ref()
  }
}

impl<OR, S, SCR> GpOrRef<'_, usize, OR, S, SCR>
where
  OR: Clone,
  S: Clone,
{
  #[inline]
  pub fn to_mph_vec(&self) -> MphOrVec<OR, S> {
    GpOr {
      hard_cstr_rslts: self.hard_cstr_rslts.to_vec(),
      obj_rslts: self.obj_rslts.to_vec(),
      soft_cstr_rslts: Default::default(),
      solution: self.solution.clone(),
    }
  }
}

impl<OR, S> GpOrRef<'_, usize, OR, S, usize>
where
  OR: Clone,
  S: Clone,
{
  #[inline]
  pub fn to_mphs_vec(&self) -> MphsOrVec<OR, S> {
    GpOr {
      hard_cstr_rslts: self.hard_cstr_rslts.to_vec(),
      obj_rslts: self.obj_rslts.to_vec(),
      soft_cstr_rslts: self.soft_cstr_rslts.to_vec(),
      solution: self.solution.clone(),
    }
  }
}
