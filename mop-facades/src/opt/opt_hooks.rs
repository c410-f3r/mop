pub trait OptHooks<P> {
  fn init(&mut self);

  fn before_iter(&mut self, p: &mut P);

  fn after_iter(&mut self, p: &mut P);

  fn finished(&mut self);
}

impl<P> OptHooks<P> for () {
  fn before_iter(&mut self, _: &mut P) {}

  fn after_iter(&mut self, _: &mut P) {}

  fn finished(&mut self) {}

  fn init(&mut self) {}
}

impl<P> OptHooks<P> for (fn(&mut P), fn(&mut P), fn(), fn()) {
  fn before_iter(&mut self, p: &mut P) {
    (self.0)(p);
  }

  fn after_iter(&mut self, p: &mut P) {
    (self.1)(p);
  }

  fn finished(&mut self) {
    (self.2)();
  }

  fn init(&mut self) {
    (self.3)();
  }
}
