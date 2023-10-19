mod dr_matrix_error;
mod dr_matrix_row_iter_impls;
#[cfg(feature = "rayon")]
mod dr_matrix_row_par_iter_impls;
mod dr_matrix_rows_constructor;

use alloc::vec::Vec;
use cl_aux::{Clear, SingleTypeStorage, Truncate, WithCapacity};
use core::cmp::Ordering;
pub use dr_matrix_error::*;
pub use dr_matrix_row_iter_impls::*;
pub use dr_matrix_rows_constructor::*;

pub type DrMatrixArray<DATA, const D: usize> = DrMatrix<[DATA; D]>;
pub type DrMatrixMut<'any, DATA, const D: usize> = DrMatrix<&'any mut [DATA]>;
pub type DrMatrixRef<'any, DATA> = DrMatrix<&'any [DATA]>;
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
/// * `DS`: Data SingleTypeStorage
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct DrMatrix<DS> {
  pub(crate) cols: usize,
  pub(crate) data: DS,
  pub(crate) rows: usize,
}

impl<DS> DrMatrix<DS> {
  /// See [`DrMatrixRowsConstructor`](struct.DrMatrixRowsConstructor.html) for more information.
  #[inline]
  pub fn constructor(&mut self) -> DrMatrixRowsConstructor<'_, DS> {
    DrMatrixRowsConstructor::new(&mut self.rows, self.cols, &mut self.data)
  }

  /// Clears the internal data and sets the number of rows to zero.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_vec;
  /// let mut dcca = dr_matrix_vec();
  /// dcca.clear();
  /// assert_eq!(dcca.cols(), 5);
  /// assert_eq!(dcca.data(), &[]);
  /// assert_eq!(dcca.rows(), 0);
  /// ```
  #[inline]
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
  /// use mop::doc_tests::dr_matrix_array;
  /// assert_eq!(dr_matrix_array().cols(), 5);
  /// ```
  #[inline]
  pub const fn cols(&self) -> usize {
    self.cols
  }

  /// The number of rows.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
  /// assert_eq!(dr_matrix_array().rows(), 4);
  /// ```
  #[inline]
  pub const fn rows(&self) -> usize {
    self.rows
  }

  /// Keeps the initial `until_row_idx` number of rows, cleaning the remaining data.
  ///
  /// # Argument
  ///
  /// * `rows` - The number of rows
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// ddma.truncate(2);
  /// assert_eq!(ddma.cols(), 5);
  /// assert_eq!(ddma.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  /// assert_eq!(ddma.rows(), 2);
  /// ```
  #[inline]
  pub fn truncate(&mut self, rows: usize)
  where
    DS: Truncate<Input = usize>,
  {
    if rows >= self.rows {
      return;
    }
    self.data.truncate(self.cols.saturating_mul(rows));
    self.rows = rows;
  }

  #[inline]
  const fn row_range(&self, row_idx: usize) -> Option<core::ops::Range<usize>> {
    let stride = self.stride(row_idx);
    if stride == usize::MAX {
      return None;
    }
    Some(stride..stride + self.cols)
  }

  #[inline]
  const fn stride(&self, row_idx: usize) -> usize {
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
  /// # Arguments
  ///
  /// * `rows` - Number of rows
  /// * `cols` - Number of columns
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::dr_matrix::DrMatrix;
  /// let matrix = DrMatrix::<Vec<i32>>::with_capacity(2, 3);
  /// assert_eq!(matrix.cols(), 3);
  /// assert_eq!(matrix.rows(), 0);
  /// ```
  #[inline]
  pub fn with_capacity(rows: usize, cols: usize) -> Self {
    DrMatrix { data: DS::with_capacity(rows * cols), cols, rows: 0 }
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: AsRef<[DATA]> + SingleTypeStorage<Item = DATA>,
{
  /// Creates a new [`DrMatrix`](DrMatrix) from raw parameters.
  ///
  /// # Arguments
  ///
  /// * `[rows, cols]` - An array containing the number of rows and columns.
  /// * `data` - The matrix data.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::dr_matrix::DrMatrixArray;
  /// let _ = DrMatrixArray::new([2, 4], [1, 2, 3, 4, 5, 6, 7, 8]);
  /// ```
  #[inline]
  pub fn new([rows, cols]: [usize; 2], data: DS) -> crate::Result<Self> {
    if rows.saturating_mul(cols) != data.as_ref().len() {
      return Err(DrMatrixError::DataLenDiffColsTimesRows.into());
    }
    Ok(Self { cols, data, rows })
  }

  /// Converts the inner storage to a generic immutable slice storage.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::{doc_tests::dr_matrix_array, dr_matrix::DrMatrixRef};
  /// assert_eq!(
  ///   Ok(dr_matrix_array().as_ref()),
  ///   DrMatrixRef::new(
  ///     [4, 5],
  ///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20][..],
  ///   )
  /// );
  /// ```
  #[inline]
  pub fn as_ref(&self) -> DrMatrixRef<'_, DATA> {
    DrMatrixRef { cols: self.cols, data: self.data.as_ref(), rows: self.rows }
  }

  /// Immutable slice of the internal data.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_vec;
  /// assert_eq!(
  ///   dr_matrix_vec().data(),
  ///   &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
  /// );
  /// ```
  #[inline]
  pub fn data(&self) -> &[DATA] {
    self.data.as_ref()
  }

  /// If `row_idx` is out of bounds, returns `None`. Otherwise, returns a slice
  /// representing data of the given row index.
  ///
  /// # Argument
  ///
  /// * `row_idx`: Row index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
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
  #[inline]
  pub fn row(&self, row_idx: usize) -> Option<&[DATA]> {
    self.data().get(self.row_range(row_idx)?)
  }

  /// Iterator where each element is a row slice.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// let mut ri = ddma.row_iter();
  /// assert_eq!(ri.next(), Some(&[1, 2, 3, 4, 5][..]));
  /// assert_eq!(ri.next(), Some(&[6, 7, 8, 9, 10][..]));
  /// assert_eq!(ri.next(), Some(&[11, 12, 13, 14, 15][..]));
  /// assert_eq!(ri.next(), Some(&[16, 17, 18, 19, 20][..]));
  /// assert_eq!(ri.next(), None);
  /// ```
  #[inline]
  pub fn row_iter(&self) -> DrMatrixRowIter<'_, DATA> {
    DrMatrixRowIter::new([self.rows, self.cols], self.data())
  }

  #[inline]
  #[cfg(feature = "rayon")]
  pub fn row_par_iter(&self) -> crate::ParallelIteratorWrapper<DrMatrixRowIter<'_, DATA>> {
    crate::ParallelIteratorWrapper(self.row_iter())
  }

  /// Copies the internal data into a heap allocated `Vec` storage and
  /// returns it as owned value.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::{doc_tests::dr_matrix_array, dr_matrix::DrMatrixVec};
  /// let ddma = dr_matrix_array();
  /// assert_eq!(
  ///   DrMatrixVec::new(
  ///     [ddma.rows(), ddma.cols()],
  ///     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
  ///   ),
  ///   Ok(ddma.to_vec()),
  /// );
  /// ```
  #[inline]
  pub fn to_vec(&self) -> DrMatrixVec<DATA>
  where
    DATA: Clone,
  {
    DrMatrixVec { cols: self.cols, data: self.data.as_ref().to_vec(), rows: self.rows }
  }

  /// If `row_idx` and `col_idx` are within bounds, returns a value reference. Otherwise,
  /// returns None.
  ///
  /// # Arguments
  ///
  /// * `row_idx` - Row index
  /// * `col_idx` - Column index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
  /// let ddma = dr_matrix_array();
  /// for row_idx in 0..4 {
  ///   let starting_row_value = row_idx * 5 + 1;
  ///   for col_idx in 0..5 {
  ///     let value = (starting_row_value + col_idx) as i32;
  ///     assert_eq!(ddma.value([row_idx, col_idx]).copied(), Some(value));
  ///   }
  /// }
  /// ```
  #[inline]
  pub fn value(&self, [row_idx, col_idx]: [usize; 2]) -> Option<&DATA> {
    self.data().get(self.stride(row_idx).saturating_add(col_idx))
  }
}

