//! Zitzler and Thiele; SPEA2: Improving the Strength Pareto Evolutionary Algorithm

use cl_traits::{Clear, Push, Storage, WithCapacity};

mod arch_union_popul;
mod environment_selection_truncation_result;

use crate::{
  genetic_algorithm::{
    operators::{crossover::Crossover, mating_selection::MatingSelection, mutation::Mutation},
    GeneticAlgorithmParams,
  },
  utils::euclidean_distance,
};
use alloc::{boxed::Box, vec::Vec};
use arch_union_popul::{ArchUnionPopul, Properties};
use core::{
  cmp::Ordering,
  fmt::Debug,
  marker::PhantomData,
  mem::swap,
  ops::{AddAssign, Div, Mul, Sub},
};
use environment_selection_truncation_result::EnvironmentSelectionTruncationResult;
use mop_blocks::{
  gp::{GpOrsEvaluators, Mp, MpOrs},
  utils::verify_pareto_dominance,
  Obj, Pct, Solution,
};
use mop_common::{Solver, SolverFuture, TraitCfg};
use num_traits::{NumCast, One, Pow, Zero};
use rand::distributions::uniform::SampleUniform;

/// # Types
///
/// * `C`: Constraint
/// * `CO`: CrossOver
/// * `D`: Solution Domain
/// * `M`: Mutation
/// * `MS`: Mating Selection
/// * `O`: Objective
/// * `OR`: Objective Result
#[derive(Debug)]
pub struct Spea2<CO, D, M, MS, OR, ORS, OS, SS> {
  arch_rslts: MpOrs<ORS, SS>,
  arch_u_popul: ArchUnionPopul<OR, ORS, SS>,
  archive_size: usize,
  estr: Vec<EnvironmentSelectionTruncationResult<OR>>,
  gap: GeneticAlgorithmParams<CO, M, MS>,
  intermediary_arch_rslts: MpOrs<ORS, SS>,
  k_buffer: Vec<OR>,
  k: usize,
  mating_pool: MpOrs<ORS, SS>,
  phantom: PhantomData<(D, OS)>,
  population_size: usize,
  two: OR,
}

