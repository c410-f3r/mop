use crate::{
  gp::{GpOperations, GpOrsEvaluators, Mp, MphsDefinitions, MphsOrs},
  Cstr,
};
use cl_traits::{Length, Push, Storage};
use mop_common::{SolverFuture, TraitCfg};

/// Mphs -> Mp -> Mphs
#[derive(Debug)]
pub struct MphsMpMphs;

impl<AORS, AOS, ASS, BORS, BOS, D, HC, HCRS, HCS, OR, S, SCRS, SCS, SS>
  GpOperations<
    MphsDefinitions<D, HCS, AOS, SCS>,
    MphsOrs<HCRS, AORS, SCRS, ASS>,
    Mp<D, BORS, BOS, SS>,
  > for MphsMpMphs
where
  AORS: AsMut<[OR]> + Extend<OR> + Length + Storage<Item = OR> + TraitCfg,
  AOS: TraitCfg,
  ASS: AsMut<[S]> + Push<Input = S> + Storage<Item = S> + TraitCfg,
  BORS: AsRef<[OR]> + Length + Storage<Item = OR> + TraitCfg,
  BOS: TraitCfg,
  D: TraitCfg,
  HC: Cstr<S> + TraitCfg,
  HCRS: AsMut<[usize]> + Extend<usize> + Length + Storage<Item = usize> + TraitCfg,
  HCS: AsRef<[HC]> + Storage<Item = HC> + TraitCfg,
  OR: Clone + Default + TraitCfg,
  S: Clone + TraitCfg,
  SCRS: AsMut<[usize]> + Extend<usize> + Length + Storage<Item = usize> + TraitCfg,
  SCS: AsRef<[HC]> + Storage<Item = HC> + TraitCfg,
  SS: AsRef<[S]> + Storage<Item = S> + TraitCfg,
{
  type Error = crate::Error;

  #[inline]
  fn convert(_: &MphsDefinitions<D, HCS, AOS, SCS>) -> Result<Mp<D, BORS, BOS, SS>, Self::Error> {
    Err(Self::Error::UnsupportedConversion)
  }

  #[inline]
  fn transfer<'a>(
    a_defs: &'a MphsDefinitions<D, HCS, AOS, SCS>,
    a_rslts: &'a mut MphsOrs<HCRS, AORS, SCRS, ASS>,
    b: &'a Mp<D, BORS, BOS, SS>,
  ) -> SolverFuture<'a, Self::Error> {
    alloc::boxed::Box::pin(async move {
      let mut c = a_rslts.constructor();
      for rslt in b.rslts().iter() {
        let ori = rslt.obj_rslts().iter().cloned().rev().skip(1).rev();
        let s = (*rslt.solution()).clone();
        c = c.or_os_iter(ori, s);
      }
      GpOrsEvaluators::eval_hard_cstrs_violations(a_defs, a_rslts).await;
      GpOrsEvaluators::eval_soft_cstrs_violations(a_defs, a_rslts).await;
      Ok(())
    })
  }
}
