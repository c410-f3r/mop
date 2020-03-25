pub trait OptHooks<S> {
  fn init(&mut self);

  fn before_iter(&mut self, s: &mut S);

  fn after_iter(&mut self, s: &mut S);

  fn finished(&mut self);
}

impl<S> OptHooks<S> for () {
  fn before_iter(&mut self, _: &mut S) {}

  fn after_iter(&mut self, _: &mut S) {}

  fn finished(&mut self) {}

  fn init(&mut self) {}
}

impl<S> OptHooks<S> for (fn(&mut S), fn(&mut S), fn(), fn()) {
  fn before_iter(&mut self, s: &mut S) {
    (self.0)(s);
  }

  fn after_iter(&mut self, s: &mut S) {
    (self.1)(s);
  }

  fn finished(&mut self) {
    (self.2)();
  }

  fn init(&mut self) {
    (self.3)();
  }
}
