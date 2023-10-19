pub trait OptHooks<P> {
  fn init(&mut self);

  fn before_iter(&mut self, p: &mut P);

  fn after_iter(&mut self, p: &mut P);

  fn finished(&mut self);
}

impl<P> OptHooks<P> for () {
  #[inline]
  fn before_iter(&mut self, _: &mut P) {}

  #[inline]
  fn after_iter(&mut self, _: &mut P) {}

  #[inline]
  fn finished(&mut self) {}

  #[inline]
  fn init(&mut self) {}
}

impl<P> OptHooks<P> for (fn(&mut P), fn(&mut P), fn(), fn()) {
  #[inline]
  fn before_iter(&mut self, p: &mut P) {
    (self.0)(p);
  }

  #[inline]
  fn after_iter(&mut self, p: &mut P) {
    (self.1)(p);
  }

  #[inline]
  fn finished(&mut self) {
    (self.2)();
  }

  #[inline]
  fn init(&mut self) {
    (self.3)();
  }
}
