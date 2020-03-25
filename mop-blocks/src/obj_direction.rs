use core::cmp::Ordering;

/// Objective direction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjDirection {
  /// Maximization
  Max,
  /// Minimization
  Min,
}

impl ObjDirection {
  pub fn partial_cmp<T>(self, a: &T, b: &T) -> Option<Ordering>
  where
    T: PartialOrd,
  {
    match self {
      ObjDirection::Max => a.partial_cmp(b),
      ObjDirection::Min => b.partial_cmp(a),
    }
  }
}
