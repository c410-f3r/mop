use crate::{
  gp::{
    mp_defs_from_gp_defs, GpOperations, MpVec, MphDefinitionsBuilderVec, MphDefinitionsVec,
    MphMpMph, MphOrVec, MphOrsVec, MphVec, NoCstr, NoCstrRslts,
  },
  objs::MinCstrsRslts,
  opt,
  quality_comparator::ObjsAvg,
  solvers::genetic_algorithm::{
    operators::{
      crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
    },
    GeneticAlgorithmParamsBuilder, Spea2,
  },
  Cstr, Either,
};
use alloc::{format, vec::Vec};
use arrayvec::ArrayVec;
use core::ops::RangeInclusive;
use js_sys::{Array, Function};
use rand::Rng;
use wasm_bindgen::prelude::*;

// HardCstr

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct HardCstr(Function);

#[wasm_bindgen]
impl HardCstr {
  #[wasm_bindgen(constructor)]
  pub fn new(f: Function) -> Self {
    HardCstr(f)
  }
}

impl Cstr<Solution> for HardCstr {
  fn violations(&self, solution: &Solution) -> usize {
    let array: Array = Array::new();
    solution.0.iter().for_each(|&x| {
      let jv = JsValue::from_f64(x);
      let _ = array.push(&jv);
    });
    let fn_ret_rslt = self.0.call1(&JsValue::NULL, &JsValue::from(array));
    let fn_ret = if let Ok(rslt) = fn_ret_rslt {
      rslt.as_f64()
    } else {
      return usize::MAX;
    };
    #[allow(
      // There is no `as_usize`
      clippy::as_conversions
    )]
    if let Some(rslt) = fn_ret {
      rslt as usize
    } else {
      usize::MAX
    }
  }
}

// Obj

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Obj(ObjDirection, Function);

#[wasm_bindgen]
impl Obj {
  #[wasm_bindgen(constructor)]
  pub fn new(od: ObjDirection, f: Function) -> Self {
    Obj(od, f)
  }
}

impl crate::Obj<f64, Solution> for Obj {
  fn obj_direction(&self) -> crate::ObjDirection {
    self.0.to_original()
  }

  fn result(&self, solution: &Solution) -> f64 {
    let array: Array = Array::new();
    solution.0.iter().for_each(|&x| {
      let jv = JsValue::from_f64(x);
      let _ = array.push(&jv);
    });
    let fn_ret_rslt = self.1.call1(&JsValue::NULL, &JsValue::from(array));
    let fn_ret = if let Ok(rslt) = fn_ret_rslt {
      rslt.as_f64()
    } else {
      return f64::MAX;
    };
    if let Some(rslt) = fn_ret {
      rslt
    } else {
      f64::MAX
    }
  }
}

// ObjDirection

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum ObjDirection {
  Max,
  Min,
}

impl ObjDirection {
  fn to_original(&self) -> crate::ObjDirection {
    match *self {
      ObjDirection::Max => crate::ObjDirection::Max,
      ObjDirection::Min => crate::ObjDirection::Min,
    }
  }
}

// OptFacade

#[wasm_bindgen]
#[derive(Debug)]
pub struct OptFacade(
  opt::OptFacade<
    Domain,
    NoCstrRslts,
    NoCstr,
    (),
    f64,
    Vec<f64>,
    Vec<Obj>,
    ObjsAvg,
    NoCstrRslts,
    NoCstr,
    Vec<Solution>,
  >,
);

#[wasm_bindgen]
impl OptFacade {
  #[wasm_bindgen(constructor)]
  pub fn new(max_iterations: usize) -> Self {
    OptFacade(opt::OptFacade::new(max_iterations))
  }

  pub fn set_max_iterations(self, max_iterations: usize) -> Self {
    OptFacade(self.0.set_max_iterations(max_iterations))
  }

  pub fn set_stagnation(self, percentage: Pct, threshold: usize) -> Result<OptFacade, JsValue> {
    let this = js_err(self.0.set_stagnation(percentage.0, threshold))?;
    Ok(OptFacade(this))
  }

