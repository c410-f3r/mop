//! Zitzler and Thiele; SPEA2: Improving the Strength Pareto Evolutionary Algorithm

mod arch_union_popul;
mod environment_selection_truncation_result;

use crate::{
  genetic_algorithm::{
    operators::{crossover::Crossover, mating_selection::MatingSelection, mutation::Mutation},
    GeneticAlgorithmParams,
  },
  quality_comparator::QualityComparator,
  utils::{euclidean_distance, verify_pareto_dominance},
};
use alloc::{boxed::Box, vec::Vec};
use arch_union_popul::{ArchUnionPopul, Properties};
use core::{
  cmp::Ordering,
  fmt::Debug,
  marker::PhantomData,
  mem::swap,
  ops::{Div, Mul, Sub},
};
use environment_selection_truncation_result::EnvironmentSelectionTruncationResult;
use mop_blocks::{
  mph::{Mph, MphOrRef, MphOrs, MphOrsEvaluators},
  Cstr, Obj, Pct, SolutionDomain,
};
use mop_common_defs::{Solver, SolverFuture, TraitCfg};
use num_traits::{NumCast, One, Pow, Zero};
use rand::distributions::uniform::SampleUniform;

/// # Types
///
/// * `C`: Constraint
/// * `CO`: CrossOver
/// * `M`: Mutation
/// * `MS`: Mating Selection
/// * `O`: Objective
/// * `OR`: Objective Result
/// * `SD`: Solution Domain
#[derive(Debug)]
pub struct Spea2<C, CO, QC, M, MS, O, OR, S, SD> {
  arch_results: MphOrs<OR, S>,
  arch_u_popul: ArchUnionPopul<OR, S>,
  archive_size: usize,
  estr: Vec<EnvironmentSelectionTruncationResult<OR>>,
  gap: GeneticAlgorithmParams<CO, M, MS>,
  intermediary_arch_results: MphOrs<OR, S>,
  k_buffer: Vec<OR>,
  k: usize,
  mating_pool: MphOrs<OR, S>,
  phantom: PhantomData<(C, O, SD)>,
  quality_comparator: QC,
}

