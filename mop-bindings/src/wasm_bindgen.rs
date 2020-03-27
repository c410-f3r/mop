#![deny(rust_2018_idioms)]

use arrayvec::ArrayVec;
use core::ops::RangeInclusive;
use mop::{
  blocks::{
    self,
    mph::{Mph, MphDefinitions, MphDefinitionsBuilder, MphOrVec, MphOrs},
    Cstr,
  },
  facades::{initial_solutions::RandomInitialSolutions, opt},
  solvers::{
    genetic_algorithm::{
      operators::{
        crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
      },
      GeneticAlgorithmParamsBuilder, Spea2,
    },
    quality_comparator::ParetoComparator,
  },
};
use rand::Rng;
use {
  js_sys::{Array, Function},
  wasm_bindgen::prelude::*,
};

// SolutionDomain

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SolutionDomain(ArrayVec<[RangeInclusive<f64>; 16]>);

#[wasm_bindgen]
impl SolutionDomain {
  #[wasm_bindgen(constructor)]
  pub fn new(ranges: Vec<JsValue>) -> Self {
    let mut va = ArrayVec::new();
    ranges.iter().for_each(|x| {
      let string = x.as_string().unwrap();
      let mut iter = string.split('=');
      let lower_bound = Self::parse_bound(iter.next());
      let upper_bound = Self::parse_bound(iter.next());
      va.push(lower_bound..=upper_bound);
    });
    SolutionDomain(va)
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

impl blocks::SolutionDomain<Solution> for SolutionDomain {
  fn len(&self) -> usize {
    self.0.len()
  }

  fn new_random_solution<R>(&self, rng: &mut R) -> Solution
  where
    R: Rng,
  {
    Solution(self.0.new_random_solution(rng))
  }

  fn set_rnd_solution_domain<R>(&self, s: &mut Solution, idx: usize, rng: &mut R)
  where
    R: Rng,
  {
    self.0.set_rnd_solution_domain(&mut s.0, idx, rng);
  }
}

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
      array.push(&jv);
    });
    self.0.call1(&JsValue::NULL, &JsValue::from(array)).unwrap().as_f64().unwrap() as usize
  }
}

// HardCstr

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

impl blocks::Obj<f64, Solution> for Obj {
  fn obj_direction(&self) -> blocks::ObjDirection {
    self.0.to_original()
  }

  fn result(&self, solution: &Solution) -> f64 {
    let array: Array = Array::new();
    solution.0.iter().for_each(|&x| {
      let jv = JsValue::from_f64(x);
      array.push(&jv);
    });
    self.1.call1(&JsValue::NULL, &JsValue::from(array)).unwrap().as_f64().unwrap()
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
  fn to_original(&self) -> blocks::ObjDirection {
    match self {
      ObjDirection::Max => blocks::ObjDirection::Max,
      ObjDirection::Min => blocks::ObjDirection::Min,
    }
  }
}

// OptFacade

#[wasm_bindgen]
#[derive(Debug)]
pub struct OptFacade(opt::OptFacade<HardCstr, Obj, (), f64, Solution, SolutionDomain>);

#[wasm_bindgen]
impl OptFacade {
  pub async fn solve(self, mut problem: OptProblem) -> OptProblem {
    let spea2 = Spea2::new(
      blocks::Pct::from_percent(50),
      GeneticAlgorithmParamsBuilder::default()
        .crossover(MultiPoint::new(1, blocks::Pct::from_percent(70)))
        .mating_selection(Tournament::new(2, ParetoComparator::default()))
        .mutation(RandomDomainAssignments::new(1, blocks::Pct::from_percent(30)))
        .build(),
      &problem.0,
      ParetoComparator::default(),
    );
    self.0.solve_problem_with(&mut problem.0, spea2).await;
    problem
  }
}

// OptFacadeBuilder

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct OptFacadeBuilder(opt::OptFacadeBuilder<(), f64>);

#[wasm_bindgen]
impl OptFacadeBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    OptFacadeBuilder(opt::OptFacadeBuilder::default())
  }

  pub fn build(self, problem: &mut OptProblem) -> OptFacade {
    let initial_solutions = RandomInitialSolutions::default();
    OptFacade(self.0.opt_hooks(()).build().initial_solutions(initial_solutions, &mut problem.0))
  }

  pub fn max_iterations(self, max_iterations: usize) -> Self {
    OptFacadeBuilder(self.0.max_iterations(max_iterations))
  }

  pub fn objs_goals(self, objs_goals: Vec<f64>) -> Self {
    OptFacadeBuilder(self.0.objs_goals(objs_goals))
  }

  pub fn stagnation_percentage(self, stagnation_percentage: Pct) -> Self {
    OptFacadeBuilder(self.0.stagnation_percentage(stagnation_percentage.0))
  }

  pub fn stagnation_threshold(self, stagnation_threshold: usize) -> Self {
    OptFacadeBuilder(self.0.stagnation_threshold(stagnation_threshold))
  }
}

// OptProblem

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblem(Mph<HardCstr, Obj, f64, Solution, SolutionDomain>);