impl<CO, D, M, MS, O, OR, ORS, OS, S, SS> Spea2<CO, D, M, MS, OR, ORS, OS, SS>
where
  O: Obj<OR, S>,
  OR: AddAssign<OR>
    + Copy
    + Div<OR, Output = OR>
    + NumCast
    + One
    + PartialOrd
    + Pow<OR, Output = OR>
    + Sub<OR, Output = OR>
    + Zero,
  ORS: AsRef<[OR]> + Clear + Extend<OR> + Storage<Item = OR> + WithCapacity<Input = usize>,
  OS: AsRef<[O]> + Storage<Item = O>,
  S: Clone + Solution,
  SS: AsRef<[S]>
    + Clear
    + Extend<S>
    + Push<Input = S>
    + Storage<Item = S>
    + WithCapacity<Input = usize>,
{
  pub fn new(
    archive_size_pct: Pct,
    gap: GeneticAlgorithmParams<CO, M, MS>,
    problem: &Mp<D, ORS, OS, SS>,
    population_size: usize,
  ) -> Result<Self, mop_blocks::Error> {
    let defs = problem.defs();
    let archive_size = archive_size_pct * population_size;
    let arch_u_popul_len = archive_size.saturating_add(population_size);
    Ok(Spea2 {
      arch_rslts: MpOrs::with_capacity(defs, archive_size),
      arch_u_popul: ArchUnionPopul {
        rslts: MpOrs::with_capacity(defs, arch_u_popul_len),
        props: Vec::with_capacity(arch_u_popul_len),
      },
      archive_size,
      phantom: PhantomData,
      estr: Vec::with_capacity(archive_size),
      intermediary_arch_rslts: MpOrs::with_capacity(defs, archive_size),
      k_buffer: Vec::with_capacity(population_size),
      gap,
      k: num_integer::sqrt(arch_u_popul_len),
      mating_pool: MpOrs::with_capacity(defs, population_size),
      population_size,
      two: mop_blocks::Error::cast_rslt(2)?,
    })
  }

  fn copy_less_than_one_to_archive(&mut self) {
    self.arch_rslts.clear();
    let (aup, ar) = (&self.arch_u_popul, &mut self.arch_rslts);
    for (r, _) in aup.rslts.iter().zip(&aup.props).filter(|&(_, prop)| prop.fitness < OR::one()) {
      ar.constructor().or_ref(r);
    }
  }

  fn manage_environment_selection_diff(&mut self) -> Option<()> {
    match self.arch_rslts.rslts_num().cmp(&self.archive_size) {
      Ordering::Equal => {}
      Ordering::Greater => self.environment_selection_truncation()?,
      Ordering::Less => {
        let (aup, ar) = (&mut self.arch_u_popul, &mut self.arch_rslts);
        sort_partial_by(&mut aup.props, |a, b| a.fitness.partial_cmp(&b.fitness));
        for prop in aup
          .props
          .iter()
          .filter(|prop| prop.fitness >= OR::one())
          .take(self.archive_size - ar.rslts_num())
        {
          let r = aup.rslts.get(prop.result_idx)?;
          ar.constructor().or_ref(r);
        }
      }
    }
    Some(())
  }

  fn environment_selection_truncation(&mut self) -> Option<()> {
    let ar = &mut self.arch_rslts;
    self.estr.clear();
    for fst_idx in 0..ar.rslts_num() {
      let k_buffer = &mut self.k_buffer;
      k_buffer.clear();
      for sec_idx in 0..ar.rslts_num() {
        let ed = euclidean_distance(ar.get(fst_idx)?.obj_rslts(), ar.get(sec_idx)?.obj_rslts())?;
        k_buffer.push(ed);
      }
      sort_partial_by(k_buffer, |a, b| a.partial_cmp(b));
      self.estr.push(EnvironmentSelectionTruncationResult {
        archive_idx: fst_idx,
        distance: k_buffer[self.k],
      });
    }
    sort_partial_by(&mut self.estr, |a, b| a.distance.partial_cmp(&b.distance));
    self.estr.truncate(self.archive_size);
    self.intermediary_arch_rslts.clear();
    for archive_idx in self.estr.iter().map(|estr| estr.archive_idx) {
      let r = ar.get(archive_idx)?;
      self.intermediary_arch_rslts.constructor().or_ref(r);
    }
    swap(&mut self.intermediary_arch_rslts, ar);
    Some(())
  }

  fn fill_arch_u_popul(&mut self, p: &Mp<D, ORS, OS, SS>) {
    self.arch_u_popul.rslts.clear();
    self.arch_u_popul.rslts.constructor().ors_ref(self.arch_rslts.as_ref());
    self.arch_u_popul.rslts.constructor().ors_ref(p.rslts().as_ref());
    self.arch_u_popul.props.clear();
    self.arch_u_popul.props.extend(
      (0..self.arch_u_popul.rslts.rslts_num()).map(|idx| Properties {
        fitness: OR::zero(),
        result_idx: idx,
        strength: OR::zero(),
      }),
    );
  }

  fn set_density(&mut self) -> Option<()> {
    let (props, rslts) = (&mut self.arch_u_popul.props, &self.arch_u_popul.rslts);
    for (idx, (fst_ind, first_prop)) in rslts.iter().zip(props).enumerate() {
      let k_buffer = &mut self.k_buffer;
      let distance = |sec_ind: &[OR]| euclidean_distance(fst_ind.obj_rslts(), sec_ind);
      k_buffer.clear();
      for sec_ind in rslts.iter().take(idx) {
        k_buffer.push(distance(sec_ind.obj_rslts())?);
      }
      k_buffer.push(OR::one());
      for sec_ind in rslts.iter().skip(idx + 1) {
        k_buffer.push(distance(sec_ind.obj_rslts())?)
      }
      sort_partial_by(k_buffer, |a, b| a.partial_cmp(b));
      let density = OR::one() / (k_buffer[self.k] + self.two);
      first_prop.fitness += density;
    }
    Some(())
  }

  fn set_strength(&mut self, p: &mut Mp<D, ORS, OS, SS>) {
    let objs = p.defs().objs();
    let (props, rslts) = (&mut self.arch_u_popul.props, &self.arch_u_popul.rslts);
    for (fst_idx, fst_ind) in rslts.iter().enumerate() {
      for (sec_idx, sec_ind) in rslts.iter().enumerate().skip(fst_idx + 1) {
        match verify_pareto_dominance(objs, fst_ind.obj_rslts(), sec_ind.obj_rslts()) {
          Ordering::Equal => {}
          Ordering::Greater => {
            props[fst_idx].strength += OR::one();
          }
          Ordering::Less => {
            props[sec_idx].strength += OR::one();
          }
        }
      }
    }
  }

  fn set_raw_fitness(&mut self, p: &mut Mp<D, ORS, OS, SS>) {
    let objs = p.defs().objs();
    let (props, rslts) = (&mut self.arch_u_popul.props, &self.arch_u_popul.rslts);
    for (fst_idx, fst_ind) in rslts.iter().enumerate() {
      for (sec_idx, sec_ind) in rslts.iter().enumerate().skip(fst_idx) {
        match verify_pareto_dominance(objs, fst_ind.obj_rslts(), sec_ind.obj_rslts()) {
          Ordering::Equal => {}
          Ordering::Greater => {
            let strength = props[fst_idx].strength;
            props[sec_idx].fitness += strength;
          }
          Ordering::Less => {
            let strength = props[sec_idx].strength;
            props[fst_idx].fitness += strength;
          }
        }
      }
    }
  }
}

