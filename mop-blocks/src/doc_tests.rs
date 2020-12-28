//! Instances for documentation tests.

use crate::dr_matrix::{DrMatrixArray, DrMatrixVec};

/// ```rust
/// //  ___________________
/// // | 1 | 2 | 3 | 4 | 5 |
/// // |___|___|___|___|___|
/// // | 6 | 7 | 8 | 9 | 10|
/// // |___|___|___|___|___|
/// // | 11| 12| 13| 14| 15|
/// // |___|___|___|___|___|
/// // | 16| 17| 18| 19| 20|
/// // |   |   |   |   |   |
/// //  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
/// ```
#[inline]
pub fn dr_matrix_array() -> DrMatrixArray<i32, 20> {
  DrMatrixArray {
    cols: 5,
    data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
    rows: 4,
  }
}

/// `Vec` version of `dr_matrix_array`.
#[inline]
pub fn dr_matrix_vec() -> DrMatrixVec<i32> {
  let dr_matrix_array = dr_matrix_array();
  DrMatrixVec {
    cols: dr_matrix_array.cols(),
    data: dr_matrix_array.data().to_vec(),
    rows: dr_matrix_array.rows(),
  }
}

/// Empty `Vec` version of `dr_matrix_array`.
#[inline]
pub fn capacited_dr_matrix_vec() -> DrMatrixVec<i32> {
  let dr_matrix_array = dr_matrix_array();
  DrMatrixVec::with_capacity(dr_matrix_array.rows(), dr_matrix_array.cols())
}