#[wasm_bindgen]
impl OptProblem {
  pub fn with_capacity(definitions: OptProblemDefinitions, results_num: usize) -> Self {
    let results = MphOrs::with_capacity(&definitions.0, results_num);
    OptProblem(Mph::new(definitions.0, results))
  }

  pub fn definitions(self) -> OptProblemDefinitions {
    OptProblemDefinitions(self.0.into_parts().0)
  }

  pub fn results(self) -> OptProblemResults {
    OptProblemResults(self.0.into_parts().1)
  }
}

// OptProblemDefinitions

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblemDefinitions(MphDefinitions<HardCstr, Obj, SolutionDomain>);

// OptProblemDefinitionsBuilder

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct OptProblemDefinitionsBuilder(MphDefinitionsBuilder<HardCstr, Obj, SolutionDomain>);

#[wasm_bindgen]
impl OptProblemDefinitionsBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    OptProblemDefinitionsBuilder(MphDefinitionsBuilder::default())
  }

  pub fn build(self) -> OptProblemDefinitions {
    OptProblemDefinitions(self.0.build())
  }

  pub fn domain(self, solution_domain: SolutionDomain) -> Self {
    OptProblemDefinitionsBuilder(self.0.solution_domain(solution_domain))
  }

  pub fn name(self, name: String) -> Self {
    OptProblemDefinitionsBuilder(self.0.name(name))
  }

  pub fn push_hard_cstr(self, hard_cstr: HardCstr) -> Self {
    OptProblemDefinitionsBuilder(self.0.push_hard_cstr(hard_cstr))
  }

  pub fn push_obj(self, obj: Obj) -> Self {
    OptProblemDefinitionsBuilder(self.0.push_obj(obj))
  }
}

// OptProblemResults

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OptProblemResults(MphOrs<f64, Solution>);

#[wasm_bindgen]
impl OptProblemResults {
  pub fn best(&self) -> Option<OptProblemResult> {
    self.0.best().map(|b| OptProblemResult(b.to_vec()))
  }

  pub fn get(&self, idx: usize) -> OptProblemResult {
    OptProblemResult(self.0.get(idx).to_vec())
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }
}

// OptProblemResults

#[wasm_bindgen]
#[derive(Debug)]
pub struct OptProblemResult(MphOrVec<f64, Solution>);

#[wasm_bindgen]
impl OptProblemResult {
  pub fn hard_cstrs(&self) -> Vec<u32> {
    self.0.hard_cstrs().iter().map(|&x| x as u32).collect()
  }

  pub fn objs(&self) -> Vec<f64> {
    self.0.objs().to_vec()
  }

  pub fn objs_avg(&self) -> f64 {
    *self.0.objs_avg()
  }

  pub fn solution(&self) -> Solution {
    self.0.solution().clone()
  }
}

// Pct

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pct(blocks::Pct);

#[wasm_bindgen]
impl Pct {
  pub fn from_decimal(pct: f64) -> Self {
    Pct(blocks::Pct::from_decimal(pct))
  }

  pub fn from_percent(pct: f64) -> Self {
    Pct(blocks::Pct::from_percent(pct))
  }
}

// Solution

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Solution(ArrayVec<[f64; 16]>);

#[wasm_bindgen]
impl Solution {
  pub fn array(&self) -> Vec<f64> {
    self.0.to_vec()
  }
}

impl blocks::Solution for Solution {
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

#[cfg(test)]
mod tests {
  use crate::wasm_bindgen::*;
  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  fn test_problem() {
    let opdbj = OptProblemDefinitionsBuilder::new()
      .domain(SolutionDomain::new(vec![JsValue::from_str("0=5"), JsValue::from_str("0=3")]))
      .name("Problem".into())
      .push_hard_cstr(HardCstr::new(Function::new_with_args(
        "solution, value",
        "let x = solution[0]; let y = solution[1]; \
        return (Math.pow(x, 2) - 10 * x + 25) + Math.pow(y, 2) > 25 | 0;",
      )))
      .push_hard_cstr(HardCstr::new(Function::new_with_args(
        "solution, value",
        "let x = solution[0]; let y = solution[1]; \
        return (Math.pow(x, 2) - 16 * x + 64) + (Math.pow(y, 2) + 6 * y + 9) < 7.7 | 0;",
      )))
      .push_obj(Obj::new(
        ObjDirection::Min,
        Function::new_with_args(
          "solution, value",
          "return 4 * Math.pow(solution[0], 2) + 4 * Math.pow(solution[1], 2);",
        ),
      ))
      .push_obj(Obj::new(
        ObjDirection::Min,
        Function::new_with_args(
          "solution, value",
          "let x = solution[0]; let y = solution[1]; \
          return (Math.pow(x, 2) - 10 * x + 25) + (Math.pow(y, 2) - 10 * y + 25);",
        ),
      ));

    let mut problem = OptProblem::with_capacity(opdbj.build(), 100);
    let facade = OptFacadeBuilder::new()
      .max_iterations(50)
      .stagnation_percentage(Pct::from_percent(1.0))
      .stagnation_threshold(10)
      .build(&mut problem);

    wasm_bindgen_futures::spawn_local(async { facade.solve(problem).await; });
  }
}
