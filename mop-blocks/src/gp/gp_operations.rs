pub(crate) mod mph_mp_mph;
pub(crate) mod mphs_mp_mphs;

use crate::{
  gp::{GpDefinitions, GpDefinitionsBuilder, GpOrs},
  Obj,
};
use cl_traits::Storage;
use mop_common::SolverFuture;

pub trait GpOperations<AD, AR, B> {
  type Error;

  fn convert(a: &AD) -> Result<B, Self::Error>;
  fn transfer<'a>(a_defs: &'a AD, a_rslts: &'a mut AR, b: &'a B) -> SolverFuture<'a, Self::Error>;
}

pub fn new_defsb_o_ref<'a, D, HCRS, HCS, NHCS, NOS, NSCS, O, OR, ORS, OS, S, SCRS, SCS, SS>(
  defs: &'a GpDefinitions<D, HCS, OS, SCS>,
  _: &GpOrs<HCRS, ORS, SCRS, SS>,
) -> GpDefinitionsBuilder<D, NHCS, NOS, NSCS>
where
  D: Clone,
  NHCS: Default,
  NOS: Default + Storage<Item = &'a dyn Obj<OR, S>>,
  NSCS: Default,
  OR: 'a,
  ORS: Storage<Item = OR>,
  OS: Storage<Item = O>,
  S: 'a,
  SS: Storage<Item = S>,
{
  GpDefinitionsBuilder {
    domain: Some(defs.domain.clone()),
    hard_cstrs: Some(Default::default()),
    soft_cstrs: Some(Default::default()),
    name: defs.name,
    objs: Some(Default::default()),
  }
}
