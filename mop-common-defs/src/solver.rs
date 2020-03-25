pub trait Solver {
  type Problem;

  /// Do solving work after stoping criteria verification.
  fn after_iter(&mut self, p: &mut Self::Problem);

  /// Do solving work before stoping criteria verification.
  fn before_iter(&mut self, p: &mut Self::Problem);

  fn finished(&mut self, _: &mut Self::Problem) {}

  fn init(&mut self, _: &mut Self::Problem) {}

  //fn problem(&self) -> &Self::Problem;
  //
  //fn problem_mut(&mut self) -> &mut Self::Problem;
}
