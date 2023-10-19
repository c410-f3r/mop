use num_traits::{Bounded, NumCast};

use crate::{Obj, ObjDirection};

#[derive(Debug)]
pub enum Either<L, R> {
  Left(L),
  Right(R),
}

impl<O, OO, OR, S> Obj<OR, S> for Either<O, OO>
where
  O: Obj<OR, S>,
  OO: Obj<OR, S>,
  OR: Bounded + NumCast,
{
  #[inline]
  fn obj_direction(&self) -> ObjDirection {
    match self {
      Either::Left(elem) => elem.obj_direction(),
      Either::Right(elem) => elem.obj_direction(),
    }
  }

  #[inline]
  fn result(&self, s: &S) -> OR {
    match self {
      Either::Left(elem) => elem.result(s),
      Either::Right(elem) => elem.result(s),
    }
  }
}
