use crate::{
  dr_matrix::DrMatrixVec,
  gp::{GpDefinitions, GpOrMut, GpOrRef, GpOrs},
  Cstr, Obj, ParBounds,
};
use alloc::string::String;
use cl_aux::SingleTypeStorage;
use core::{fmt::Debug, marker::PhantomData};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Debug)]
pub struct GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> {
  phantom: PhantomData<(D, HCRS, HCS, ORS, OS, SCRS, SCS, SS)>,
}

impl<D, HCRS, HCS, OR, ORS, OS, S, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  D: ParBounds,
  HCRS: ParBounds,
  HCS: ParBounds,
  OR: ParBounds,
  ORS: AsMut<[OR]> + ParBounds + SingleTypeStorage<Item = OR>,
  OS: ParBounds,
  S: ParBounds,
  SCRS: ParBounds,
  SCS: ParBounds,
  SS: AsMut<[S]> + ParBounds + SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn eval_hard_cstrs_violations<HC, SCR>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) where
    HC: Cstr<S> + ParBounds,
    HCRS: AsMut<[usize]> + SingleTypeStorage<Item = usize>,
    HCS: AsRef<[HC]> + SingleTypeStorage<Item = HC>,
    SCR: ParBounds,
    SCRS: AsMut<[SCR]> + SingleTypeStorage<Item = SCR>,
  {
    let hard_cstrs = defs.hard_cstrs();
    Self::eval_cstrs_rslts(hard_cstrs, rslts, |rslt| (rslt.hard_cstr_rslts, rslt.solution));
  }

  #[inline]
  pub fn eval_soft_cstrs_violations<HCR, SC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) where
    HCR: ParBounds,
    HCRS: AsMut<[HCR]> + SingleTypeStorage<Item = HCR>,
    SC: Cstr<S>,
    SCRS: AsMut<[usize]> + SingleTypeStorage<Item = usize>,
    SC: ParBounds,
    SCS: AsRef<[SC]> + SingleTypeStorage<Item = SC>,
  {
    let soft_cstrs = defs.soft_cstrs();
    Self::eval_cstrs_rslts(soft_cstrs, rslts, |rslt| (rslt.soft_cstr_rslts, rslt.solution));
  }

  #[inline]
  fn eval_cstrs_rslts<C, HCR, SCR, F>(cstrs: &[C], rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>, cb: F)
  where
    C: Cstr<S> + ParBounds,
    F: Fn(GpOrMut<'_, HCR, OR, S, SCR>) -> (&mut [usize], &S) + ParBounds,
    HCR: ParBounds,
    HCRS: AsMut<[HCR]> + SingleTypeStorage<Item = HCR>,
    SCR: ParBounds,
    SCRS: AsMut<[SCR]> + SingleTypeStorage<Item = SCR>,
  {
    let func = |rslt| {
      let (hard_cstr_rslts, solution) = cb(rslt);
      for (hard_cstr, hard_cstr_rslt) in cstrs.iter().zip(hard_cstr_rslts.iter_mut()) {
        *hard_cstr_rslt = hard_cstr.violations(solution);
      }
    };
    #[cfg(not(feature = "rayon"))]
    rslts.iter_mut().for_each(func);
    #[cfg(feature = "rayon")]
    rslts.par_iter_mut().for_each(func);
  }
}

