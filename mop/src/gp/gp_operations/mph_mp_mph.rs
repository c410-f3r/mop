use crate::{
  gp::{GpOperations, GpOrsEvaluators, Mp, MphDefinitions, MphOrs},
  Cstr, ParBounds,
};
use cl_aux::{Length, Push, SingleTypeStorage};

/// Mph -> Mp -> Mph
#[derive(Debug)]
pub struct MphMpMph;

impl<AORS, AOS, ASS, BORS, BOS, D, HC, HCRS, HCS, OR, S, SS>
  GpOperations<MphDefinitions<D, HCS, AOS>, MphOrs<HCRS, AORS, ASS>, Mp<D, BORS, BOS, SS>>
  for MphMpMph
where
  AOS: ParBounds,
  AORS: AsMut<[OR]> + Extend<OR> + Length + ParBounds + SingleTypeStorage<Item = OR>,
  ASS: AsMut<[S]> + ParBounds + Push<S> + SingleTypeStorage<Item = S>,
  BORS: AsRef<[OR]> + Length + SingleTypeStorage<Item = OR>,
  D: ParBounds,
  HC: Cstr<S> + ParBounds,
  HCRS: AsMut<[usize]> + Extend<usize> + Length + ParBounds + SingleTypeStorage<Item = usize>,
  HCS: AsRef<[HC]> + ParBounds + SingleTypeStorage<Item = HC>,
  OR: Clone + Default + ParBounds,
  S: Clone + ParBounds,
  SS: AsRef<[S]> + SingleTypeStorage<Item = S>,
{
  type Error = crate::Error;

  #[inline]
  fn convert(_: &MphDefinitions<D, HCS, AOS>) -> Result<Mp<D, BORS, BOS, SS>, Self::Error> {
    Err(Self::Error::UnsupportedConversion)
  }

  #[inline]
  fn transfer(
    a_defs: &MphDefinitions<D, HCS, AOS>,
    a_rslts: &mut MphOrs<HCRS, AORS, ASS>,
    b: &Mp<D, BORS, BOS, SS>,
  ) -> Result<(), Self::Error> {
    let mut c = a_rslts.constructor();
    for rslt in b.rslts().iter() {
      let ori = rslt.obj_rslts().iter().cloned().rev().skip(1).rev();
      let s = (*rslt.solution()).clone();
      let _ = c.or_os_iter(ori, s);
    }
    GpOrsEvaluators::eval_hard_cstrs_violations(a_defs, a_rslts);
    Ok(())
  }
}
