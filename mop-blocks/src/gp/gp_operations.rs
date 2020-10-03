pub(crate) mod mph_mp_mph;
pub(crate) mod mphs_mp_mphs;

use crate::{
  gp::{GpDefinitions, MpDefinitionsBuilder},
  Obj,
};
use cl_traits::{Push, Storage};
use mop_common::SolverFuture;

pub trait GpOperations<AD, AR, B> {
  type Error;

  fn convert(a: &AD) -> Result<B, Self::Error>;
  fn transfer<'a>(a_defs: &'a AD, a_rslts: &'a mut AR, b: &'a B) -> SolverFuture<'a, Self::Error>;
}

pub fn mp_defs_from_gp_defs<'a, D, HCS, NOS, O, OR, OS, S, SCS>(
  defs: &'a GpDefinitions<D, HCS, OS, SCS>,
) -> MpDefinitionsBuilder<D, NOS>
where
  D: Clone,
  NOS: Default + Push<Input = &'a dyn Obj<OR, S>> + Storage<Item = &'a dyn Obj<OR, S>>,
  O: Obj<OR, S> + 'a,
  OR: 'a,
  OS: AsRef<[O]> + Storage<Item = O>,
  S: 'a,
{
  MpDefinitionsBuilder {
    domain: Some(defs.domain.clone()),
    hard_cstrs: Some(Default::default()),
    soft_cstrs: Some(Default::default()),
    name: defs.name,
    objs: {
      let mut objs: NOS = Default::default();
      for obj in defs.objs() {
        objs.push(obj);
      }
      Some(objs)
    },
  }
}