impl<D, HCR, HCRS, HCS, OR, ORS, OS, S, SCR, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  D: ParBounds,
  HCR: ParBounds,
  HCRS: AsRef<[HCR]> + ParBounds + SingleTypeStorage<Item = HCR>,
  HCS: ParBounds,
  OR: ParBounds,
  ORS: AsRef<[OR]> + ParBounds + SingleTypeStorage<Item = OR>,
  OS: ParBounds,
  S: ParBounds,
  SCR: ParBounds,
  SCRS: AsRef<[SCR]> + ParBounds + SingleTypeStorage<Item = SCR>,
  SCS: ParBounds,
  SS: AsRef<[S]> + ParBounds + SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn eval_hard_cstrs_reasons<HC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    HC: Cstr<S> + ParBounds,
    HCS: AsRef<[HC]> + SingleTypeStorage<Item = HC>,
  {
    let cols = rslts.hard_cstr_rslts.cols();
    let rows = rslts.hard_cstr_rslts.rows();
    Self::eval_cstrs_reasons([rows, cols], defs.hard_cstrs(), rslts)
  }

  #[inline]
  pub fn eval_soft_cstrs_reasons<SC>(
    defs: &GpDefinitions<D, HCS, OS, SCS>,
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    SC: Cstr<S> + ParBounds,
    SCS: AsRef<[SC]> + SingleTypeStorage<Item = SC>,
  {
    let cols = rslts.soft_cstr_rslts.cols();
    let rows = rslts.soft_cstr_rslts.rows();
    Self::eval_cstrs_reasons([rows, cols], defs.soft_cstrs(), rslts)
  }

  #[inline]
  fn eval_cstrs_reasons<C>(
    [rows, cols]: [usize; 2],
    cstrs: &[C],
    rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>,
  ) -> DrMatrixVec<String>
  where
    C: Cstr<S> + ParBounds,
  {
    let mut reasons = DrMatrixVec::with_capacity(rows, cols);
    let _ = reasons.constructor().fill_rows(rows, String::with_capacity(256));
    let func = |(strings, rslt): (&mut [String], GpOrRef<'_, _, _, _, _>)| {
      for (c, r) in cstrs.iter().zip(strings.iter_mut()) {
        *r = c.reasons(rslt.solution);
      }
    };
    #[cfg(feature = "rayon")]
    reasons.row_par_iter_mut().zip_eq(rslts.par_iter()).for_each(func);
    #[cfg(not(feature = "rayon"))]
    reasons.row_iter_mut().zip(rslts.iter()).for_each(func);
    reasons
  }
}

impl<D, HCR, HCRS, HCS, O, OR, ORS, OS, S, SCR, SCRS, SCS, SS>
  GpOrsEvaluators<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  D: ParBounds,
  HCR: ParBounds,
  HCRS: AsMut<[HCR]> + ParBounds + SingleTypeStorage<Item = HCR>,
  HCS: ParBounds,
  O: Obj<OR, S> + ParBounds,
  OR: ParBounds,
  ORS: AsMut<[OR]> + ParBounds + SingleTypeStorage<Item = OR>,
  OS: AsRef<[O]> + ParBounds + SingleTypeStorage<Item = O>,
  S: ParBounds,
  SCR: ParBounds,
  SCRS: AsMut<[SCR]> + ParBounds + SingleTypeStorage<Item = SCR>,
  SCS: ParBounds,
  SS: AsMut<[S]> + ParBounds + SingleTypeStorage<Item = S>,
{
  #[inline]
  pub fn eval_objs(defs: &GpDefinitions<D, HCS, OS, SCS>, rslts: &mut GpOrs<HCRS, ORS, SCRS, SS>) {
    let func = |mut r| Self::eval_objs_for_solution(defs.objs(), &mut r);
    #[cfg(feature = "rayon")]
    rslts.par_iter_mut().for_each(func);
    #[cfg(not(feature = "rayon"))]
    rslts.iter_mut().for_each(func);
  }

  #[inline]
  fn eval_objs_for_solution(objs: &[O], rslt: &mut GpOrMut<'_, HCR, OR, S, SCR>) {
    let func = |(obj, obj_rslts): (&O, &mut OR)| {
      let or = obj.result(rslt.solution);
      *obj_rslts = or;
    };
    #[cfg(feature = "rayon")]
    objs.par_iter().zip_eq(rslt.obj_rslts.par_iter_mut()).for_each(func);
    #[cfg(not(feature = "rayon"))]
    objs.iter().zip(rslt.obj_rslts.iter_mut()).for_each(func);
  }
}