impl<CO, D, M, MS, O, OR, ORS, OS, S, SS> Solver<Mp<D, ORS, OS, SS>>
  for Spea2<CO, D, M, MS, OR, ORS, OS, SS>
where
  D: TraitCfg,
  CO: Crossover<MpOrs<ORS, SS>> + TraitCfg,
  M: Mutation<D, MpOrs<ORS, SS>> + TraitCfg,
  MS: MatingSelection<[O], MpOrs<ORS, SS>> + TraitCfg,
  O: Obj<OR, S> + TraitCfg,
  OR: AddAssign<OR>
    + Copy
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
  ORS: AsMut<[OR]>
    + AsRef<[OR]>
    + Clear
    + Extend<OR>
    + Storage<Item = OR>
    + TraitCfg
    + WithCapacity<Input = usize>,
  OS: AsRef<[O]> + Storage<Item = O> + TraitCfg,
  S: Clone + Solution + TraitCfg,
  SS: AsMut<[S]>
    + AsRef<[S]>
    + Clear
    + Extend<S>
    + Push<Input = S>
    + Storage<Item = S>
    + TraitCfg
    + WithCapacity<Input = usize>,
  mop_blocks::Error: From<CO::Error> + From<M::Error> + From<MS::Error>,
{
  type Error = mop_blocks::Error;

  fn after_iter<'a>(&'a mut self, p: &'a mut Mp<D, ORS, OS, SS>) -> SolverFuture<'a, Self::Error> {
    Box::pin(async move {
      let filling_num = self.population_size;
      self.gap.mating_selection.mating_selection(
        p.defs().objs(),
        &mut self.arch_rslts,
        &mut self.mating_pool,
        filling_num,
      )?;
      self.gap.crossover.crossover(&mut self.mating_pool, p.rslts_mut(), filling_num)?;

      let (defs, rslts) = p.parts_mut();
      self.gap.mutation.mutation(defs.domain(), rslts)?;

      Ok(())
    })
  }

  fn before_iter<'a>(&'a mut self, p: &'a mut Mp<D, ORS, OS, SS>) -> SolverFuture<'a, Self::Error> {
    Box::pin(async move {
      let (defs, rslts) = p.parts_mut();
      let ar = &mut self.arch_rslts;
      GpOrsEvaluators::eval_objs(defs, ar).await;
      GpOrsEvaluators::eval_objs(defs, rslts).await;

      self.fill_arch_u_popul(p);
      self.set_strength(p);
      self.set_raw_fitness(p);
      mop_blocks::Error::opt_rslt(self.set_density())?;

      self.copy_less_than_one_to_archive();
      mop_blocks::Error::opt_rslt(self.manage_environment_selection_diff())?;
      Ok(())
    })
  }
}

