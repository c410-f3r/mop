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
use core::cmp::Ordering;
use mop::{
  blocks::{
    mph::{Mph, MphDefinitionsBuilder, MphOrRef},
    ObjDirection, Pct,
  },
  facades::{initial_solutions::RandomInitialSolutions, opt::OptFacadeBuilder},
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
  let objs = result.objs();
  let hcs = result.hard_cstrs();
  println!("x0: {}, x1: {}", solution[0], solution[1]);
  println!("f1: {}, f2: {}, AVG: {}", objs[0], objs[1], result.objs_avg());
  println!("g1: {}, g2: {}", hcs[0], hcs[1]);
  println!();
}

#[tokio::main] // Or any other runtime
async fn main() {
  let mut problem = Mph::with_capacity(
    MphDefinitionsBuilder::default()
      .name("Binh and Korn")
      .push_hard_cstr(g1 as fn(&Solution) -> usize)
      .push_hard_cstr(g2)
      .push_obj((ObjDirection::Min, f1 as fn(&Solution) -> f64))
      .push_obj((ObjDirection::Min, f2))
      .solution_domain([0.0..=5.0, 0.0..=3.0])
      .build(),
    500,
  );

  let facade = OptFacadeBuilder::default()
    .max_iterations(50)
    .opt_hooks(())
    .stagnation_percentage(Pct::from_percent(2))
    .stagnation_threshold(10)
    .build();

  let spea2 = Spea2::new(
    Pct::from_percent(50),
    GeneticAlgorithmParamsBuilder::default()
      .crossover(MultiPoint::new(1, Pct::from_percent(70)))
      .mating_selection(Tournament::new(10, ParetoComparator::default()))
      .mutation(RandomDomainAssignments::new(1, Pct::from_percent(30)))
      .build(),
    &problem,
    ParetoComparator::default(),
  );

  facade
    .initial_solutions(RandomInitialSolutions::default(), &mut problem)
    .solve_problem_with(&mut problem, spea2).await;

  for (result_idx, result) in problem.results().iter().enumerate() {
    println!("***** Result #{} *****", result_idx + 1);
    print_result(result);
  }

  println!("***** Best result *****");
  print_result(problem.results().best().unwrap());
}
```

![Binh and Korn - Objectives](https://imgur.com/VwmLLzP.jpg)

## Solvers

* `SPEA2` (Zitzler and Thiele; SPEA2: Improving the Strength Pareto Evolutionary Algorithm)

## Optional features

- `std`
- Bindings (wasm-bindgen)
- Concurrent evaluation (futures)
- Deserialization/Serialization (serde)
- Dynamic arrays (ArrayVec)
- Multidimensional storage (ndsparse)
