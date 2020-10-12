use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

#[cfg(not(feature = "with-futures"))]
pub type SolverFuture<'a, E> = Pin<Box<dyn Future<Output = Result<(), E>> + 'a>>;

#[cfg(feature = "with-futures")]
pub type SolverFuture<'a, E> = Pin<Box<dyn Future<Output = Result<(), E>> + Send + Sync + 'a>>;

pub trait Solver<P> {
  type Error;

  /// Do solving work after stoping criteria verification.
  fn after_iter<'a>(&'a mut self, p: &'a mut P) -> SolverFuture<'a, Self::Error>;

  /// Do solving work before stoping criteria verification.
  fn before_iter<'a>(&'a mut self, p: &'a mut P) -> SolverFuture<'a, Self::Error>;

  /// Verifies or modifies `P` when solving was completed
  fn finished(&mut self, _: &mut P) {}

  /// Verifies or modifies `P` when solving is starting
  fn init(&mut self, _: &mut P) {}
}