impl<C, CO, QC, M, MS, O, OR, S, SD> Spea2<C, CO, QC, M, MS, O, OR, S, SD>
where
  QC: for<'a> QualityComparator<[O], MphOrRef<'a, OR, S>>,
  O: Obj<OR, S>,
  OR: Copy
    + Div<OR, Output = OR>
    + Mul<OR, Output = OR>
    + NumCast
    + One
    + PartialOrd
    + Pow<OR, Output = OR>
    + Sub<OR, Output = OR>
    + Zero,
  SD: SolutionDomain<S>,
  S: Clone,
{
  pub fn new(
    archive_size: Pct,
    gap: GeneticAlgorithmParams<CO, M, MS>,
    problem: &Mph<C, O, OR, S, SD>,
    quality_comparator: QC,
  ) -> Self {
    let (definitions, results) = problem.parts();
    let population_size = results.results_num();
    let archive_size = (*archive_size * population_size as f64) as usize;
    let arch_u_popul_len = archive_size + population_size;
    Spea2 {
      arch_results: MphOrs::with_capacity(definitions, archive_size),
      arch_u_popul: ArchUnionPopul {
        results: MphOrs::with_capacity(definitions, arch_u_popul_len),
        props: Vec::with_capacity(arch_u_popul_len),
      },
      archive_size,
      estr: Vec::with_capacity(archive_size),
      intermediary_arch_results: MphOrs::with_capacity(definitions, archive_size),
      k_buffer: Vec::with_capacity(population_size),
      gap,
      k: (arch_u_popul_len as f64).sqrt() as usize,
      mating_pool: MphOrs::with_capacity(definitions, population_size),
      phantom: PhantomData,
      quality_comparator,
    }
  }

  fn best_result(&mut self, p: &mut Mph<C, O, OR, S, SD>) -> usize {
    let mut best_idx = 0;
    let (defs, results) = p.parts();
    for current_idx in 1..results.len() {
      if self.quality_comparator.is_better(
        defs.objs(),
        &results.get(current_idx),
        &results.get(best_idx),
      ) {
        best_idx = current_idx;
      }
    }
    best_idx
  }

  fn copy_to_archive(&mut self) {
    self.arch_results.clear();
    let (aup, ar) = (&self.arch_u_popul, &mut self.arch_results);
    for (r, _) in aup.results.iter().zip(&aup.props).filter(|(_, prop)| prop.fitness < OR::one()) {
      ar.constructor().copy_result(&r).commit(**r.objs_avg(), (*r.solution()).clone());
    }
  }

  fn environment_selection(&mut self) {
    self.copy_to_archive();
    match self.arch_results.len().cmp(&self.archive_size) {
      Ordering::Equal => {}
      Ordering::Greater => self.environment_selection_truncation(),
      Ordering::Less => {
        let (aup, ar) = (&mut self.arch_u_popul, &mut self.arch_results);
        aup.props.sort_unstable_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        for prop in aup
          .props
          .iter()
          .filter(|prop| prop.fitness >= OR::one())
          .take(self.archive_size - ar.len())
        {
          let r = aup.results.get(prop.result_idx);
          ar.constructor().copy_result(&r).commit(**r.objs_avg(), (*r.solution()).clone());
        }
      }
    }
  }

  fn environment_selection_truncation(&mut self) {
    let ar = &mut self.arch_results;
    self.estr.clear();
    for fst_idx in 0..ar.len() {
      let k_buffer = &mut self.k_buffer;
      k_buffer.clear();
      for sec_idx in 0..ar.len() {
        k_buffer.push(euclidean_distance(ar.get(fst_idx).objs(), ar.get(sec_idx).objs()));
      }
      k_buffer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
      self.estr.push(EnvironmentSelectionTruncationResult {
        archive_idx: fst_idx,
        distance: k_buffer[self.k],
      });
    }
    self.estr.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    self.estr.truncate(self.archive_size);
    self.intermediary_arch_results.clear();
    for archive_idx in self.estr.iter().map(|estr| estr.archive_idx) {
      let r = ar.get(archive_idx);
      self
        .intermediary_arch_results
        .constructor()
        .copy_result(&r)
        .commit(**r.objs_avg(), (*r.solution()).clone());
    }
    swap(&mut self.intermediary_arch_results, ar);
  }

  fn fill_properties_with_default_values(&mut self) {
    let iter = (0..self.arch_u_popul.results.len()).map(|idx| Properties {
      fitness: OR::zero(),
      result_idx: idx,
      strength: OR::zero(),
    });
    self.arch_u_popul.props.clear();
    self.arch_u_popul.props.extend(iter);
  }

  fn fitness_assignment(&mut self, p: &mut Mph<C, O, OR, S, SD>) {
    self.fill_properties_with_default_values();
    self.set_strength(p);
    self.set_raw_fitness(p);
    self.set_density();
  }

  fn set_density(&mut self) {
    let (props, results) = (&mut self.arch_u_popul.props, &self.arch_u_popul.results);
    for (fst_ind, first_prop) in results.iter().zip(props) {
      let k_buffer = &mut self.k_buffer;
      k_buffer.clear();
      for sec_ind in results.iter() {
        k_buffer.push(euclidean_distance(fst_ind.objs(), sec_ind.objs()));
      }
      k_buffer.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
      let density = OR::one() / (k_buffer[self.k] + NumCast::from(2).unwrap());
      first_prop.fitness = first_prop.fitness + density;
    }
  }

  fn set_strength(&mut self, p: &mut Mph<C, O, OR, S, SD>) {
    let objs = p.definitions().objs();
    let (props, results) = (&mut self.arch_u_popul.props, &self.arch_u_popul.results);
    for (fst_idx, fst_ind) in results.iter().enumerate() {
      for (sec_idx, sec_ind) in results.iter().enumerate().skip(fst_idx) {
        match verify_pareto_dominance(objs, fst_ind.objs(), sec_ind.objs()) {
          Ordering::Greater => {
            props[fst_idx].strength = props[fst_idx].strength + OR::one();
          }
          Ordering::Less => {
            props[sec_idx].strength = props[sec_idx].strength + OR::one();
          }
          _ => {}
        }
      }
    }
  }

  fn set_raw_fitness(&mut self, p: &mut Mph<C, O, OR, S, SD>) {
    let objs = p.definitions().objs();
    let (props, results) = (&mut self.arch_u_popul.props, &self.arch_u_popul.results);
    for (fst_idx, fst_ind) in results.iter().enumerate() {
      for (sec_idx, sec_ind) in results.iter().enumerate().skip(fst_idx) {
        match verify_pareto_dominance(objs, fst_ind.objs(), sec_ind.objs()) {
          Ordering::Greater => {
            props[sec_idx].fitness = props[sec_idx].fitness + props[fst_idx].strength;
          }
          Ordering::Less => {
            props[fst_idx].fitness = props[fst_idx].fitness + props[sec_idx].strength;
          }
          _ => {}
        }
      }
    }
  }
}

