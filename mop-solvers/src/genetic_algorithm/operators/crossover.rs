mod multi_point;

pub use self::multi_point::MultiPoint;

pub trait Crossover<T> {
  fn crossover(&self, source: &mut T, destination: &mut T, filling_num: usize);
}

impl<T> Crossover<T> for () {
  fn crossover(&self, _: &mut T, _: &mut T, _: usize) {}
}
