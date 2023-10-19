/// Solver
pub trait Solver<P> {
  /// Error
  type Error;

  /// Do solving work after stoping criteria verification.
  fn after_iter(&mut self, p: &mut P) -> Result<(), Self::Error>;

  /// Do solving work before stoping criteria verification.
  fn before_iter(&mut self, p: &mut P) -> Result<(), Self::Error>;

  /// Verifies or modifies `P` when solving was completed
  #[inline]
  fn finished(&mut self, _: &mut P) {}

  /// Verifies or modifies `P` when solving is starting
  #[inline]
  fn init(&mut self, _: &mut P) {}
}
