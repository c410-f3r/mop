use core::ops::{Deref, Div};
use num_traits::SaturatingMul;

/// A percent from 0u16 to 100u16.
///
/// Arithmetic operations won't result in values greater than u16::MAX.
#[derive(Clone, Copy, Debug)]
pub struct Pct(u16);

impl Pct {
  /// From percent representation (0 to 100).
  ///
  /// Values greater than 100 will be truncated to 100.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::Pct;
  /// assert_eq!(*Pct::from_percent(40), 40);
  /// ```
  #[inline]
  pub const fn from_percent(pct: u16) -> Self {
    Pct(pct)
  }

  /// Is In Random Probability?
  #[cfg(feature = "rand")]
  #[inline]
  pub fn is_in_rnd_pbty<R>(self, rng: &mut R) -> bool
  where
    R: rand::Rng,
  {
    let random = rng.gen::<u16>() % 100;
    random < self.0
  }

  #[inline]
  pub fn saturating_mul<T>(self, rhs: &T) -> T
  where
    T: Div<T, Output = T> + From<u16> + SaturatingMul,
  {
    let t: T = self.0.into();
    t.saturating_mul(rhs).div(100.into())
  }
}

impl AsRef<u16> for Pct {
  #[inline]
  fn as_ref(&self) -> &u16 {
    &self.0
  }
}

impl Deref for Pct {
  type Target = u16;

  #[inline]
  fn deref(&self) -> &u16 {
    &self.0
  }
}
