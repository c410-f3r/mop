#[derive(Debug, PartialEq)]
pub enum DrMatrixError {
  /// The length of `data` isn't equal the number of rows times the number of columns.
  /// ```rust
  /// use mop_blocks::dr_matrix::DrMatrixRef;
  /// let _ = DrMatrixRef::new(2, 4, &[1, 2, 3][..]);
  /// ```
  DataLenDiffColsTimesRows,
}
