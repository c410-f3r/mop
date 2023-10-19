use crate::{
  gp::{GpOperations, GpOrsEvaluators, Mp, MphsDefinitions, MphsOrs},
  Cstr, ParBounds,
};
use cl_aux::{Length, Push, SingleTypeStorage};

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
  AORS: AsMut<[OR]> + Extend<OR> + Length + ParBounds + SingleTypeStorage<Item = OR>,
  AOS: ParBounds,
  ASS: AsMut<[S]> + ParBounds + Push<S> + SingleTypeStorage<Item = S>,
  BORS: AsRef<[OR]> + Length + SingleTypeStorage<Item = OR>,
  D: ParBounds,
  HC: Cstr<S> + ParBounds,
  HCRS: AsMut<[usize]> + Extend<usize> + Length + ParBounds + SingleTypeStorage<Item = usize>,
  HCS: AsRef<[HC]> + ParBounds + SingleTypeStorage<Item = HC>,
  OR: Clone + Default + ParBounds,
  S: Clone + ParBounds,
  SCRS: AsMut<[usize]> + Extend<usize> + Length + ParBounds + SingleTypeStorage<Item = usize>,
  SCS: AsRef<[HC]> + ParBounds + SingleTypeStorage<Item = HC>,
  SS: AsRef<[S]> + SingleTypeStorage<Item = S>,
{
  type Error = crate::Error;

  #[inline]
  fn convert(_: &MphsDefinitions<D, HCS, AOS, SCS>) -> Result<Mp<D, BORS, BOS, SS>, Self::Error> {
    Err(Self::Error::UnsupportedConversion)
  }

  #[inline]
  fn transfer(
    a_defs: &MphsDefinitions<D, HCS, AOS, SCS>,
    a_rslts: &mut MphsOrs<HCRS, AORS, SCRS, ASS>,
    b: &Mp<D, BORS, BOS, SS>,
  ) -> Result<(), Self::Error> {
    let mut c = a_rslts.constructor();
    for rslt in b.rslts().iter() {
      let ori = rslt.obj_rslts().iter().cloned().rev().skip(1).rev();
      let s = (*rslt.solution()).clone();
      let _ = c.or_os_iter(ori, s);
    }
    GpOrsEvaluators::eval_hard_cstrs_violations(a_defs, a_rslts);
    GpOrsEvaluators::eval_soft_cstrs_violations(a_defs, a_rslts);
    Ok(())
  }
}
