# MOP (Many OPtimizations)

[![CI](https://github.com/c410-f3r/mop/workflows/CI/badge.svg)](https://github.com/c410-f3r/mop/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/mop.svg)](https://crates.io/crates/mop)
[![Documentation](https://docs.rs/mop/badge.svg)](https://docs.rs/mop)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.42-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.42.html)

MOP is a flexible and modular framework for different NP-Problems with different solvers. Through its default pipeline you can define your own custom problem and choose any supported solver combination.

See [this blog post](https://c410-f3r.github.io/posts/a-flexible-and-modular-framework-to-solve-np-problems) for more details or have fun using [the online playground](https://c410-f3r.github.io/mop-playground/).

## Example

The definitions and results of `Binh and Korn`, a multi-objective problem with two hard constraints and two objectives.

![Binh and Korn](https://imgur.com/VDB0oBM.jpg)

###### Picture taken from https://en.wikipedia.org/wiki/Test_functions_for_optimization#Test_functions_for_multi-objective_optimization.

```rust
// Binh T. and Korn U. (1997) MOBES: A Multiobjective Evolution Strategy for Constrained Optimization Problems

use core::cmp::Ordering;
use mop::{
  blocks::{
    gp::{new_defsb_o_ref, GpOperations, MpVec, MphDefinitionsBuilder, MphMpMph, MphOrRef, MphVec},
    objs::MinCstrsRslts,
    quality_comparator::ObjsAvg,
    ObjDirection, Pct,
  },
  facades::opt::OptFacade,
  solvers::genetic_algorithm::{
    operators::{
      crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
    },
    GeneticAlgorithmParamsBuilder, Spea2,
  },
};

const RSLTS_NUM: usize = 200;

type Solution = [f64; 2];

fn f1(s: &Solution) -> f64 {
  4.0 * s[0].powi(2) + 4.0 * s[1].powi(2)
}

fn f2(s: &Solution) -> f64 {
  (s[0].powi(2) - 10.0 * s[0] + 25.0) + (s[1].powi(2) - 10.0 * s[1] + 25.0)
}

fn g1(s: &Solution) -> usize {
  let lhs = (s[0].powi(2) - 10.0 * s[0] + 25.0) + s[1].powi(2);
  match lhs.partial_cmp(&25.0) {
    Some(Ordering::Equal) | Some(Ordering::Less) => 0,
    _ => 1,
  }
}

fn g2(s: &Solution) -> usize {
  let lhs = (s[0].powi(2) - 16.0 * s[0] + 64.0) + (s[1].powi(2) + 6.0 * s[1] + 9.0);
  match lhs.partial_cmp(&7.7) {
    Some(Ordering::Equal) | Some(Ordering::Greater) => 0,
    _ => 1,
  }
}

fn print_result(result: MphOrRef<f64, Solution>) {
  let solution = result.solution();
  let objs = result.obj_rslts();
  let hcs = result.hard_cstr_rslts();
  println!("x0: {}, x1: {}", solution[0], solution[1]);
  println!("f1: {}, f2: {}", objs[0], objs[1]);
  println!("g1: {}, g2: {}", hcs[0], hcs[1]);
  println!();
}

#[tokio::main] // Or any other runtime
async fn main() -> Result<(), mop::blocks::Error> {
  // Problem definitions and results
  let mut mph = MphVec::with_capacity(
    MphDefinitionsBuilder::default()
      .domain([0.0..=5.0, 0.0..=3.0])
      .name("Binh and Korn")
      .push_hard_cstr(g1 as fn(&Solution) -> usize)
      .push_hard_cstr(g2 as fn(&Solution) -> usize)
      .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
      .push_obj((ObjDirection::Min, f2 as fn(&Solution) -> f64))
      .build()?,
    RSLTS_NUM,
  );
  let (mph_defs, mut mph_rslts) = mph.parts_mut();

  // SPEA2 is an unconstrained solver but Binh and Korn is a constrained problem. To workaround
  // this incompatibility, our `mph` problem is converted to a `mp` problem by adding an objective
  // that minimizes all constraints violations.
  //
  // It is possible to define your own converstion procedure with any desired set of objectives.
  let mcr = MinCstrsRslts::from_gp_hcs(mph_defs);
  let mp_defs_ref = new_defsb_o_ref(mph_defs, mph_rslts).push_obj(&mcr as &_).build()?;
  let mut mp_ref = MpVec::with_random_solutions(mp_defs_ref, 100)?;

  // SPEA2 and overall genetic algorithm parameters are specified here.
  let spea2 = Spea2::new(
    Pct::from_percent(50),
    GeneticAlgorithmParamsBuilder::default()
      .crossover(MultiPoint::new(1, Pct::from_percent(70)))
      .mating_selection(Tournament::new(10, ObjsAvg))
      .mutation(RandomDomainAssignments::new(1, Pct::from_percent(30)))
      .build()?,
    &mp_ref,
    RSLTS_NUM,
  )?;

  // Generic criterias to inspect or stop the solving process.
  let of = OptFacade::new(50)
    .set_opt_hooks(())
    .set_quality_comparator(ObjsAvg)
    .set_stagnation(Pct::from_percent(2i32), 10)?
    .solve_problem_with(&mut mp_ref, spea2)
    .await?;

  // Transfers all solutions and objectives results of `mp` to `mph`.
  MphMpMph::transfer(&mph_defs, &mut mph_rslts, &mp_ref).await?;

  for (result_idx, result) in mph_rslts.iter().enumerate() {
    println!("***** Result #{} *****", result_idx + 1);
    print_result(result);
  }

  if let Some(best_idx) = of.curr_best_idx() {
    if let Some(result) = mph.rslts().get(best_idx) {
      println!("***** Best result *****");
      print_result(result);
    }
  }

  Ok(())
}
```

![Binh and Korn - Objectives](https://imgur.com/VwmLLzP.jpg)

## Solvers

* `SPEA2` (Zitzler and Thiele; SPEA2: Improving the Strength Pareto Evolutionary Algorithm)

## Features

- `no_std` by default
- Different storages (Array, Vec, Slice and more!)
- Fully documented
- Fuzz testing
- No `unsafe`

## Optional features

- `std`
- Bindings (wasm-bindgen)
- Concurrent evaluation (futures)
- Deserialization/Serialization (serde)
- Multidimensional storage (ndsparse)
