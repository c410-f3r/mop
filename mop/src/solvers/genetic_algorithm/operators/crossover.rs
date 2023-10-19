mod multi_point;

pub use multi_point::MultiPoint;

pub trait Crossover<T> {
  type Error;

  fn crossover(
    &self,
    source: &mut T,
    destination: &mut T,
    filling_num: usize,
  ) -> Result<(), Self::Error>;
}

impl<T> Crossover<T> for () {
  type Error = core::convert::Infallible;

  #[inline]
  fn crossover(&self, _: &mut T, _: &mut T, _: usize) -> Result<(), Self::Error> {
    Ok(())
  }
}