impl<DATA, DS> DrMatrix<DS>
where
  DS: AsMut<[DATA]> + SingleTypeStorage<Item = DATA>,
{
  /// Mutable version of [`data`](#method.data).
  #[inline]
  pub fn data_mut(&mut self) -> &mut [DATA] {
    self.data.as_mut()
  }

  /// Removes a row of a given `idx` by shifting all posterior rows.
  ///
  /// # Arguments
  ///
  /// * `idx` - Row index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// ddma.remove_row(2);
  /// assert_eq!(ddma.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 17, 18, 19, 20]);
  /// assert_eq!(ddma.rows(), 3);
  /// ```
  #[inline]
  #[must_use]
  pub fn remove_row(&mut self, idx: usize) -> bool
  where
    DS: Truncate<Input = usize>,
  {
    if idx >= self.rows {
      return false;
    }
    let mut from_row_idx = idx;
    let mut to_row_idx = idx + 1;
    while to_row_idx < self.rows {
      let _ = self.swap_rows(from_row_idx, to_row_idx);
      from_row_idx += 1;
      to_row_idx += 1;
    }
    self.truncate(self.rows - 1);
    true
  }

  /// Mutable version of [`row`](#method.row).
  #[inline]
  pub fn row_mut(&mut self, row_idx: usize) -> Option<&mut [DATA]> {
    let row_range = self.row_range(row_idx)?;
    self.data_mut().get_mut(row_range)
  }

  /// Mutable version of [`row_iter`](#method.row_iter).
  #[inline]
  pub fn row_iter_mut(&mut self) -> DrMatrixRowIterMut<'_, DATA> {
    DrMatrixRowIterMut::new([self.rows, self.cols], self.data.as_mut())
  }

  #[inline]
  #[cfg(feature = "rayon")]
  pub fn row_par_iter_mut(
    &mut self,
  ) -> crate::ParallelIteratorWrapper<DrMatrixRowIterMut<'_, DATA>> {
    crate::ParallelIteratorWrapper(self.row_iter_mut())
  }

  /// Swaps a single value given the two provided indices.
  ///
  /// # Arguments
  ///
  /// * `a` - First pair of indices
  /// * `b` - Second pair of indices
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_vec;
  /// let mut ddma = dr_matrix_vec();
  /// let _ = ddma.swap([0, 0], [3, 1]);
  /// assert_eq!(ddma.data(), &[17, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 18, 19, 20]);
  /// ```
  #[inline]
  #[must_use]
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

  /// Swaps two entire rows.
  ///
  /// # Arguments
  ///
  /// * `a` - First row index
  /// * `b` - Second row index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
  /// let mut matrix = dr_matrix_array();
  /// let original_0_row = matrix.row(0).unwrap().to_vec();
  /// let original_3_row = matrix.row(3).unwrap().to_vec();
  /// matrix.swap_rows(0, 3);
  /// assert_eq!(matrix.row(0), Some(&original_3_row[..]));
  /// assert_eq!(matrix.row(3), Some(&original_0_row[..]));
  /// ```
  #[inline]
  #[must_use]
  pub fn swap_rows(&mut self, a: usize, b: usize) -> bool {
    if let Some([first_row, second_row]) = self.two_rows_mut(a, b) {
      first_row.swap_with_slice(second_row);
      true
    } else {
      false
    }
  }

  /// Returns two mutable slices that represent two different rows.
  ///
  /// # Arguments
  ///
  /// * `a` - First row index
  /// * `b` - Second row index
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::doc_tests::dr_matrix_array;
  /// let mut matrix = dr_matrix_array();
  /// let [a, b] = matrix.two_rows_mut(0, 1).unwrap();
  /// a.iter_mut().for_each(|elem| *elem += 1);
  /// b.iter_mut().for_each(|elem| *elem += 2);
  /// assert_eq!(matrix.row(0), Some(&[2, 3, 4, 5, 6][..]));
  /// assert_eq!(matrix.row(1), Some(&[8, 9, 10, 11, 12][..]));
  /// ```
  #[inline]
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

  /// Mutable version of [`value`](#method.value).
  #[inline]
  pub fn value_mut(&mut self, row_idx: usize, col_idx: usize) -> Option<&mut DATA> {
    let data_idx = self.stride(row_idx).saturating_add(col_idx);
    self.data_mut().get_mut(data_idx)
  }
}

#[cfg(feature = "rand")]
impl<DATA, DS> DrMatrix<DS>
where
  DS: Default + SingleTypeStorage<Item = DATA> + cl_aux::Capacity + cl_aux::Push<DATA>,
{
  /// Creates a new random and valid instance delimited by the passed arguments.
  ///
  /// # Arguments
  ///
  /// * `[rows, cols]` - Pair indices.
  /// * `rng` - `rand::Rng` trait.
  /// * `cb` - Function that returns a new `DATA`.
  #[inline]
  pub fn new_random_with_rand<F, R>(
    [rows, cols]: [usize; 2],
    rng: &mut R,
    mut cb: F,
  ) -> crate::Result<Self>
  where
    F: FnMut(&mut R, [usize; 2]) -> DATA,
    R: rand::Rng,
    crate::Error: From<DS::Error>,
  {
    let mut data = DS::default();
    if rows.saturating_mul(cols) != data.capacity() {
      return Err(crate::Error::InsufficientCapacity);
    }
    for row in 0..rows {
      for col in 0..cols {
        data.push(cb(rng, [row, col]))?;
      }
    }
    Ok(DrMatrix { cols, data, rows })
  }
}
