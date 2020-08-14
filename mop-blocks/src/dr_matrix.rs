mod dr_matrix_error;
mod dr_matrix_row_iter_impls;
mod dr_matrix_rows_constructor;

use alloc::vec::Vec;
use cl_traits::{ArrayWrapper, Clear, Storage, Truncate, WithCapacity};
use core::cmp::Ordering;
pub use {dr_matrix_error::*, dr_matrix_row_iter_impls::*, dr_matrix_rows_constructor::*};

pub type DrMatrixArray<DA> = DrMatrix<ArrayWrapper<DA>>;
pub type DrMatrixMut<'a, DATA> = DrMatrix<&'a mut [DATA]>;
pub type DrMatrixRef<'a, DATA> = DrMatrix<&'a [DATA]>;
pub type DrMatrixVec<T> = DrMatrix<Vec<T>>;
pub type Result<T> = core::result::Result<T, DrMatrixError>;

/// Dense Row Matrix
///
/// Dense matrix filled row-by-row, i.e., the traditional matrix that is generally taught
/// in class or used by dense linear algebra algorithms.
///
/// Tailored exclusively for storing purposes, doesn't provide any arithmetic method.
///
/// # Types
///
/// * `DS`: Data Storage
#[cfg_attr(feature = "with-serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct DrMatrix<DS> {
  pub(crate) data: DS,
  pub(crate) cols: usize,
  pub(crate) rows: usize,
}

impl<DS> DrMatrix<DS> {
  /// See [`DrMatrixRowsConstructor`](struct.DrMatrixRowsConstructor.html) for more information.
  pub fn constructor(&mut self) -> DrMatrixRowsConstructor<'_, DS> {
    DrMatrixRowsConstructor::new(&mut self.rows, self.cols, &mut self.data)
  }

  /// Clears the internal data and sets the number of rows to zero.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_vec;
  /// let mut dcca = dr_matrix_vec();
  /// dcca.clear();
  /// assert_eq!(dcca.cols(), 5);
  /// assert_eq!(dcca.data(), &[]);
  /// assert_eq!(dcca.rows(), 0);
  /// ```
  pub fn clear(&mut self)
  where
    DS: Clear,
  {
    self.data.clear();
    self.rows = 0;
  }

  /// The number of columns.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// assert_eq!(dr_matrix_array().cols(), 5);
  /// ```
  #[inline]
  pub fn cols(&self) -> usize {
    self.cols
  }

  /// The number of rows.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// assert_eq!(dr_matrix_array().rows(), 4);
  /// ```
  #[inline]
  pub fn rows(&self) -> usize {
    self.rows
  }

  /// Keeps the initial `until_row_idx` number of rows, cleaning the remaining data.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// ddma.truncate(2);
  /// assert_eq!(ddma.cols(), 5);
  /// assert_eq!(ddma.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  /// assert_eq!(ddma.rows(), 2);
  /// ```
  pub fn truncate(&mut self, until_row_idx: usize)
  where
    DS: Truncate<Input = usize>,
  {
    self.data.truncate(self.cols.saturating_mul(until_row_idx));
    self.rows = until_row_idx;
  }

  fn row_range(&self, row_idx: usize) -> Option<core::ops::Range<usize>> {
    let stride = self.stride(row_idx);
    if stride == usize::MAX {
      return None;
    }
    Some(stride..stride + self.cols)
  }

  #[inline]
  fn stride(&self, row_idx: usize) -> usize {
    self.cols.saturating_mul(row_idx)
  }
}

