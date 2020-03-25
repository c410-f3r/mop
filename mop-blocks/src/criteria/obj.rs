use crate::ObjDirection;

/// Objective
///
/// # Types
///
/// * `OR`: Objective Result
/// * `S`: Solution
pub trait Obj<OR, S> {
  fn obj_direction(&self) -> ObjDirection;

  fn result(&self, solution: &S) -> OR;
}

impl<OR, S> Obj<OR, S> for (ObjDirection, fn(&S) -> OR) {
  fn obj_direction(&self) -> ObjDirection {
    self.0
  }

  fn result(&self, solution: &S) -> OR {
    (self.1)(solution)
  }
}
