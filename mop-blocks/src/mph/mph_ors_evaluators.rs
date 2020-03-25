use crate::dr_matrix::DrMatrixVec;
use crate::{
  mph::{MphDefinitions, MphOrMut, MphOrRef, MphOrs},
  Cstr, Obj,
};
use alloc::string::String;
use core::{fmt::Debug, marker::PhantomData, ops::Div};
use num_traits::{NumCast, Zero};
#[cfg(feature = "with_rayon")]
use rayon::prelude::*;

#[derive(Debug)]
pub struct MphOrsEvaluators<C, O, OR, S, SD> {
  phantom: PhantomData<(C, O, OR, S, SD)>,
}

impl<C, O, OR, S, SD> MphOrsEvaluators<C, O, OR, S, SD>
where
  O: Obj<OR, S>,
  OR: Copy + Debug + Div<OR, Output = OR> + NumCast + Zero,
{
  #[cfg(feature = "with_rayon")]
  pub fn eval_ors_cstrs(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>)
  where
    C: Cstr<S>,
    C: Send + Sync,
    O: Send + Sync,
    OR: Send + Sync,
    SD: Send + Sync,
    S: Send + Sync,
  {
    rslts.par_iter_mut().for_each(|mut x| Self::eval_or_cstrs(defs.hard_cstrs(), &mut x));
  }

  #[cfg(not(feature = "with_rayon"))]
  pub fn eval_ors_cstrs(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>)
  where
    C: Cstr<S>,
  {
    rslts.iter_mut().for_each(|mut x| Self::eval_or_cstrs(defs.hard_cstrs(), &mut x));
  }

  fn eval_or_cstrs(cstrs: &[C], rslt: &mut MphOrMut<'_, OR, S>)
  where
    C: Cstr<S>,
  {
    for (c, cr) in cstrs.iter().zip(rslt.hard_cstrs.iter_mut()) {
      *cr = c.violations(rslt.solution);
    }
  }

  #[cfg(feature = "with_rayon")]
  pub fn eval_ors_objs(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>)
  where
    C: Send + Sync,
    O: Send + Sync,
    OR: Send + Sync,
    SD: Send + Sync,
    S: Send + Sync,
  {
    rslts.par_iter_mut().for_each(|r| Self::eval_or_objs(defs.objs(), r));
  }

  #[cfg(not(feature = "with_rayon"))]
  pub fn eval_ors_objs(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>) {
    rslts.iter_mut().for_each(|r| Self::eval_or_objs(defs.objs(), r));
  }

  #[cfg(feature = "with_rayon")]
  pub fn cstrs_reasons(
    defs: &MphDefinitions<C, O, SD>,
    rslts: &MphOrs<OR, S>,
  ) -> DrMatrixVec<String>
  where
    C: Cstr<S>,
    C: Send + Sync,
    O: Send + Sync,
    OR: Send + Sync,
    SD: Send + Sync,
    S: Send + Sync,
  {
    let mut reasons = DrMatrixVec::with_capacity(rslts.hard_cstrs.rows(), rslts.hard_cstrs.cols());
    reasons.fill(|_, _| String::new());
    reasons.row_par_iter_mut().zip(rslts.par_iter()).for_each(|(rslt_reasons, rslt)| {
      Self::result_cstrs_reasons(defs.hard_cstrs(), rslt_reasons, &rslt)
    });
    reasons
  }

  #[cfg(not(feature = "with_rayon"))]
  pub fn cstrs_reasons(
    defs: &MphDefinitions<C, O, SD>,
    rslts: &MphOrs<OR, S>,
  ) -> DrMatrixVec<String>
  where
    C: Cstr<S>,
  {
    let mut reasons = DrMatrixVec::with_capacity(rslts.hard_cstrs.rows(), rslts.hard_cstrs.cols());
    reasons.fill(|_, _| String::with_capacity(256));
    reasons.row_iter_mut().zip(rslts.iter()).for_each(|(rslt_reasons, rslt)| {
      Self::result_cstrs_reasons(defs.hard_cstrs(), rslt_reasons, &rslt)
    });
    reasons
  }

  fn eval_or_objs(objs: &[O], rslt: MphOrMut<'_, OR, S>) {
    let mut objs_avg = OR::zero();
    for (obj, obj_rslts) in objs.iter().zip(rslt.objs.iter_mut()) {
      let rslt = obj.result(&rslt.solution);
      *obj_rslts = rslt;
      objs_avg = objs_avg + rslt;
    }
    let cast = NumCast::from(objs.len()).unwrap();
    *rslt.objs_avg = objs_avg / cast;
  }

  fn result_cstrs_reasons(cstrs: &[C], reasons: &mut [String], rslt: &MphOrRef<'_, OR, S>)
  where
    C: Cstr<S>,
  {
    for (c, r) in cstrs.iter().zip(reasons.iter_mut()) {
      *r = c.reasons(rslt.solution);
    }
  }
}