impl<C, CO, QC, M, MS, O, OR, S, SD> Solver<Mph<C, O, OR, S, SD>> for Spea2<C, CO, QC, M, MS, O, OR, S, SD>
where
  C: Cstr<S> + TraitCfg,
  CO: Crossover<MphOrs<OR, S>> + TraitCfg,
  QC: for<'a> QualityComparator<[O], MphOrRef<'a, OR, S>> + TraitCfg,
  M: Mutation<SD, MphOrs<OR, S>> + TraitCfg,
  MS: MatingSelection<[O], MphOrs<OR, S>> + TraitCfg,
  O: Obj<OR, S> + TraitCfg,
  OR: Copy
    + Debug
    + Div<OR, Output = OR>
    + Mul<OR, Output = OR>
    + NumCast
    + One
    + PartialOrd
    + Pow<OR, Output = OR>
    + SampleUniform
    + Sub<OR, Output = OR>
    + TraitCfg
    + Zero,
  S: Clone + TraitCfg,
  SD: SolutionDomain<S> + TraitCfg,
{
  fn after_iter<'a>(&'a mut self, p: &'a mut Mph<C, O, OR, S, SD>) -> SolverFuture<'a> {
    let filling_num = p.results().results_num();
    self.gap.mating_selection.mating_selection(
      p.definitions().objs(),
      &mut self.arch_results,
      &mut self.mating_pool,
      filling_num,
    );
    self.gap.crossover.crossover(&mut self.mating_pool, p.results_mut(), filling_num);

    let (defs, results) = p.parts_mut();
    self.gap.mutation.mutation(defs.solution_domain_mut(), results);

    Box::pin(async {})
  }

  fn before_iter<'a>(&'a mut self, p: &'a mut Mph<C, O, OR, S, SD>) -> SolverFuture<'a> {
    Box::pin(async move {
      {
        let (defs, results) = p.parts_mut();
        let ar = &mut self.arch_results;
        MphOrsEvaluators::eval_cstrs_violations(defs, ar).await;
        MphOrsEvaluators::eval_objs(defs, ar).await;
        MphOrsEvaluators::eval_cstrs_violations(defs, results).await;
        MphOrsEvaluators::eval_objs(defs, results).await;
      }
      self.arch_u_popul.results.clear();
      self.arch_u_popul.results.extend(&self.arch_results);
      self.arch_u_popul.results.extend(&p.results_mut());
      self.fitness_assignment(p);
      self.environment_selection();
      let best_idx = self.best_result(p);
      p.results_mut().set_best(best_idx);
    })
  }
}
