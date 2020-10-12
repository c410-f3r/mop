use core::fmt;

type OptHooksFn<'a, P> = (fn(&'a mut P), fn(&'a mut P), fn(), fn());

#[derive(Debug)]
pub struct OptHooksFnBuilder<'a, P> {
  after_opt_move: Option<FnWrapper<'a, P>>,
  before_opt_move: Option<FnWrapper<'a, P>>,
  finished: Option<fn()>,
  init: Option<fn()>,
}

impl<'a, P> OptHooksFnBuilder<'a, P> {
  pub fn build(self) -> OptHooksFn<'a, P> {
    (
      self.after_opt_move.unwrap_or_default().0,
      self.before_opt_move.unwrap_or_default().0,
      self.finished.unwrap_or_else(|| || {}),
      self.init.unwrap_or_else(|| || {}),
    )
  }

  pub fn after_iter(mut self, after_opt_move: fn(&mut P)) -> Self {
    self.after_opt_move = Some(FnWrapper(after_opt_move));
    self
  }

  pub fn before_iter(mut self, before_opt_move: fn(&mut P)) -> Self {
    self.before_opt_move = Some(FnWrapper(before_opt_move));
    self
  }

  pub fn finished(mut self, finished: fn()) -> Self {
    self.finished = Some(finished);
    self
  }

  pub fn init(mut self, init: fn()) -> Self {
    self.init = Some(init);
    self
  }
}

impl<'a, P> Default for OptHooksFnBuilder<'a, P> {
  fn default() -> Self {
    Self { after_opt_move: None, before_opt_move: None, finished: None, init: None }
  }
}

struct FnWrapper<'a, P>(fn(&'a mut P));

impl<'a, P> fmt::Debug for FnWrapper<'a, P> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Pointer::fmt(&self.0, f)
  }
}

impl<'a, P> Default for FnWrapper<'a, P> {
  fn default() -> Self {
    FnWrapper(|_| {})
  }
}
