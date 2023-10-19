use crate::gp::{GpDefinitions, NoCstr, OneObj};
use alloc::vec::Vec;
use cl_aux::{Push, SingleTypeStorage};
use core::fmt;

pub type MpDefinitionsBuilder<D, OS> = GpDefinitionsBuilder<D, NoCstr, OS, NoCstr>;
pub type MphDefinitionsBuilder<D, HCS, OS> = GpDefinitionsBuilder<D, HCS, OS, NoCstr>;
pub type MphsDefinitionsBuilder<D, HCS, OS, SCS> = GpDefinitionsBuilder<D, HCS, OS, SCS>;
pub type SpDefinitionsBuilder<D, O> = GpDefinitionsBuilder<D, NoCstr, OneObj<O>, NoCstr>;

pub type MpDefinitionsBuilderVec<D, O> = MpDefinitionsBuilder<D, Vec<O>>;
pub type MphDefinitionsBuilderVec<D, HC, O> = MphDefinitionsBuilder<D, Vec<HC>, Vec<O>>;
pub type MphsDefinitionsBuilderVec<D, HC, O, SC> =
  MphsDefinitionsBuilder<D, Vec<HC>, Vec<O>, Vec<SC>>;
pub type SpDefinitionsBuilderVec<D, O> = SpDefinitionsBuilder<D, OneObj<O>>;

/// Definitions Builder for GP.
///
/// This structure is generic over single or multi objective problems, constrained or not.
///
/// # Types
///
/// * `D`: Domain
/// * `HCS`: Hard Constraints Storage
/// * `OR`: Objective Result
/// * `OS`: Objectives Storage
/// * `S`: Solution
/// * `SCS`: Soft Constraints Storage
#[derive(Clone, Debug)]
pub struct GpDefinitionsBuilder<D, HCS, OS, SCS> {
  pub(crate) domain: Option<D>,
  pub(crate) hard_cstrs: Option<HCS>,
  pub(crate) name: &'static str,
  pub(crate) objs: Option<OS>,
  pub(crate) soft_cstrs: Option<SCS>,
}

impl<D, HCS, OS, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS> {
  #[inline]
  pub fn build(self) -> crate::Result<GpDefinitions<D, HCS, OS, SCS>>
  where
    HCS: Default,
    SCS: Default,
  {
    let Some(domain) = self.domain else {
      return Err(GpDefinitionsBuilderError::NoDomainForDefinitionsBuilder.into());
    };
    let Some(objs) = self.objs else {
      return Err(GpDefinitionsBuilderError::NoObjForDefinitionsBuilder.into());
    };
    Ok(GpDefinitions {
      domain,
      hard_cstrs: self.hard_cstrs.unwrap_or_default(),
      name: self.name,
      soft_cstrs: self.soft_cstrs.unwrap_or_default(),
      objs,
    })
  }

  #[inline]
  #[must_use]
  pub fn domain(mut self, domain: D) -> Self {
    self.domain = Some(domain);
    self
  }

  #[inline]
  #[must_use]
  pub fn name(mut self, name: &'static str) -> Self {
    self.name = name;
    self
  }
}

impl<D, HCS, OS, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS> {
  /// Hard constraints
  #[inline]
  #[must_use]
  pub fn hard_cstrs(mut self, hard_cstrs: HCS) -> Self {
    self.hard_cstrs = Some(hard_cstrs);
    self
  }
}

impl<D, HC, HCS, OS, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS>
where
  HCS: SingleTypeStorage<Item = HC>,
{
  /// Push hard constraint
  #[inline]
  pub fn push_hard_cstr(mut self, hard_cstr: HC) -> crate::Result<Self>
  where
    HCS: Default + Push<HC>,
  {
    if let Some(hcs) = self.hard_cstrs.as_mut() {
      hcs.push(hard_cstr).map_err(|_e| crate::Error::InsufficientCapacity)?;
    } else {
      let mut hard_cstrs = HCS::default();
      hard_cstrs.push(hard_cstr).map_err(|_e| crate::Error::InsufficientCapacity)?;
      self.hard_cstrs = Some(hard_cstrs);
    }
    Ok(self)
  }

  /// Push objectives
  #[inline]
  pub fn push_hard_cstrs<CI>(self, hard_cstrs: CI) -> crate::Result<Self>
  where
    CI: IntoIterator<Item = HC>,
    HCS: Default + Push<HC>,
  {
    hard_cstrs.into_iter().try_fold(self, GpDefinitionsBuilder::push_hard_cstr)
  }
}

