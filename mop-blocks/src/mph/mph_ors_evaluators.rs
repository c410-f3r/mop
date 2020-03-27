use crate::dr_matrix::DrMatrixVec;
use crate::{
  mph::{MphDefinitions, MphOrMut, MphOrRef, MphOrs},
  Cstr, Obj,
};
use alloc::string::String;
use core::{fmt::Debug, marker::PhantomData, ops::Div};
#[cfg(feature = "with_futures")]
use futures::stream::{self, StreamExt};
use mop_common_defs::TraitCfg;
use num_traits::{NumCast, Zero};

#[derive(Debug)]
pub struct MphOrsEvaluators<C, O, OR, S, SD> {
  phantom: PhantomData<(C, O, OR, S, SD)>,
}

impl<C, O, OR, S, SD> MphOrsEvaluators<C, O, OR, S, SD>
where
  C: Cstr<S> + TraitCfg,
  O: Obj<OR, S> + TraitCfg,
  OR: Copy + Debug + Div<OR, Output = OR> + NumCast + TraitCfg + Zero,
{
  pub async fn eval_cstrs_reasons(
    defs: &MphDefinitions<C, O, SD>,
    rslts: &MphOrs<OR, S>,
  ) -> DrMatrixVec<String> {
    let func = |(reasons, r)| Self::eval_cstrs_reasons_for_solution(defs.hard_cstrs(), reasons, &r);
    let mut reasons = DrMatrixVec::with_capacity(rslts.hard_cstrs.rows(), rslts.hard_cstrs.cols());
    reasons.fill(|_, _| String::with_capacity(256));
    let iter = reasons.row_iter_mut().zip(rslts.iter());
    #[cfg(not(feature = "with_futures"))]
    iter.for_each(func);
    #[cfg(feature = "with_futures")]
    stream::iter(iter)
      .for_each_concurrent(None, |(reasons, r)| async move { func((reasons, r)) })
      .await;
    reasons
  }

  pub async fn eval_cstrs_violations(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>) {
    let func = |r| Self::eval_cstrs_violations_for_solution(defs.hard_cstrs(), r);
    #[cfg(not(feature = "with_futures"))]
    rslts.iter_mut().for_each(func);
    #[cfg(feature = "with_futures")]
    stream::iter(rslts.iter_mut()).for_each_concurrent(None, |r| async { func(r) }).await;
  }

  pub async fn eval_objs(defs: &MphDefinitions<C, O, SD>, rslts: &mut MphOrs<OR, S>) {
    let func = |r| Self::eval_objs_for_solution(defs.objs(), r);
    #[cfg(not(feature = "with_futures"))]
    rslts.iter_mut().for_each(func);
    #[cfg(feature = "with_futures")]
    stream::iter(rslts.iter_mut()).for_each_concurrent(None, |r| async { func(r) }).await;
  }

  fn eval_cstrs_reasons_for_solution(
    cstrs: &[C],
    reasons: &mut [String],
    rslt: &MphOrRef<'_, OR, S>,
  ) {
    for (c, r) in cstrs.iter().zip(reasons.iter_mut()) {
      *r = c.reasons(rslt.solution);
    }
  }

  fn eval_cstrs_violations_for_solution(cstrs: &[C], rslt: MphOrMut<'_, OR, S>) {
    for (c, cr) in cstrs.iter().zip(rslt.hard_cstrs.iter_mut()) {
      *cr = c.violations(rslt.solution);
    }
  }

  fn eval_objs_for_solution(objs: &[O], rslt: MphOrMut<'_, OR, S>) {
    let mut objs_avg = OR::zero();
    for (obj, obj_rslts) in objs.iter().zip(rslt.objs.iter_mut()) {
      let rslt = obj.result(&rslt.solution);
      *obj_rslts = rslt;
      objs_avg = objs_avg + rslt;
    }
    let cast = NumCast::from(objs.len()).unwrap();
    *rslt.objs_avg = objs_avg / cast;
  }
}
