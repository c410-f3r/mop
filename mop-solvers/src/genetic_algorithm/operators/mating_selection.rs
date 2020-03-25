mod tournament;

pub use self::tournament::Tournament;

pub trait MatingSelection<M, T>
where
  M: ?Sized,
{
  fn mating_selection(&self, misc: &M, source: &mut T, destination: &mut T, filling_num: usize);
}

impl<M, T> MatingSelection<M, T> for () {
  fn mating_selection(&self, _: &M, _: &mut T, _: &mut T, _: usize) {}
}
