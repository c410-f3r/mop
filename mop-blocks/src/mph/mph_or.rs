//! Orho (*Optimization* *R*esult with *H*ard constraints, and *O*bjectives)

use alloc::vec::Vec;
use cl_traits::Storage;

/// MPH-OR (Multi-objective Problem with Hard constraints - Optimization Result)
///
/// # Types
///
/// * `CRS`: Constraint Result Storage
/// * `OR`: Objective Result
/// * `ORS`: Objective Result Storage
/// * `S`: Solution
#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct MphOr<CRS, OR, ORS, S> {
  pub(crate) hard_cstrs: CRS,
  pub(crate) objs: ORS,
  pub(crate) objs_avg: OR,
  pub(crate) solution: S,
}

pub type MphOrMut<'a, OR, S> = MphOr<&'a mut [usize], &'a mut OR, &'a mut [OR], &'a mut S>;
pub type MphOrRef<'a, OR, S> = MphOr<&'a [usize], &'a OR, &'a [OR], &'a S>;
pub type MphOrVec<OR, S> = MphOr<Vec<usize>, OR, Vec<OR>, S>;

impl<CRS, OR, ORS, S> MphOr<CRS, OR, ORS, S> {
  pub fn objs_avg(&self) -> &OR {
    &self.objs_avg
  }

  pub fn solution(&self) -> &S {
    &self.solution
  }

  pub fn solution_mut(&mut self) -> &mut S {
    &mut self.solution
  }
}

impl<CRS, OR, ORS, S> MphOr<CRS, OR, ORS, S>
where
  CRS: AsRef<[usize]>,
{
  pub fn hard_cstrs(&self) -> &[usize] {
    self.hard_cstrs.as_ref()
  }
}

impl<CRS, OR, ORS, S> MphOr<CRS, OR, ORS, S>
where
  CRS: AsMut<[usize]>,
{
  pub fn hard_cstrs_mut(&mut self) -> &mut [usize] {
    self.hard_cstrs.as_mut()
  }
}

impl<CRS, OR, ORS, S> MphOr<CRS, OR, ORS, S>
where
  ORS: AsRef<[<ORS as Storage>::Item]> + Storage,
{
  pub fn objs(&self) -> &[ORS::Item] {
    self.objs.as_ref()
  }
}

impl<OR, S> MphOrRef<'_, OR, S> {
  pub fn to_vec(&self) -> MphOrVec<OR, S>
  where
    OR: Clone,
    S: Clone,
  {
    MphOrVec {
      hard_cstrs: self.hard_cstrs().to_vec(),
      objs: self.objs().to_vec(),
      objs_avg: self.objs_avg.clone(),
      solution: self.solution.clone(),
    }
  }
}
