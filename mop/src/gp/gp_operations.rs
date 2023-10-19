pub(crate) mod mph_mp_mph;
pub(crate) mod mphs_mp_mphs;

use crate::{
  gp::{GpDefinitions, MpDefinitionsBuilder},
  Either, Obj,
};
use cl_aux::{Push, SingleTypeStorage};

pub trait GpOperations<AD, AR, B> {
  type Error;

  fn convert(a: &AD) -> Result<B, Self::Error>;

  fn transfer(a_defs: &AD, a_rslts: &mut AR, b: &B) -> Result<(), Self::Error>;
}

#[inline]
pub fn mp_defs_from_gp_defs<'any, D, HCS, NOS, O, OO, OR, OS, S, SCS>(
  defs: &'any GpDefinitions<D, HCS, OS, SCS>,
) -> crate::Result<MpDefinitionsBuilder<D, NOS>>
where
  D: Clone,
  NOS: Default + Push<Either<&'any O, OO>>,
  O: Obj<OR, S> + 'any,
  OR: 'any,
  OS: AsRef<[O]> + SingleTypeStorage<Item = O>,
  S: 'any,
{
  Ok(MpDefinitionsBuilder {
    domain: Some(defs.domain.clone()),
    hard_cstrs: Some(<_>::default()),
    soft_cstrs: Some(<_>::default()),
    name: defs.name,
    objs: {
      let mut objs: NOS = <_>::default();
      for obj in defs.objs() {
        objs.push(Either::Left(obj)).map_err(|_e| crate::Error::InsufficientCapacity)?;
      }
      Some(objs)
    },
  })
}
