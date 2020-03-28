use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

#[cfg(not(feature = "with_futures"))]
pub type SolverFuture<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

#[cfg(feature = "with_futures")]
pub type SolverFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;

pub trait Solver<P> {
  /// Do solving work after stoping criteria verification.
  fn after_iter<'a>(&'a mut self, p: &'a mut P) -> SolverFuture<'a>;

  /// Do solving work before stoping criteria verification.
  fn before_iter<'a>(&'a mut self, p: &'a mut P) -> SolverFuture<'a>;

  fn finished(&mut self, _: &mut P) {}

  fn init(&mut self, _: &mut P) {}
}
