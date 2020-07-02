use crate::ObjDirection;
use alloc::boxed::Box;
use mop_common::TraitCfg;

/// Objective
///
/// # Types
///
/// * `OR`: Objective Result
/// * `S`: Solution
pub trait Obj<OR, S>: TraitCfg {
  fn obj_direction(&self) -> ObjDirection;

  fn result(&self, solution: &S) -> OR;
}

impl<O, OR, S> Obj<OR, S> for &'_ O
where
  O: Obj<OR, S> + ?Sized,
{
  fn obj_direction(&self) -> ObjDirection {
    (*self).obj_direction()
  }

  fn result(&self, solution: &S) -> OR {
    (*self).result(solution)
  }
}

impl<O, OR, S> Obj<OR, S> for Box<O>
where
  O: Obj<OR, S>,
{
  fn obj_direction(&self) -> ObjDirection {
    self.as_ref().obj_direction()
  }

  fn result(&self, solution: &S) -> OR {
    self.as_ref().result(solution)
  }
}

impl<OR, S> Obj<OR, S> for (ObjDirection, fn(&S) -> OR) {
  fn obj_direction(&self) -> ObjDirection {
    self.0
  }

  fn result(&self, solution: &S) -> OR {
    (self.1)(solution)
  }
}
