//! Constraint

use alloc::{boxed::Box, string::String};

/// Constraint
///
/// # Types
///
/// * `S`: Solution
pub trait Cstr<S> {
  #[inline]
  fn reasons(&self, _: &S) -> String {
    String::new()
  }

  fn violations(&self, solution: &S) -> usize;
}

impl<C, S> Cstr<S> for &'_ C
where
  C: Cstr<S> + ?Sized,
{
  #[inline]
  fn reasons(&self, solution: &S) -> String {
    (*self).reasons(solution)
  }

  #[inline]
  fn violations(&self, solution: &S) -> usize {
    (*self).violations(solution)
  }
}

impl<C, S> Cstr<S> for Box<C>
where
  C: Cstr<S>,
{
  #[inline]
  fn reasons(&self, solution: &S) -> String {
    self.as_ref().reasons(solution)
  }

  #[inline]
  fn violations(&self, solution: &S) -> usize {
    self.as_ref().violations(solution)
  }
}

impl<S> Cstr<S> for fn(&S) -> usize {
  #[inline]
  fn violations(&self, solution: &S) -> usize {
    self(solution)
  }
}

impl<S> Cstr<S> for (fn(&S) -> String, fn(&S) -> usize) {
  #[inline]
  fn reasons(&self, solution: &S) -> String {
    self.0(solution)
  }

  #[inline]
  fn violations(&self, solution: &S) -> usize {
    self.1(solution)
  }
}
