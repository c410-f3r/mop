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
use cl_traits::{Push, Storage, WithCapacity};
#[cfg(feature = "with-rand")]
use rand::{rngs::StdRng, SeedableRng};
pub use {
  gp_definitions::*,
  gp_definitions_builder::*,
  gp_operations::{mph_mp_mph::*, mphs_mp_mphs::*, *},
  gp_or::*,
  gp_ors::*,
  gp_ors_constructor::*,
  gp_ors_evaluators::*,
};

/// Marker for unconstrained problems
pub type NoCstr = ArrayVec<[(); 0]>;
/// Marker result for unconstrained problems
pub type NoCstrRslts = ArrayVec<[(); 0]>;
/// Marker for single-objective problems
pub type OneObj<O> = ArrayVec<[O; 1]>;

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
  pub fn defs(&self) -> &GpDefinitions<D, HCS, OS, SCS> {
    &self.defs
  }

  pub fn into_parts(self) -> (GpDefinitions<D, HCS, OS, SCS>, GpOrs<HCRS, ORS, SCRS, SS>) {
    (self.defs, self.ors)
  }

  pub fn parts(&self) -> (&GpDefinitions<D, HCS, OS, SCS>, &GpOrs<HCRS, ORS, SCRS, SS>) {
    (&self.defs, &self.ors)
  }

  pub fn parts_mut(
    &mut self,
  ) -> (&GpDefinitions<D, HCS, OS, SCS>, &mut GpOrs<HCRS, ORS, SCRS, SS>) {
    (&mut self.defs, &mut self.ors)
  }

  pub fn rslts(&self) -> &GpOrs<HCRS, ORS, SCRS, SS> {
    &self.ors
  }

  pub fn rslts_mut(&mut self) -> &mut GpOrs<HCRS, ORS, SCRS, SS> {
    &mut self.ors
  }
}

impl<D, HC, HCR, HCRS, HCS, O, OR, ORS, OS, S, SC, SCR, SCRS, SCS, SS>
  Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>
where
  HCRS: Extend<HCR> + Storage<Item = HCR> + WithCapacity<Input = usize>,
  HCS: AsRef<[HC]> + Storage<Item = HC>,
  ORS: Extend<OR> + Storage<Item = OR> + WithCapacity<Input = usize>,
  O: Obj<OR, S>,
  OS: AsRef<[O]> + Storage<Item = O>,
  SCRS: Extend<SCR> + Storage<Item = SCR> + WithCapacity<Input = usize>,
  SCS: AsRef<[SC]> + Storage<Item = SC>,
  SS: Push<Input = S> + Storage<Item = S> + WithCapacity<Input = usize>,
{
  pub fn with_capacity(defs: GpDefinitions<D, HCS, OS, SCS>, rslts_num: usize) -> Self {
    let ors = GpOrs::with_capacity(&defs, rslts_num);
    Self { defs, ors }
  }

  #[cfg(feature = "with-rand")]
  pub fn with_random_solutions(
    defs: GpDefinitions<D, HCS, OS, SCS>,
    rslts_num: usize,
  ) -> crate::Result<Self>
  where
    crate::Error: From<D::Error>,
    D: crate::Domain<S>,
    HCR: Clone + Default,
    OR: Clone + Default,
    SCR: Clone + Default,
  {
    let mut rng = StdRng::from_entropy();
    let mut ors = GpOrs::with_capacity(&defs, rslts_num);
    let fun = (0..rslts_num).map(|_| defs.domain().new_random_solution(&mut rng));
    ors.constructor().ors_s_iter(fun);
    Ok(Self { defs, ors })
  }

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
    ors.constructor().ors_s_iter((0..rslts_num).map(|idx| Ok::<_, ()>(f(idx))));
    Self { defs, ors }
  }
}
