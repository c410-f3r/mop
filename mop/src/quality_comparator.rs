mod nsga2;
mod objs_avg;

pub use nsga2::*;
pub use objs_avg::*;

pub trait QualityComparator<M, T>
where
  M: ?Sized,
{
  /// Is `a` better than `b`?
  fn is_better(&self, misc: &M, a: &T, b: &T) -> bool;
}
