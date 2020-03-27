#[cfg(all(feature = "with_rand", test))]
mod dr_matrix_quickcheck;
mod dr_matrix_row_constructor;
mod dr_matrix_row_iter_impls;

use alloc::vec::Vec;
use cl_traits::{ArrayWrapper, Clear, Push, Storage, Truncate, WithCapacity};
use core::{cmp::Ordering, iter::Extend};
pub use dr_matrix_row_constructor::*;
pub use dr_matrix_row_iter_impls::*;

/// Dense Row Matrix
///
/// Dense matrix filled row-by-row, i.e., the tradicional matrix that is generally taught
/// in class or used by dense linear algebra algorithms.
///
/// Tailored exclusively for storing purposes, doesn't provide any arithmetic method.
///
/// # Types
///
/// * `DS`: Data Storage
#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct DrMatrix<DS> {
  data: DS,
  cols: usize,
  rows: usize,
}

pub type DrMatrixArray<DA> = DrMatrix<ArrayWrapper<DA>>;
pub type DrMatrixMut<'a, DATA> = DrMatrix<&'a mut [DATA]>;
pub type DrMatrixRef<'a, DATA> = DrMatrix<&'a [DATA]>;
pub type DrMatrixVec<T> = DrMatrix<Vec<T>>;

impl<DS> DrMatrix<DS>
where
  DS: WithCapacity<Input = usize>,
{
  pub fn with_capacity(rows: usize, cols: usize) -> Self {
    DrMatrix { data: DS::with_capacity(rows * cols), cols, rows: 0 }
  }
}

impl<DS> DrMatrix<DS> {
  #[inline]
  pub fn cols(&self) -> usize {
    self.cols
  }

  #[inline]
  pub fn rows(&self) -> usize {
    self.rows
  }

  #[inline]
  pub fn stride(&self, idx: usize) -> usize {
    self.cols() * idx
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
  ///
  /// # Assertions
  ///
  /// * The length of `data` must be equal the number of rows times the number of columns.
  ///
  /// ```should_panic
  /// use mop_blocks::dr_matrix::DrMatrixRef;
  /// let _ = DrMatrixRef::new(2, 4, &[1, 2, 3][..]);
  /// ```
  pub fn new<IDS>(rows: usize, cols: usize, into_data: IDS) -> Self
  where
    IDS: Into<DS>,
  {
    let data = into_data.into();
    assert!(rows * cols == data.as_ref().len());
    Self { data, rows, cols }
  }

  /// Converts the inner storage to a generic immutable slice storage.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::dr_matrix_array, dr_matrix::DrMatrixRef};
  /// assert_eq!(
  ///   dr_matrix_array().as_ref(),
  ///   DrMatrixRef::new(
  ///     4, 5,
  ///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20][..],
  ///   )
  /// );
  /// ```
  pub fn as_ref(&self) -> DrMatrixRef<'_, DATA> {
    DrMatrix::new(self.rows, self.cols, self.data.as_ref())
  }

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

  pub fn data(&self) -> &[DATA] {
    &self.data.as_ref()
  }

