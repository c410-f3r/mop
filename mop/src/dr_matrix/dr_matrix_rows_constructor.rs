use crate::dr_matrix::DrMatrixRef;
use alloc::vec::Vec;
use cl_aux::{Length, Push, SingleTypeStorage};
use core::iter::Extend;

pub type DrMatrixRowConstructorMut<'any, DATA> = DrMatrixRowsConstructor<'any, &'any mut [DATA]>;
pub type DrMatrixRowConstructorRef<'any, DATA> = DrMatrixRowsConstructor<'any, &'any [DATA]>;
pub type DrMatrixRowConstructorVec<'any, T> = DrMatrixRowsConstructor<'any, Vec<T>>;

/// Constructs a new valid row in a easy and interactive manner.
///
/// This struct may panic when out of scope. Please see the `Drop` documentation in
/// the [`Trait Implementations`](#implementations) section for more information.
#[derive(Debug, PartialEq)]
pub struct DrMatrixRowsConstructor<'any, DS> {
  pub(crate) data: &'any mut DS,
  pub(crate) cols: usize,
  pub(crate) rows: &'any mut usize,
}

impl<'any, DS> DrMatrixRowsConstructor<'any, DS> {
  #[inline]
  pub(crate) fn new(rows: &'any mut usize, cols: usize, data: &'any mut DS) -> Self {
    DrMatrixRowsConstructor { data, cols, rows }
  }
}

impl<DATA, DS> DrMatrixRowsConstructor<'_, DS>
where
  DS: SingleTypeStorage<Item = DATA>,
{
  #[inline]
  #[must_use]
  pub fn fill_row(self, elem: DATA) -> Self
  where
    DATA: Clone,
    DS: Extend<DATA>,
  {
    self.data.extend((0..self.cols).map(|_| elem.clone()));
    *self.rows += 1;
    self
  }

  #[inline]
  #[must_use]
  pub fn fill_rows(self, rows: usize, elem: DATA) -> Self
  where
    DATA: Clone,
    DS: Extend<DATA>,
  {
    self.data.extend((0..rows * self.cols).map(|_| elem.clone()));
    *self.rows += rows;
    self
  }

  #[inline]
  pub fn matrix_ref(self, other: DrMatrixRef<'_, DATA>) -> Option<Self>
  where
    DATA: Clone,
    DS: Extend<DATA>,
  {
    if self.cols != other.cols {
      return None;
    }
    self.data.extend(other.data.iter().cloned());
    *self.rows += other.rows();
    Some(self)
  }

  #[inline]
  pub fn row_cb<F>(self, mut cb: F) -> crate::Result<Self>
  where
    DS: Push<DATA>,
    F: FnMut(usize) -> DATA,
  {
    for idx in 0..self.cols {
      self.data.push(cb(idx)).map_err(|_e| crate::Error::InsufficientCapacity)?;
    }
    *self.rows += 1;
    Ok(self)
  }

  #[inline]
  pub fn row_iter<I>(&mut self, i: I) -> &mut Self
  where
    DATA: Default,
    DS: Extend<DATA> + Length,
    I: Iterator<Item = DATA>,
  {
    let old_len = self.data.length();
    self.data.extend(i.take(self.cols));
    let new_len = self.data.length();
    let diff = self.cols - (new_len - old_len);
    self.data.extend((0..diff).map(|_| DATA::default()));
    *self.rows += 1;
    self
  }

  /// Clones all values of `row` into the current row.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.constructor().row_slice(&[1, 2, 3, 4, 5]);
  /// assert_eq!(Ok(a.as_ref()), DrMatrixRef::new([1, 5], &[1, 2, 3, 4, 5][..]));
  /// ```
  #[inline]
  pub fn row_slice(self, row: &[DS::Item]) -> Option<Self>
  where
    DS::Item: Clone,
    DS: Extend<DATA>,
  {
    if row.len() != self.cols {
      return None;
    }
    self.data.extend(row.iter().cloned());
    *self.rows += 1;
    Some(self)
  }
}
