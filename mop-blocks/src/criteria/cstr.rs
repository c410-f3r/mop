//! Constraint

use alloc::{boxed::Box, string::String};

/// Constraint
///
/// # Types
///
/// * `S`: Solution
pub trait Cstr<S> {
  fn reasons(&self, _: &S) -> String {
    String::new()
  }

  fn violations(&self, solution: &S) -> usize;
}

impl<C, S> Cstr<S> for &'_ C
where
  C: Cstr<S> + ?Sized,
{
  fn reasons(&self, solution: &S) -> String {
    (*self).reasons(solution)
  }

  fn violations(&self, solution: &S) -> usize {
    (*self).violations(solution)
  }
}

impl<C, S> Cstr<S> for Box<C>
where
  C: Cstr<S>,
{
  fn reasons(&self, solution: &S) -> String {
    self.as_ref().reasons(solution)
  }

  fn violations(&self, solution: &S) -> usize {
    self.as_ref().violations(solution)
  }
}

impl<S> Cstr<S> for fn(&S) -> usize {
  fn violations(&self, solution: &S) -> usize {
    self(solution)
  }
}

impl<S> Cstr<S> for (fn(&S) -> String, fn(&S) -> usize) {
  fn reasons(&self, solution: &S) -> String {
    self.0(solution)
  }

  fn violations(&self, solution: &S) -> usize {
    self.1(solution)
  }
}
