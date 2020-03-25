//! Constraint

use alloc::string::String;

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

impl<S> Cstr<S> for () {
  fn violations(&self, _: &S) -> usize {
    core::usize::MAX
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
