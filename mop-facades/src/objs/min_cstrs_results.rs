use alloc::vec::Vec;
use core::{fmt::Debug, marker::PhantomData};
use mop_blocks::{Cstr, Obj, ObjDirection};
use num_traits::NumCast;

#[derive(Debug)]
pub struct MinCstrsResults<C, OR, S> {
  obj_direction: ObjDirection,
  cstrs: Vec<C>,
  phantom: PhantomData<(OR, S)>,
}

impl<C, OR, S> MinCstrsResults<C, OR, S> {
  pub fn new(cstrs: Vec<C>) -> Self {
    let obj_direction = ObjDirection::Min;
    Self { obj_direction, phantom: PhantomData, cstrs }
  }
}

impl<C, OR, S> Obj<OR, S> for MinCstrsResults<C, OR, S>
where
  C: Cstr<S>,
  OR: Debug + NumCast,
{
  fn obj_direction(&self) -> ObjDirection {
    self.obj_direction
  }

  fn result(&self, s: &S) -> OR {
    min_cstrs_results(self.cstrs.iter(), s)
  }
}

pub fn min_cstrs_results<'a, C, OR, S>(cstrs: impl Iterator<Item = &'a C>, s: &S) -> OR
where
  C: Cstr<S> + 'a,
  OR: Debug + NumCast,
{
  let sum: usize = cstrs.map(|cstr| cstr.violations(s)).sum();
  OR::from(sum).unwrap()
}
