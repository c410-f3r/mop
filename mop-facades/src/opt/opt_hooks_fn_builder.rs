use core::fmt;

type OptHooksFn<P> = (fn(&mut P), fn(&mut P), fn(), fn());

#[derive(Debug)]
pub struct OptHooksFnBuilder<P> {
  after_opt_move: Option<FnWrapper<P>>,
  before_opt_move: Option<FnWrapper<P>>,
  finished: Option<fn()>,
  init: Option<fn()>,
}

impl<P> OptHooksFnBuilder<P> {
  #[inline]
  pub fn build(self) -> OptHooksFn<P> {
    (
      self.after_opt_move.unwrap_or_default().0,
      self.before_opt_move.unwrap_or_default().0,
      self.finished.unwrap_or_else(|| || {}),
      self.init.unwrap_or_else(|| || {}),
    )
  }

  #[inline]
  pub fn after_iter(mut self, after_opt_move: fn(&mut P)) -> Self {
    self.after_opt_move = Some(FnWrapper(after_opt_move));
    self
  }

  #[inline]
  pub fn before_iter(mut self, before_opt_move: fn(&mut P)) -> Self {
    self.before_opt_move = Some(FnWrapper(before_opt_move));
    self
  }

  #[inline]
  pub fn finished(mut self, finished: fn()) -> Self {
    self.finished = Some(finished);
    self
  }

  #[inline]
  pub fn init(mut self, init: fn()) -> Self {
    self.init = Some(init);
    self
  }
}

impl<P> Default for OptHooksFnBuilder<P> {
  #[inline]
  fn default() -> Self {
    Self { after_opt_move: None, before_opt_move: None, finished: None, init: None }
  }
}

struct FnWrapper<P>(fn(&mut P));

impl<P> fmt::Debug for FnWrapper<P> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[allow(
      // Harmless conversion
      clippy::as_conversions
    )]
    fmt::Pointer::fmt(&(self.0 as *const fn(&mut P)), f)
  }
}

impl<P> Default for FnWrapper<P> {
  fn default() -> Self {
    FnWrapper(|_| {})
  }
}
