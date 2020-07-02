use crate::{
  dr_matrix::DrMatrixVec,
  gp::{GpDefinitions, GpOrMut, GpOrRef, GpOrs},
  Cstr, Obj,
};
use alloc::string::String;
use cl_traits::Storage;
use core::{fmt::Debug, marker::PhantomData};
#[cfg(feature = "with-futures")]
use futures::stream::{self, StreamExt};
use mop_common::TraitCfg;

#[derive(Debug)]
pub struct GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> {
  phantom: PhantomData<(D, HCRS, HCS, ORS, OS, SCRS, SCS, SS)>,
}

impl<D, HCRS, HCS, OR, ORS, OS, S, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  ORS: AsMut<[OR]> + Storage<Item = OR>,
  SS: AsMut<[S]> + Storage<Item = S>,
{
  pub async fn eval_hard_cstrs_violations<HC, SCR>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) where
    HC: Cstr<S>,
    HCRS: AsMut<[usize]> + Storage<Item = usize>,
    HCS: AsRef<[HC]> + Storage<Item = HC>,
    SCRS: AsMut<[SCR]> + Storage<Item = SCR>,
  {
    let hard_cstrs = defs.hard_cstrs();
    Self::eval_cstrs_rslts(hard_cstrs, rslts, |rslt| (rslt.hard_cstr_rslts, rslt.solution)).await
  }

  pub async fn eval_soft_cstrs_violations<HCR, SC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) where
    HCRS: AsMut<[HCR]> + Storage<Item = HCR>,
    SC: Cstr<S>,
    SCRS: AsMut<[usize]> + Storage<Item = usize>,
    SCS: AsRef<[SC]> + Storage<Item = SC>,
  {
    let soft_cstrs = defs.soft_cstrs();
    Self::eval_cstrs_rslts(soft_cstrs, rslts, |rslt| (rslt.soft_cstr_rslts, rslt.solution)).await
  }

  async fn eval_cstrs_rslts<C, HCR, SCR, F>(
    cstrs: &[C],
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
    cb: F,
  ) where
    C: Cstr<S>,
    F: Fn(GpOrMut<'_, HCR, OR, S, SCR>) -> (&mut [usize], &S),
    HCRS: AsMut<[HCR]> + Storage<Item = HCR>,
    SCRS: AsMut<[SCR]> + Storage<Item = SCR>,
  {
    let func = |rslt| {
      let (hard_cstr_rslts, solution) = cb(rslt);
      for (hard_cstr, hard_cstr_rslt) in cstrs.iter().zip(hard_cstr_rslts.iter_mut()) {
        *hard_cstr_rslt = hard_cstr.violations(solution);
      }
    };
    #[cfg(not(feature = "with-futures"))]
    rslts.iter_mut().for_each(func);
    #[cfg(feature = "with-futures")]
    stream::iter(rslts.iter_mut()).for_each_concurrent(None, |rslt| async { func(rslt) }).await;
  }
}

impl<D, HCR, HCRS, HCS, OR, ORS, OS, S, SCR, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  HCRS: AsRef<[HCR]> + Storage<Item = HCR>,
  ORS: AsRef<[OR]> + Storage<Item = OR>,
  SCRS: AsRef<[SCR]> + Storage<Item = SCR>,
  SS: AsRef<[S]> + Storage<Item = S>,
{
  pub async fn eval_hard_cstrs_reasons<HC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    HC: Cstr<S>,
    HCS: AsRef<[HC]> + Storage<Item = HC>,
  {
    let cols = rslts.hard_cstr_rslts.cols();
    let rows = rslts.hard_cstr_rslts.rows();
    Self::eval_cstrs_reasons(rows, cols, defs.hard_cstrs(), rslts).await
  }

  pub async fn eval_soft_cstrs_reasons<SC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    SC: Cstr<S>,
    SCS: AsRef<[SC]> + Storage<Item = SC>,
  {
    let cols = rslts.soft_cstr_rslts.cols();
    let rows = rslts.soft_cstr_rslts.rows();
    Self::eval_cstrs_reasons(rows, cols, defs.soft_cstrs(), rslts).await
  }

  async fn eval_cstrs_reasons<C>(
    rows: usize,
    cols: usize,
    cstrs: &[C],
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    C: Cstr<S>,
  {
    let mut reasons = DrMatrixVec::with_capacity(rows, cols);
    reasons.constructor().fill_rows(rows, String::with_capacity(256));
    let iter = reasons.row_iter_mut().zip(rslts.iter());
    let func = |(reasons, rslt): (&mut [String], GpOrRef<'_, _, _, _, _>)| {
      for (c, r) in cstrs.iter().zip(reasons.iter_mut()) {
        *r = c.reasons(rslt.solution);
      }
    };
    #[cfg(not(feature = "with-futures"))]
    iter.for_each(func);
    #[cfg(feature = "with-futures")]
    stream::iter(iter)
      .for_each_concurrent(None, |(reasons, rslt)| async move {
        func((reasons, rslt));
      })
      .await;
    reasons
  }
}

impl<D, HCR, HCRS, HCS, O, OR, ORS, OS, S, SCR, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  HCRS: AsMut<[HCR]> + Storage<Item = HCR>,
  O: Obj<OR, S> + TraitCfg,
  ORS: AsMut<[OR]> + Storage<Item = OR>,
  OS: AsRef<[O]> + Storage<Item = O>,
  SCRS: AsMut<[SCR]> + Storage<Item = SCR>,
  SS: AsMut<[S]> + Storage<Item = S>,
{
  pub async fn eval_objs(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) {
    let func = |r| Self::eval_objs_for_solution(defs.objs(), r);
    #[cfg(not(feature = "with-futures"))]
    rslts.iter_mut().for_each(func);
    #[cfg(feature = "with-futures")]
    stream::iter(rslts.iter_mut()).for_each_concurrent(None, |r| async { func(r) }).await;
  }

  fn eval_objs_for_solution(objs: &[O], rslt: GpOrMut<'_, HCR, OR, S, SCR>) {
    for (obj, obj_rslts) in objs.iter().zip(rslt.obj_rslts.iter_mut()) {
      let rslt = obj.result(&rslt.solution);
      *obj_rslts = rslt;
    }
  }
}
