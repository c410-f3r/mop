mod random_domain_assignments;
mod swap;

pub use self::{random_domain_assignments::RandomDomainAssignments, swap::Swap};

pub trait Mutation<M, T> {
  fn mutation(&self, misc: &M, source: &mut T);
}

impl<M, T> Mutation<M, T> for () {
  fn mutation(&self, _: &M, _: &mut T) {}
}