#[allow(clippy::unwrap_used)]
fn sort_partial_by<F, T>(slice: &mut [T], f: F)
where
  F: Fn(&T, &T) -> Option<Ordering>,
{
  slice.sort_unstable_by(|a, b| f(a, b).unwrap());
}

#[cfg(test)]
mod tests {
  use crate::genetic_algorithm::{
    operators::{
      crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
    },
    GeneticAlgorithmParamsBuilder, Spea2,
  };
  use mop_blocks::{quality_comparator::ObjsAvg, utils::dummy_mp_with_solutions, Pct};

  #[test]
  fn spea2() {
    let is_equal = |x: f64, y: f64| (x - y).abs() < 0.001;

    let mut problem = dummy_mp_with_solutions();

    let mut spea2 = Spea2::new(
      Pct::from_percent(50),
      GeneticAlgorithmParamsBuilder::default()
        .crossover(MultiPoint::new(1, Pct::from_percent(70)))
        .mating_selection(Tournament::new(5, ObjsAvg))
        .mutation(RandomDomainAssignments::new(1, Pct::from_percent(30)))
        .build()
        .unwrap(),
      &problem,
      4,
    )
    .unwrap();

    spea2.fill_arch_u_popul(&problem);
    spea2.set_strength(&mut problem);

    assert!(is_equal(spea2.arch_u_popul.props[0].strength, 3.0));
    assert!(is_equal(spea2.arch_u_popul.props[1].strength, 2.0));
    assert!(is_equal(spea2.arch_u_popul.props[2].strength, 1.0));
    assert!(is_equal(spea2.arch_u_popul.props[3].strength, 0.0));

    spea2.set_raw_fitness(&mut problem);

    assert!(is_equal(spea2.arch_u_popul.props[0].fitness, 0.0));
    assert!(is_equal(spea2.arch_u_popul.props[1].fitness, 3.0));
    assert!(is_equal(spea2.arch_u_popul.props[2].fitness, 5.0));
    assert!(is_equal(spea2.arch_u_popul.props[3].fitness, 6.0));

    spea2.set_density();

    assert!(is_equal(spea2.arch_u_popul.props[0].fitness, 0.130_601_937_481_87));
    assert!(is_equal(spea2.arch_u_popul.props[1].fitness, 3.166_666_666_666_66));
    assert!(is_equal(spea2.arch_u_popul.props[2].fitness, 5.166_666_666_666_66));
    assert!(is_equal(spea2.arch_u_popul.props[3].fitness, 6.130_601_937_481_87));

    spea2.copy_less_than_one_to_archive();

    assert_eq!(spea2.arch_rslts.rslts_num(), 1);
    assert_eq!(spea2.arch_rslts.get(0), problem.rslts().get(0));

    spea2.manage_environment_selection_diff();

    assert_eq!(spea2.arch_rslts.get(0), problem.rslts().get(0));
    assert_eq!(spea2.arch_rslts.get(1), problem.rslts().get(1));
  }
}
