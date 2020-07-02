use core::{
  ops::Deref,
  ops::{Div, Mul},
};
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
  /// use mop_blocks::Pct;
  /// assert_eq!(*Pct::from_percent(40), 40);
  /// ```
  pub fn from_percent(pct: u16) -> Self {
    Pct(pct)
  }

  /// Is In Random Probability?
  #[cfg(feature = "with-rand")]
  pub fn is_in_rnd_pbty<R>(self, rng: &mut R) -> bool
  where
    R: rand::Rng,
  {
    let random = rng.gen::<u16>() % 100;
    random < self.0
  }
}

impl AsRef<u16> for Pct {
  fn as_ref(&self) -> &u16 {
    &self.0
  }
}

impl Deref for Pct {
  type Target = u16;
  fn deref(&self) -> &u16 {
    &self.0
  }
}

/// # Example
///
/// ```rust
/// use mop_blocks::Pct;
/// assert_eq!(Pct::from_percent(20) * 10, 2);
/// ```
impl<T> Mul<T> for Pct
where
  T: Div<T, Output = T> + From<u16> + SaturatingMul,
{
  type Output = T;

  fn mul(self, rhs: T) -> T {
    let t: T = self.0.into();
    t.saturating_mul(&rhs).div(100.into())
  }
}
