use core::ops::Deref;

/// Percent in decimal format
#[derive(Clone, Copy, Debug)]
pub struct Pct(f64);

impl Pct {
  /// From decimal representation.
  pub fn from_decimal<T>(pct: T) -> Self
  where
    T: Into<f64>,
  {
    Pct(pct.into())
  }

  /// From percent representation.
  pub fn from_percent<T>(pct: T) -> Self
  where
    T: Into<f64>,
  {
    Pct(pct.into() / 100.0)
  }

  /// Is In Random Probability?
  #[cfg(feature = "with_rand")]
  pub fn is_in_rnd_pbty<R>(self, rng: &mut R) -> bool
  where
    R: rand::Rng,
  {
    rng.gen::<f64>() < self.0
  }
}

impl Deref for Pct {
  type Target = f64;
  fn deref(&self) -> &f64 {
    &self.0
  }
}
