use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

#[cfg(not(feature = "with_futures"))]
pub type SolverFuture<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

#[cfg(feature = "with_futures")]
pub type SolverFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;

pub trait Solver {
  type Problem;

  /// Do solving work after stoping criteria verification.
  fn after_iter<'a>(&'a mut self, p: &'a mut Self::Problem) -> SolverFuture<'a>;

  /// Do solving work before stoping criteria verification.
  fn before_iter<'a>(&'a mut self, p: &'a mut Self::Problem) -> SolverFuture<'a>;

  fn finished(&mut self, _: &mut Self::Problem) {}

  fn init(&mut self, _: &mut Self::Problem) {}
}
