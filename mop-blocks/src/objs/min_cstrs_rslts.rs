use crate::{gp::GpDefinitions, Cstr, Obj, ObjDirection};
use core::{fmt::Debug, iter::Chain, slice::Iter};
use mop_common::TraitCfg;
use num_traits::{Bounded, NumCast};

#[derive(Debug)]
pub struct MinCstrsRslts<CI> {
  cstrs: CI,
}

impl<CI> MinCstrsRslts<CI> {
  #[inline]
  pub fn new(cstrs: CI) -> Self {
    Self { cstrs }
  }
}

impl<'a, C> MinCstrsRslts<Iter<'a, C>> {
  #[inline]
  pub fn from_gp_hcs<D, HCS, OS, SCS>(defs: &'a GpDefinitions<D, HCS, OS, SCS>) -> Self
  where
    HCS: AsRef<[C]>,
  {
    Self { cstrs: defs.hard_cstrs.as_ref().iter() }
  }
}

impl<'a, C> MinCstrsRslts<Chain<Iter<'a, C>, Iter<'a, C>>> {
  #[inline]
  pub fn from_gp_hcs_and_scs<D, HCS, OS, SCS>(defs: &'a GpDefinitions<D, HCS, OS, SCS>) -> Self
  where
    HCS: AsRef<[C]>,
    SCS: AsRef<[C]>,
  {
    let hci = defs.hard_cstrs.as_ref().iter();
    let sci = defs.soft_cstrs.as_ref().iter();
    Self { cstrs: hci.chain(sci) }
  }
}

impl<C, CI, OR, S> Obj<OR, S> for MinCstrsRslts<CI>
where
  C: Cstr<S>,
  CI: Clone + Iterator<Item = C> + TraitCfg,
  OR: Bounded + NumCast,
{
  #[inline]
  fn obj_direction(&self) -> ObjDirection {
    ObjDirection::Min
  }

  #[inline]
  fn result(&self, s: &S) -> OR {
    let sum: usize = self.cstrs.clone().map(|cstr| cstr.violations(s)).sum();
    OR::from(sum).unwrap_or_else(OR::max_value)
  }
}

impl<'a, C, CI, OR, S> From<&'a MinCstrsRslts<CI>> for &'a dyn Obj<OR, S>
where
  C: Cstr<S>,
  CI: Clone + Iterator<Item = C> + TraitCfg,
  OR: Bounded + NumCast,
{
  #[inline]
  fn from(f: &'a MinCstrsRslts<CI>) -> Self {
    f
  }
}
