mod objs_avg;
mod pareto_comparator;

pub use objs_avg::*;
pub use pareto_comparator::*;

pub trait QualityComparator<M, T>
where
  M: ?Sized,
{
  /// Is `a` better than `b`?
  fn is_better(&self, misc: &M, a: &T, b: &T) -> bool;
}
