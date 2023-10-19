mod random_domain_assignments;
mod swap;

pub use random_domain_assignments::RandomDomainAssignments;
pub use swap::Swap;

pub trait Mutation<M, T> {
  type Error;

  fn mutation(&self, misc: &M, source: &mut T) -> Result<(), Self::Error>;
}

impl<M, T> Mutation<M, T> for () {
  type Error = core::convert::Infallible;

  #[inline]
  fn mutation(&self, _: &M, _: &mut T) -> Result<(), Self::Error> {
    Ok(())
  }
}