impl<DS> DrMatrix<DS>
where
  DS: WithCapacity<Input = usize>,
{
  /// Creates a new instance with an initial `rows` * `cols` capacity.
  ///
  /// The number of columns will be equal `cols` while the number of rows will be
  /// equal to zero.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::dr_matrix::DrMatrix;
  /// let matrix = DrMatrix::<Vec<i32>>::with_capacity(2, 3);
  /// assert_eq!(matrix.cols(), 3);
  /// assert_eq!(matrix.rows(), 0);
  /// ```
  pub fn with_capacity(rows: usize, cols: usize) -> Self {
    DrMatrix { data: DS::with_capacity(rows * cols), cols, rows: 0 }
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: AsRef<[DATA]> + Storage<Item = DATA>,
{
  /// Creates a new [`DrMatrix`](DrMatrix) from raw parameters.
  ///
  /// # Arguments
  ///
  /// * `shape` - An array containing the number of rows and columns.
  /// * `data` - The matrix data.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::dr_matrix::DrMatrixArray;
  /// let _ = DrMatrixArray::new(2, 4, [1, 2, 3, 4, 5, 6, 7, 8]);
  /// ```
  pub fn new<IDS>(rows: usize, cols: usize, into_data: IDS) -> Result<Self>
  where
    IDS: Into<DS>,
  {
    let data = into_data.into();
    if rows.saturating_mul(cols) != data.as_ref().len() {
      return Err(DrMatrixError::DataLenDiffColsTimesRows);
    }
    Ok(Self { data, rows, cols })
  }

  /// Converts the inner storage to a generic immutable slice storage.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::dr_matrix_array, dr_matrix::DrMatrixRef};
  /// assert_eq!(
  ///   Ok(dr_matrix_array().as_ref()),
  ///   DrMatrixRef::new(
  ///     4, 5,
  ///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20][..],
  ///   )
  /// );
  /// ```
  pub fn as_ref(&self) -> DrMatrixRef<'_, DATA> {
    DrMatrixRef { cols: self.cols, data: self.data.as_ref(), rows: self.rows }
  }

  /// Immutable slice of the internal data.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_vec;
  /// assert_eq!(
  ///   dr_matrix_vec().data(),
  ///   &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
  /// );
  /// ```
  pub fn data(&self) -> &[DATA] {
    &self.data.as_ref()
  }

  /// If `row_idx` is out of bounds, returns `None`. Otherwise, returns a slice
  /// representing data of the given row index.
  ///
  /// # Arguments
  ///
  /// * `row_idx`: Row index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// for row_idx in 0..4 {
  ///   let starting_row_value = row_idx * 5 + 1;
  ///   assert_eq!(
  ///     ddma.row(row_idx as usize),
  ///     Some(&[
  ///       starting_row_value,
  ///       starting_row_value + 1,
  ///       starting_row_value + 2,
  ///       starting_row_value + 3,
  ///       starting_row_value + 4
  ///     ][..])
  ///   );
  /// }
  /// ```
  pub fn row(&self, row_idx: usize) -> Option<&[DATA]> {
    self.data().get(self.row_range(row_idx)?)
  }

  pub fn row_iter(&self) -> DrMatrixRowIter<'_, DATA> {
    DrMatrixRowIter::new(self.rows(), self.cols, self.data().as_ref())
  }

  pub fn to_vec(&self) -> DrMatrixVec<DATA>
  where
    DATA: Clone,
  {
    DrMatrixVec { cols: self.cols, data: self.data.as_ref().to_vec(), rows: self.rows }
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// for row_idx in 0..4 {
  ///   let starting_row_value = row_idx * 5 + 1;
  ///   for col_idx in 0..5 {
  ///     let value = (starting_row_value + col_idx) as i32;
  ///     assert_eq!(ddma.value(row_idx, col_idx).copied(), Some(value));
  ///   }
  /// }
  /// ```
  pub fn value(&self, row_idx: usize, col_idx: usize) -> Option<&DATA> {
    self.data().get(self.stride(row_idx).saturating_add(col_idx))
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: AsMut<[DATA]> + Storage<Item = DATA>,
{
  pub fn data_mut(&mut self) -> &mut [DATA] {
    self.data.as_mut()
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// ddma.remove_row(2);
  /// assert_eq!(ddma.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 17, 18, 19, 20]);
  /// assert_eq!(ddma.rows(), 3);
  /// ```
  pub fn remove_row(&mut self, idx: usize)
  where
    DS: Truncate<Input = usize>,
  {
    assert!(idx < self.rows);
    let mut from_row_idx = idx;
    let mut to_row_idx = idx + 1;
    while to_row_idx < self.rows {
      self.swap_rows(from_row_idx, to_row_idx);
      from_row_idx += 1;
      to_row_idx += 1;
    }
    self.truncate(self.rows - 1);
  }

  pub fn row_mut(&mut self, row_idx: usize) -> Option<&mut [DATA]> {
    let row_range = self.row_range(row_idx)?;
    self.data_mut().get_mut(row_range)
  }

  pub fn row_iter_mut(&mut self) -> DrMatrixRowIterMut<'_, DATA> {
    DrMatrixRowIterMut::new(self.rows, self.cols, self.data.as_mut())
  }

  pub fn swap(&mut self, a: [usize; 2], b: [usize; 2]) -> bool
  where
    DS: AsRef<[DATA]>,
  {
    let a_data_idx = self.stride(a[0]).saturating_add(a[1]);
    let b_data_idx = self.stride(b[0]).saturating_add(b[1]);
    if self.data().get(a_data_idx).is_none() {
      return false;
    }
    if self.data().get(b_data_idx).is_none() {
      return false;
    }
    self.data_mut().swap(a_data_idx, b_data_idx);
    true
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let mut matrix = dr_matrix_array();
  /// let original_0_row = matrix.row(0).unwrap().to_vec();
  /// let original_3_row = matrix.row(3).unwrap().to_vec();
  /// matrix.swap_rows(0, 3);
  /// assert_eq!(matrix.row(0), Some(&original_3_row[..]));
  /// assert_eq!(matrix.row(3), Some(&original_0_row[..]));
  /// ```
  pub fn swap_rows(&mut self, a: usize, b: usize) -> bool {
    if let Some([first_row, second_row]) = self.two_rows_mut(a, b) {
      first_row.swap_with_slice(second_row);
      true
    } else {
      false
    }
  }

  pub fn two_rows_mut(&mut self, a: usize, b: usize) -> Option<[&mut [DATA]; 2]> {
    let [max, min] = match a.cmp(&b) {
      Ordering::Equal => return None,
      Ordering::Greater => [a, b],
      Ordering::Less => [b, a],
    };
    let max_stride = self.stride(max);
    let min_stride = self.stride(min);
    let (first, second) = self.data.as_mut().split_at_mut(max_stride);
    let first_range = min_stride..min_stride.saturating_add(self.cols);
    let second_range = ..self.cols;
    Some([first.get_mut(first_range)?, second.get_mut(second_range)?])
  }

  pub fn value_mut(&mut self, row_idx: usize, col_idx: usize) -> Option<&mut DATA> {
    let data_idx = self.stride(row_idx).saturating_add(col_idx);
    self.data_mut().get_mut(data_idx)
  }
}

#[cfg(feature = "with-rand")]
impl<DATA, DS> DrMatrix<DS>
where
  DS: Default + cl_traits::Push<Input = DATA> + Storage<Item = DATA>,
{
  pub fn new_random_with_rand<F, R>(rows: usize, cols: usize, rng: &mut R, mut cb: F) -> Self
  where
    F: FnMut(&mut R, usize, usize) -> DATA,
    R: rand::Rng,
  {
    let mut data = DS::default();
    for row in 0..rows {
      for col in 0..cols {
        data.push(cb(rng, row, col));
      }
    }
    DrMatrix { cols, data, rows }
  }
}