  pub fn solve(self, mut orig: OptProblem, rslts_num: usize) -> Result<OptProblem, JsValue> {
    let (mph_defs, mut mph_rslts) = orig.0.parts_mut();

    let mp_defs_ref = js_err({
      let mut mdfgd = js_err(mp_defs_from_gp_defs(mph_defs))?;
      mdfgd = js_err(mdfgd.push_obj(Either::Right(MinCstrsRslts::from_gp_hcs(mph_defs))))?;
      mdfgd.build()
    })?;
    let mut mp_ref = js_err(MpVec::with_random_solutions(mp_defs_ref, 100))?;

    let spea2 = js_err(Spea2::new(
      crate::Pct::from_percent(50),
      js_err(
        GeneticAlgorithmParamsBuilder::default()
          .crossover(MultiPoint::new(1, crate::Pct::from_percent(70)))
          .mating_selection(Tournament::new(2, ObjsAvg))
          .mutation(RandomDomainAssignments::new(1, crate::Pct::from_percent(30)))
          .build(),
      )?,
      &mp_ref,
      rslts_num,
    ))?;
    let facade = opt::OptFacade::new(self.0.max_iterations())
      .set_quality_comparator(ObjsAvg)
      .set_opt_hooks(());
    let facade = if let Some((pct, threshold)) = self.0.stagnation() {
      js_err(facade.set_stagnation(pct, threshold))?
    } else {
      facade
    };
    let _this = js_err(facade.solve_problem_with(&mut mp_ref, spea2))?;
    js_err(MphMpMph::transfer(&mph_defs, &mut mph_rslts, &mp_ref))?;

    Ok(orig)
  }
}

// OptProblem

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblem(MphVec<Domain, HardCstr, Obj, f64, Solution>);

#[wasm_bindgen]
impl OptProblem {
  pub fn with_capacity(defs: OptProblemDefinitions, len: usize) -> Self {
    OptProblem(MphVec::with_capacity(defs.0, len))
  }

  pub fn defs(self) -> OptProblemDefinitions {
    OptProblemDefinitions(self.0.into_parts().0)
  }

  pub fn rslts(self) -> OptProblemRslts {
    OptProblemRslts(self.0.into_parts().1)
  }
}

// OptProblemDefinitions

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblemDefinitions(MphDefinitionsVec<Domain, HardCstr, Obj>);

// OptProblemDefinitionsBuilder

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct OptProblemDefinitionsBuilder(MphDefinitionsBuilderVec<Domain, HardCstr, Obj>);

#[wasm_bindgen]
impl OptProblemDefinitionsBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self(MphDefinitionsBuilderVec::default())
  }

  pub fn build(self) -> Result<OptProblemDefinitions, JsValue> {
    Ok(OptProblemDefinitions(js_err(self.0.build())?))
  }

  pub fn domain(self, domain: Domain) -> Self {
    Self(self.0.domain(domain))
  }

  pub fn push_hard_cstr(
    self,
    hard_cstr: HardCstr,
  ) -> Result<OptProblemDefinitionsBuilder, JsValue> {
    Ok(Self(js_err(self.0.push_hard_cstr(hard_cstr))?))
  }

  pub fn push_obj(self, obj: Obj) -> Result<OptProblemDefinitionsBuilder, JsValue> {
    Ok(Self(js_err(self.0.push_obj(obj))?))
  }
}

// OptProblemRslts

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblemRslts(MphOrsVec<f64, Solution>);

#[wasm_bindgen]
impl OptProblemRslts {
  pub fn get(&self, idx: usize) -> Result<OptProblemResult, JsValue> {
    let result = self.0.get(idx).ok_or_else(|| JsValue::from("Unknown element"))?;
    Ok(OptProblemResult(result.to_mph_vec()))
  }

  pub fn rslts_num(&self) -> usize {
    self.0.rslts_num()
  }
}

// OptProblemRslts

#[wasm_bindgen]
#[derive(Debug)]
pub struct OptProblemResult(MphOrVec<f64, Solution>);

#[wasm_bindgen]
impl OptProblemResult {
  pub fn hard_cstr_rslts(&self) -> Vec<usize> {
    self.0.hard_cstr_rslts().iter().copied().collect()
  }

  pub fn obj_rslts(&self) -> Vec<f64> {
    self.0.obj_rslts().to_vec()
  }

  pub fn solution(&self) -> Solution {
    self.0.solution().clone()
  }
}

// Pct

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pct(crate::Pct);

#[wasm_bindgen]
impl Pct {
  pub fn from_percent(pct: u16) -> Self {
    Pct(crate::Pct::from_percent(pct))
  }
}

