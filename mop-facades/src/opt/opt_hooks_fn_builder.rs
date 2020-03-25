use core::fmt;

type OptHooksFn<S> = (fn(&mut S), fn(&mut S), fn(), fn());

#[derive(Debug)]
pub struct OptHooksFnBuilder<S> {
  after_opt_move: Option<FnWrapper<S>>,
  before_opt_move: Option<FnWrapper<S>>,
  finished: Option<fn()>,
  init: Option<fn()>,
}

impl<S> OptHooksFnBuilder<S> {
  pub fn build(self) -> OptHooksFn<S> {
    (
      self.after_opt_move.unwrap_or_default().0,
      self.before_opt_move.unwrap_or_default().0,
      self.finished.unwrap_or_else(|| || {}),
      self.init.unwrap_or_else(|| || {}),
    )
  }

  pub fn after_iter(mut self, after_opt_move: fn(&mut S)) -> Self {
    self.after_opt_move = Some(FnWrapper(after_opt_move));
    self
  }

  pub fn before_iter(mut self, before_opt_move: fn(&mut S)) -> Self {
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

impl<S> Default for OptHooksFnBuilder<S> {
  fn default() -> Self {
    Self { after_opt_move: None, before_opt_move: None, finished: None, init: None }
  }
}

struct FnWrapper<S>(fn(&mut S));

impl<S> fmt::Debug for FnWrapper<S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Pointer::fmt(&(self.0 as *const ()), f)
  }
}

impl<S> Default for FnWrapper<S> {
  fn default() -> Self {
    FnWrapper(|_| {})
  }
}
