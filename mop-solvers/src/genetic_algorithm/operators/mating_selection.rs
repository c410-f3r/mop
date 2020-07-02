mod tournament;

pub use tournament::Tournament;

pub trait MatingSelection<M, T>
where
  M: ?Sized,
{
  type Error;

  fn mating_selection(
    &self,
    misc: &M,
    source: &mut T,
    destination: &mut T,
    filling_num: usize,
  ) -> Result<(), Self::Error>;
}

impl<M, T> MatingSelection<M, T> for () {
  type Error = core::convert::Infallible;

  fn mating_selection(&self, _: &M, _: &mut T, _: &mut T, _: usize) -> Result<(), Self::Error> {
    Ok(())
  }
}
