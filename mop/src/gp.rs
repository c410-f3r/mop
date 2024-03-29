//! MP (Multi-objective Problem)

mod gp_definitions;
mod gp_definitions_builder;
mod gp_operations;
mod gp_or;
mod gp_ors;
mod gp_ors_constructor;
mod gp_ors_evaluators;

use crate::Obj;
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use cl_aux::{Push, SingleTypeStorage, WithCapacity};
pub use gp_definitions::*;
pub use gp_definitions_builder::*;
pub use gp_operations::{mph_mp_mph::*, mphs_mp_mphs::*, *};
pub use gp_or::*;
pub use gp_ors::*;
pub use gp_ors_constructor::*;
pub use gp_ors_evaluators::*;
#[cfg(feature = "rand")]
use rand::rngs::OsRng;

/// Marker for unconstrained problems
pub type NoCstr = ArrayVec<(), 0>;
/// Marker result for unconstrained problems
pub type NoCstrRslts = ArrayVec<(), 0>;
/// Marker for single-objective problems
pub type OneObj<O> = ArrayVec<O, 1>;

// MP (Multi-objective Problem)
pub type Mp<D, ORS, OS, SS> = Gp<D, NoCstrRslts, NoCstr, ORS, OS, NoCstrRslts, NoCstr, SS>;
// MPH (Multi-objective Problem with Hard constraints)
pub type Mph<D, HCRS, HCS, ORS, OS, SS> = Gp<D, HCRS, HCS, ORS, OS, NoCstrRslts, NoCstr, SS>;
// MPHS (Multi-objective Problem with Hard constraints and Soft constraints)
pub type Mphs<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> = Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>;
// SP (Single-objective Problem)
pub type Sp<D, ORS, O, SS> = Gp<D, NoCstrRslts, NoCstr, ORS, OneObj<O>, NoCstrRslts, NoCstr, SS>;

pub type MpVec<D, O, OR, S> = Mp<D, Vec<OR>, Vec<O>, Vec<S>>;
pub type MphsVec<D, HC, O, OR, S, SC> =
  Mphs<D, Vec<usize>, Vec<HC>, Vec<OR>, Vec<O>, Vec<usize>, Vec<SC>, Vec<S>>;
pub type MphVec<D, HC, O, OR, S> = Mph<D, Vec<usize>, Vec<HC>, Vec<OR>, Vec<O>, Vec<S>>;
pub type SpVec<D, O, OR, S> = Sp<D, Vec<OR>, O, Vec<S>>;

/// GP (Generic Problem)
///
/// This structure is generic over single or multi-objective problems, constrained or not.
///
/// # Types
///
/// * `D`: Domain
/// * `HCRS`: Hard Constraint Results Storage
/// * `HCS`: Hard Constraint Storage
/// * `ORS`: Objective Results Storage
/// * `OS`: Objective Storage
/// * `SCRS`: Soft Constraint Results Storage
/// * `SCS`: Soft Constraint Storage
/// * `SS`: Solution Storage
#[derive(Clone, Debug)]
pub struct Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> {
  pub(crate) defs: GpDefinitions<D, HCS, OS, SCS>,
  pub(crate) ors: GpOrs<HCRS, ORS, SCRS, SS>,
}

impl<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS> {
  #[inline]
  pub fn defs(&self) -> &GpDefinitions<D, HCS, OS, SCS> {
    &self.defs
  }

  #[inline]
  pub fn into_parts(self) -> (GpDefinitions<D, HCS, OS, SCS>, GpOrs<HCRS, ORS, SCRS, SS>) {
    (self.defs, self.ors)
  }

  #[inline]
  pub fn parts(&self) -> (&GpDefinitions<D, HCS, OS, SCS>, &GpOrs<HCRS, ORS, SCRS, SS>) {
    (&self.defs, &self.ors)
  }

  #[inline]
  pub fn parts_mut(
    &mut self,
  ) -> (&GpDefinitions<D, HCS, OS, SCS>, &mut GpOrs<HCRS, ORS, SCRS, SS>) {
    (&mut self.defs, &mut self.ors)
  }

  #[inline]
  pub fn rslts(&self) -> &GpOrs<HCRS, ORS, SCRS, SS> {
    &self.ors
  }

  #[inline]
  pub fn rslts_mut(&mut self) -> &mut GpOrs<HCRS, ORS, SCRS, SS> {
    &mut self.ors
  }
}

impl<D, HC, HCR, HCRS, HCS, O, OR, ORS, OS, S, SC, SCR, SCRS, SCS, SS>
  Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  HCRS: Extend<HCR> + SingleTypeStorage<Item = HCR> + WithCapacity<Input = usize>,
  HCS: AsRef<[HC]> + SingleTypeStorage<Item = HC>,
  ORS: Extend<OR> + SingleTypeStorage<Item = OR> + WithCapacity<Input = usize>,
  O: Obj<OR, S>,
  OS: AsRef<[O]> + SingleTypeStorage<Item = O>,
  SCRS: Extend<SCR> + SingleTypeStorage<Item = SCR> + WithCapacity<Input = usize>,
  SCS: AsRef<[SC]> + SingleTypeStorage<Item = SC>,
  SS: Push<S> + SingleTypeStorage<Item = S> + WithCapacity<Input = usize>,
{
  #[inline]
  pub fn with_capacity(defs: GpDefinitions<D, HCS, OS, SCS>, rslts_num: usize) -> Self {
    let ors = GpOrs::with_capacity(&defs, rslts_num);
    Self { defs, ors }
  }

  #[cfg(feature = "rand")]
  #[inline]
  pub fn with_random_solutions(
    defs: GpDefinitions<D, HCS, OS, SCS>,
    rslts_num: usize,
  ) -> crate::Result<Self>
  where
    D: crate::Domain<S>,
    HCR: Clone + Default,
    OR: Clone + Default,
    SCR: Clone + Default,
  {
    let mut rng = OsRng;
    let mut ors = GpOrs::with_capacity(&defs, rslts_num);
    let fun = (0..rslts_num).map(|_| defs.domain().new_random_solution(&mut rng));
    let _ = ors.constructor().ors_s_iter(fun);
    Ok(Self { defs, ors })
  }

  #[inline]
  pub fn with_user_solutions<F>(
    defs: GpDefinitions<D, HCS, OS, SCS>,
    rslts_num: usize,
    mut f: F,
  ) -> Self
  where
    F: FnMut(usize) -> S,
    HCR: Clone + Default,
    OR: Clone + Default,
    SCR: Clone + Default,
  {
    let mut ors = GpOrs::with_capacity(&defs, rslts_num);
    let _ = ors.constructor().ors_s_iter((0..rslts_num).map(|idx| Ok::<_, ()>(f(idx))));
    Self { defs, ors }
  }
}