// Solution

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Solution(ArrayVec<f64, 16>);

#[wasm_bindgen]
impl Solution {
  pub fn array(&self) -> Vec<f64> {
    self.0.to_vec()
  }
}

impl crate::Solution for Solution {
  const MAX_LEN: usize = 16;

  fn has_var(&self, idx: usize) -> bool {
    self.0.has_var(idx)
  }

  fn inter_swap(&mut self, other: &mut Self, idx: usize) {
    self.0.inter_swap(&mut other.0, idx);
  }

  fn intra_swap(&mut self, a: usize, b: usize) {
    self.0.intra_swap(a, b);
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}

// Domain

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Domain(ArrayVec<RangeInclusive<f64>, 16>);

#[wasm_bindgen]
impl Domain {
  #[wasm_bindgen(constructor)]
  pub fn new(ranges: Vec<JsValue>) -> Result<Domain, JsValue> {
    let mut va = ArrayVec::new();
    for x in ranges {
      let string = x.as_string().ok_or_else(|| JsValue::from_str("Bad range"))?;
      let mut iter = string.split('=');
      let lower_bound = Self::parse_bound(iter.next());
      let upper_bound = Self::parse_bound(iter.next());
      va.push(lower_bound..=upper_bound);
    }
    Ok(Domain(va))
  }

  fn parse_bound(bound_option: Option<&str>) -> f64 {
    if let Some(bound_str) = bound_option {
      if let Ok(bound) = bound_str.parse::<f64>() {
        bound
      } else {
        panic!("Couldn't parse domain range into a floating number");
      }
    } else {
      panic!("Empty left or right bound for range domain");
    }
  }
}

impl crate::Domain<Solution> for Domain {
  type Error = crate::Error;

  fn len(&self) -> usize {
    self.0.len()
  }

  fn new_random_solution<R>(&self, rng: &mut R) -> Result<Solution, crate::Error>
  where
    R: Rng,
  {
    Ok(Solution(self.0.new_random_solution(rng)?))
  }

  fn set_rnd_domain<R>(&self, s: &mut Solution, idx: usize, rng: &mut R)
  where
    R: Rng,
  {
    self.0.set_rnd_domain(&mut s.0, idx, rng);
  }
}

fn js_err<T, E>(rslt: Result<T, E>) -> Result<T, JsValue>
where
  E: core::fmt::Debug,
{
  rslt.map_err(|e| JsValue::from(format!("{:?}", e)))
}

#[cfg(test)]
mod tests {
  use crate::wasm_bindgen::*;
  use alloc::vec;
  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  fn test_problem() {
    let opdb = OptProblemDefinitionsBuilder::default()
      .domain(Domain::new(vec![JsValue::from_str("0=5"), JsValue::from_str("0=3")]).unwrap())
      .push_hard_cstr(HardCstr::new(Function::new_with_args(
        "solution, value",
        "let x = solution[0]; let y = solution[1]; \
        return (Math.pow(x, 2) - 10 * x + 25) + Math.pow(y, 2) > 25 | 0;",
      )))
      .unwrap()
      .push_hard_cstr(HardCstr::new(Function::new_with_args(
        "solution, value",
        "let x = solution[0]; let y = solution[1]; \
        return (Math.pow(x, 2) - 16 * x + 64) + (Math.pow(y, 2) + 6 * y + 9) < 7.7 | 0;",
      )))
      .unwrap()
      .push_obj(Obj::new(
        ObjDirection::Min,
        Function::new_with_args(
          "solution, value",
          "return 4 * Math.pow(solution[0], 2) + 4 * Math.pow(solution[1], 2);",
        ),
      ))
      .unwrap()
      .push_obj(Obj::new(
        ObjDirection::Min,
        Function::new_with_args(
          "solution, value",
          "let x = solution[0]; let y = solution[1]; \
          return (Math.pow(x, 2) - 10 * x + 25) + (Math.pow(y, 2) - 10 * y + 25);",
        ),
      ))
      .unwrap();

    let problem = OptProblem::with_capacity(opdb.build().unwrap(), 100);
    let facade = OptFacade::new(50).set_stagnation(Pct::from_percent(1), 10).unwrap();
    let _opt_problem = facade.solve(problem, 100).unwrap();
  }
}
