use core::fmt;

/// Errors related to the DrMatrix module.
#[derive(Debug, PartialEq)]
pub enum DrMatrixError {
  /// The length of `data` isn't equal the number of rows times the number of columns.
  ///
  /// ```rust
  /// use mop_blocks::dr_matrix::DrMatrixRef;
  /// let _ = DrMatrixRef::new(2, 4, &[1, 2, 3][..]);
  /// ```
  DataLenDiffColsTimesRows,

  /// The capacity of `data` isn't enough to store all desired elements.
  ///
  /// ```rust
  /// use mop_blocks::dr_matrix::DrMatrixVec;
  /// let _ = DrMatrixVec::new(999, 999, vec![1, 2, 3]);
  /// ```
  NotEnoughCapacity,
}

impl fmt::Display for DrMatrixError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::DataLenDiffColsTimesRows => write!(f, "Data length differs from columns times rows"),
      Self::NotEnoughCapacity => write!(f, "Not enough capacity"),
    }
  }
}
