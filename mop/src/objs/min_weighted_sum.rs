use crate::{gp::Gp, Obj, ObjDirection};
use cl_aux::SingleTypeStorage;
use core::{
  iter::Sum,
  ops::{Add, Div},
  slice::Iter,
};
use num_traits::One;

#[derive(Debug)]
pub struct MinWeightedSum<OI, WI> {
  objs: OI,
  weights: WI,
}

impl<OI, WI> MinWeightedSum<OI, WI> {
  #[inline]
  pub fn new<CS, D, OR, S>(objs: OI, weights: WI) -> Self {
    Self { objs, weights }
  }
}

impl<'any, O, WI> MinWeightedSum<Iter<'any, O>, WI> {
  #[inline]
  pub fn from_gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>(
    mp: &'any Gp<D, HCRS, HCS, ORS, OS, SCRS, SCS, SS>,
    weights: WI,
  ) -> Self
  where
    OS: AsRef<[O]> + SingleTypeStorage<Item = O>,
  {
    Self { objs: mp.defs().objs().as_ref().iter(), weights }
  }
}

impl<O, OR, OI, S, WI> Obj<OR, S> for MinWeightedSum<OI, WI>
where
  O: Obj<OR, S>,
  OR: Add<Output = OR> + Div<Output = OR> + One + Sum,
  OI: Clone + Iterator<Item = O>,
  WI: Clone + Iterator<Item = OR>,
{
  #[inline]
  fn obj_direction(&self) -> ObjDirection {
    ObjDirection::Min
  }

  #[inline]
  fn result(&self, s: &S) -> OR {
    self
      .objs
      .clone()
      .zip(self.weights.clone())
      .map(|(o, w)| {
        let result = o.result(s);
        let transformed = match o.obj_direction() {
          ObjDirection::Max => OR::one() / (OR::one() + result),
          ObjDirection::Min => result,
        };
        transformed * w
      })
      .sum()
  }
}