impl<D, HC, O, SC> GpDefinitionsBuilder<D, HC, [O; 1], SC> {
  /// Objective
  #[inline]
  #[must_use]
  pub fn obj(mut self, obj: O) -> Self {
    self.objs = Some([obj]);
    self
  }
}

impl<D, HCS, O, OS, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS>
where
  OS: SingleTypeStorage<Item = O>,
{
  /// Objectives
  #[inline]
  #[must_use]
  pub fn objs(mut self, objs: OS) -> Self {
    self.objs = Some(objs);
    self
  }

  /// Push objective
  #[inline]
  pub fn push_obj(mut self, obj: O) -> crate::Result<Self>
  where
    OS: Default + Push<O>,
  {
    if let Some(objs) = self.objs.as_mut() {
      objs.push(obj).map_err(|_e| crate::Error::InsufficientCapacity)?;
    } else {
      let mut objs = OS::default();
      objs.push(obj).map_err(|_e| crate::Error::InsufficientCapacity)?;
      self.objs = Some(objs);
    }
    Ok(self)
  }

  /// Push objectives
  #[inline]
  pub fn push_objs<OI>(self, objs: OI) -> crate::Result<Self>
  where
    OI: IntoIterator<Item = O>,
    OS: Default + Push<O>,
  {
    objs.into_iter().try_fold(self, GpDefinitionsBuilder::push_obj)
  }
}

impl<D, HCS, OS, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS> {
  /// Soft constraints
  #[inline]
  #[must_use]
  pub fn soft_cstrs(mut self, soft_cstrs: SCS) -> Self {
    self.soft_cstrs = Some(soft_cstrs);
    self
  }
}

impl<D, HCS, OS, SC, SCS> GpDefinitionsBuilder<D, HCS, OS, SCS>
where
  SCS: SingleTypeStorage<Item = SC>,
{
  /// Push soft constraint
  #[inline]
  pub fn push_soft_cstr(mut self, soft_cstr: SC) -> crate::Result<Self>
  where
    SCS: Default + Push<SC>,
  {
    if let Some(hcs) = self.soft_cstrs.as_mut() {
      hcs.push(soft_cstr).map_err(|_e| crate::Error::InsufficientCapacity)?;
    } else {
      let mut soft_cstrs = SCS::default();
      soft_cstrs.push(soft_cstr).map_err(|_e| crate::Error::InsufficientCapacity)?;
      self.soft_cstrs = Some(soft_cstrs);
    }
    Ok(self)
  }

  /// Push soft constraints
  #[inline]
  pub fn push_soft_cstrs<CI>(self, soft_cstrs: CI) -> crate::Result<Self>
  where
    CI: IntoIterator<Item = SC>,
    SCS: Default + Push<SC>,
  {
    soft_cstrs.into_iter().try_fold(self, GpDefinitionsBuilder::push_soft_cstr)
  }
}

impl<D, HCS, OS, SCS> Default for GpDefinitionsBuilder<D, HCS, OS, SCS> {
  #[inline]
  fn default() -> Self {
    Self { domain: None, hard_cstrs: None, name: "Unknown problem", objs: None, soft_cstrs: None }
  }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum GpDefinitionsBuilderError {
  /// A domain must be included
  NoDomainForDefinitionsBuilder,
  /// Must have at least one objective
  NoObjForDefinitionsBuilder,
}

#[cfg(feature = "std")]
impl std::error::Error for GpDefinitionsBuilderError {}

impl fmt::Display for GpDefinitionsBuilderError {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match *self {
      Self::NoDomainForDefinitionsBuilder => "NoDomainForDefinitionsBuilder",
      Self::NoObjForDefinitionsBuilder => "NoObjForDefinitionsBuilder",
    };
    write!(f, "{s}")
  }
}
