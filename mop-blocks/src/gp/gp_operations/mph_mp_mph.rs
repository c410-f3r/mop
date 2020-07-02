use crate::{
  gp::{GpOperations, GpOrsEvaluators, Mp, MphDefinitions, MphOrs},
  Cstr,
};
use cl_traits::{Length, Push, Storage};
use mop_common::{SolverFuture, TraitCfg};

/// Mph -> Mp -> Mph
#[derive(Debug)]
pub struct MphMpMph;

impl<AORS, AOS, ASS, BORS, BOS, D, HC, HCRS, HCS, OR, S, SS>
  GpOperations<MphDefinitions<D, HCS, AOS>, MphOrs<HCRS, AORS, ASS>, Mp<D, BORS, BOS, SS>>
  for MphMpMph
where
  AORS: AsMut<[OR]> + Extend<OR> + Length<Output = usize> + Storage<Item = OR> + TraitCfg,
  AOS: TraitCfg,
  ASS: AsMut<[S]> + Push<Input = S> + Storage<Item = S> + TraitCfg,
  BORS: AsRef<[OR]> + Length<Output = usize> + Storage<Item = OR> + TraitCfg,
  BOS: TraitCfg,
  D: TraitCfg,
  HC: Cstr<S> + TraitCfg,
  HCRS: AsMut<[usize]> + Extend<usize> + Length<Output = usize> + Storage<Item = usize> + TraitCfg,
  HCS: AsRef<[HC]> + Storage<Item = HC> + TraitCfg,
  OR: Clone + Default + TraitCfg,
  S: Clone + TraitCfg,
  SS: AsRef<[S]> + Storage<Item = S> + TraitCfg,
{
  type Error = crate::Error;

  fn convert(_: &MphDefinitions<D, HCS, AOS>) -> Result<Mp<D, BORS, BOS, SS>, Self::Error> {
    Err(Self::Error::Other("Unsupported conversion"))
  }

  fn transfer<'a>(
    a_defs: &'a MphDefinitions<D, HCS, AOS>,
    a_rslts: &'a mut MphOrs<HCRS, AORS, ASS>,
    b: &'a Mp<D, BORS, BOS, SS>,
  ) -> SolverFuture<'a, Self::Error> {
    alloc::boxed::Box::pin(async move {
      let mut c = a_rslts.constructor();
      for rslt in b.rslts().iter() {
        let ori = rslt.obj_rslts().iter().cloned().rev().skip(1).rev();
        let s = (*rslt.solution()).clone();
        c = crate::Error::opt_rslt(c.or_os_iter(ori, s))?;
      }
      GpOrsEvaluators::eval_hard_cstrs_violations(a_defs, a_rslts).await;
      Ok(())
    })
  }
}
