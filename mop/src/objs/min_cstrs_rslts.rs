use crate::{gp::GpDefinitions, Cstr, Obj, ObjDirection};
use core::{fmt::Debug, iter::Chain, slice::Iter};
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

impl<'any, C> MinCstrsRslts<Iter<'any, C>> {
  #[inline]
  pub fn from_gp_hcs<D, HCS, OS, SCS>(defs: &'any GpDefinitions<D, HCS, OS, SCS>) -> Self
  where
    HCS: AsRef<[C]>,
  {
    Self { cstrs: defs.hard_cstrs.as_ref().iter() }
  }
}

impl<'any, C> MinCstrsRslts<Chain<Iter<'any, C>, Iter<'any, C>>> {
  #[inline]
  pub fn from_gp_hcs_and_scs<D, HCS, OS, SCS>(defs: &'any GpDefinitions<D, HCS, OS, SCS>) -> Self
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
  CI: Clone + Iterator<Item = C>,
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