  pub fn extend<'b>(&mut self, other: &'b DrMatrix<DS>)
  where
    DATA: Copy + 'b,
    DS: Extend<&'b DATA>,
  {
    assert!(self.cols == other.cols);
    self.data.extend(other.data.as_ref());
    self.rows += other.rows();
  }

  pub fn extend_from_clone<'b>(&mut self, other: &'b DrMatrix<DS>)
  where
    DATA: Clone + 'b,
    DS: Extend<&'b DATA>,
  {
    assert!(self.cols == other.cols);
    self.data.extend(other.data.as_ref());
    self.rows += other.rows();
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// for row_idx in 0..4 {
  ///   let starting_row_value = row_idx * 5 + 1;
  ///   assert_eq!(
  ///     ddma.row(row_idx as usize),
  ///     &[
  ///       starting_row_value,
  ///       starting_row_value + 1,
  ///       starting_row_value + 2,
  ///       starting_row_value + 3,
  ///       starting_row_value + 4
  ///     ]
  ///   );
  /// }
  /// ```
  pub fn row(&self, idx: usize) -> &[DATA] {
    let stride = self.stride(idx);
    &self.data()[stride..stride + self.cols]
  }

  pub fn row_iter(&self) -> DrMatrixRowIter<'_, DATA> {
    DrMatrixRowIter::new(self.rows(), self.cols, self.data().as_ptr())
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// for row_idx in 0..4 {
  ///   let starting_row_value = row_idx * 5 + 1;
  ///   for col_idx in 0..5 {
  ///     assert_eq!(*ddma.value(row_idx as usize, col_idx as usize), starting_row_value + col_idx);
  ///   }
  /// }
  /// ```
  pub fn value(&self, row_idx: usize, col_idx: usize) -> &DATA {
    &self.data()[self.stride(row_idx) + col_idx]
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: AsMut<[DATA]> + Storage<Item = DATA>,
{
  pub fn data_mut(&mut self) -> &mut [DATA] {
    self.data.as_mut()
  }

  pub fn row_mut(&mut self, idx: usize) -> &mut [DATA] {
    let stride = self.stride(idx);
    &mut self.data.as_mut()[stride..stride + self.cols]
  }

  pub fn row_iter_mut(&mut self) -> DrMatrixRowIterMut<'_, DATA> {
    DrMatrixRowIterMut::new(self.rows, self.cols, self.data_mut().as_mut_ptr())
  }

  pub fn swap(&mut self, a: [usize; 2], b: [usize; 2]) {
    let a_data_idx = self.stride(a[0]) + a[1];
    let b_data_idx = self.stride(b[0]) + b[1];
    self.data_mut().swap(a_data_idx, b_data_idx)
  }

  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_array;
  /// let mut matrix = dr_matrix_array();
  /// let original_0_row = matrix.row(0).to_vec();
  /// let original_3_row = matrix.row(3).to_vec();
  /// matrix.swap_rows(0, 3);
  /// assert_eq!(matrix.row(0), &original_3_row[..]);
  /// assert_eq!(matrix.row(3), &original_0_row[..]);
  /// ```
  pub fn swap_rows(&mut self, a: usize, b: usize) {
    let [max, min] = match a.cmp(&b) {
      Ordering::Equal => return,
      Ordering::Greater => [a, b],
      Ordering::Less => [b, a],
    };
    let cols = self.cols;
    let (left, right) = self.data_mut().split_at_mut(cols * max);
    let left_start = cols * min;
    left[left_start..left_start + cols].swap_with_slice(&mut right[0..cols]);
  }

  pub fn two_rows_mut(&mut self, smaller_idx: usize, bigger_idx: usize) -> [&mut [DATA]; 2] {
    let bigger = self.stride(bigger_idx);
    let smaller = self.stride(smaller_idx);
    let (first, second) = self.data.as_mut().split_at_mut(bigger);
    [&mut first[smaller..smaller + self.cols], &mut second[0..self.cols]]
  }

  pub fn value_mut(&mut self, row_idx: usize, col_idx: usize) -> &mut DATA {
    let stride = self.stride(row_idx);
    &mut self.data_mut()[stride + col_idx]
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: Push<Input = DATA> + Storage<Item = DATA>,
{
  pub fn fill<F>(&mut self, mut cb: F)
  where
    F: FnMut(usize, usize) -> DATA,
  {
    for row in 0..self.rows {
      for col in 0..self.cols {
        self.data.push(cb(row, col));
      }
    }
  }

  pub fn row_constructor(&mut self) -> DrMatrixRowConstructor<'_, DS> {
    DrMatrixRowConstructor::new(&mut self.rows, self.cols, &mut self.data)
  }
}
impl<DATA, DS> DrMatrix<DS>
where
  DS: AsMut<[DATA]> + Storage<Item = DATA> + Truncate<Input = usize>,
{
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// ddma.remove_row(2);
  /// assert_eq!(ddma.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 17, 18, 19, 20]);
  /// assert_eq!(ddma.rows(), 3);
  /// ```
  pub fn remove_row(&mut self, idx: usize) {
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
}

impl<DS> DrMatrix<DS>
where
  DS: Truncate<Input = usize>,
{
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
  pub fn truncate(&mut self, until_row_idx: usize) {
    self.data.truncate(self.cols * until_row_idx);
    self.rows = until_row_idx;
  }
}

#[cfg(feature = "with_rand")]
impl<DATA, DS> DrMatrix<DS>
where
  DS: Default + Push<Input = DATA> + Storage<Item = DATA>,
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
