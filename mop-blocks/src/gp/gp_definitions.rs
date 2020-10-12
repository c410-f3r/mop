use crate::gp::{GpDefinitionsBuilder, NoCstr, OneObj};
use alloc::vec::Vec;
use cl_traits::Storage;

pub type MpDefinitions<D, OS> = GpDefinitions<D, NoCstr, OS, NoCstr>;
pub type MphDefinitions<D, HCS, OS> = GpDefinitions<D, HCS, OS, NoCstr>;
pub type MphsDefinitions<D, HCS, OS, SCS> = GpDefinitions<D, HCS, OS, SCS>;
pub type SpDefinitions<D, O> = GpDefinitions<D, NoCstr, OneObj<O>, NoCstr>;

pub type MpDefinitionsVec<D, O> = MpDefinitions<D, Vec<O>>;
pub type MphDefinitionsVec<D, HC, O> = MphDefinitions<D, Vec<HC>, Vec<O>>;
pub type MphsDefinitionsVec<D, HC, O, SC> = MphsDefinitions<D, Vec<HC>, Vec<O>, Vec<SC>>;
pub type SpDefinitionsVec<D, O> = SpDefinitions<D, OneObj<O>>;

/// Definitions for GP.
///
/// This structure is generic over single or multi objective problems, constrained or not.
///
/// # Types
///
/// * `D`: Solution Domain
/// * `HCS`: Hard Constraints Storage
/// * `OR`: Objective Result
/// * `OS`: Objectives Storage
/// * `S`: Solution
/// * `SCS`: Soft Constraints Storage
#[derive(Clone, Debug)]
pub struct GpDefinitions<D, HCS, OS, SCS> {
  pub(crate) domain: D,
  pub(crate) hard_cstrs: HCS,
  pub(crate) name: &'static str,
  pub(crate) objs: OS,
  pub(crate) soft_cstrs: SCS,
}

impl<D, HCS, OS, SCS> GpDefinitions<D, HCS, OS, SCS> {
  #[inline]
  pub fn into_builder(self) -> GpDefinitionsBuilder<D, HCS, OS, SCS> {
    GpDefinitionsBuilder {
      domain: Some(self.domain),
      hard_cstrs: Some(self.hard_cstrs),
      name: self.name,
      soft_cstrs: Some(self.soft_cstrs),
      objs: Some(self.objs),
    }
  }

  #[inline]
  pub fn domain(&self) -> &D {
    &self.domain
  }

  #[inline]
  pub fn name(&self) -> &str {
    self.name
  }
}

impl<D, HC, HCS, OS, SCS> GpDefinitions<D, HCS, OS, SCS>
where
  HCS: AsRef<[HC]> + Storage<Item = HC>,
{
  /// Hard constraints
  #[inline]
  pub fn hard_cstrs(&self) -> &[HC] {
    self.hard_cstrs.as_ref()
  }
}

impl<D, HCS, O, SCS> GpDefinitions<D, HCS, [O; 1], SCS> {
  /// Objective
  #[inline]
  pub fn obj(&self) -> &O {
    &self.objs[0]
  }
}

impl<D, HCS, O, OS, SCS> GpDefinitions<D, HCS, OS, SCS>
where
  OS: AsRef<[O]> + Storage<Item = O>,
{
  /// Objectives
  #[inline]
  pub fn objs(&self) -> &[O] {
    self.objs.as_ref()
  }
}

impl<D, HCS, OS, SC, SCS> GpDefinitions<D, HCS, OS, SCS>
where
  SCS: AsRef<[SC]> + Storage<Item = SC>,
{
  /// Soft constraints
  #[inline]
  pub fn soft_cstrs(&self) -> &[SC] {
    self.soft_cstrs.as_ref()
  }
}

impl<D, HCS, OS, SCS> Default for GpDefinitions<D, HCS, OS, SCS>
where
  D: Default,
  HCS: Default,
  SCS: Default,
  OS: Default,
{
  #[inline]
  fn default() -> Self {
    Self {
      domain: D::default(),
      hard_cstrs: Default::default(),
      name: Default::default(),
      objs: OS::default(),
      soft_cstrs: Default::default(),
    }
  }
}
