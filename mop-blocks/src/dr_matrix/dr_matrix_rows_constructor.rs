use crate::dr_matrix::DrMatrixRef;
use alloc::vec::Vec;
use cl_traits::{Length, Push, Storage};
use core::iter::Extend;

pub type DrMatrixRowConstructorMut<'a, DATA> = DrMatrixRowsConstructor<'a, &'a mut [DATA]>;
pub type DrMatrixRowConstructorRef<'a, DATA> = DrMatrixRowsConstructor<'a, &'a [DATA]>;
pub type DrMatrixRowConstructorVec<'a, T> = DrMatrixRowsConstructor<'a, Vec<T>>;

/// Constructs a new valid row in a easy and interactive manner.
///
/// This struct may panic when out of scope. Please see the `Drop` documentation in
/// the [`Trait Implementations`](#implementations) section for more information.
#[derive(Debug, PartialEq)]
pub struct DrMatrixRowsConstructor<'a, DS> {
  pub(crate) data: &'a mut DS,
  pub(crate) cols: usize,
  pub(crate) rows: &'a mut usize,
}

impl<'a, DS> DrMatrixRowsConstructor<'a, DS> {
  pub(crate) fn new(rows: &'a mut usize, cols: usize, data: &'a mut DS) -> Self {
    DrMatrixRowsConstructor { data, rows, cols }
  }
}

impl<'a, DATA, DS> DrMatrixRowsConstructor<'a, DS>
where
  DS: Storage<Item = DATA>,
{
  pub fn fill_row(self, elem: DATA) -> Self
  where
    DATA: Clone,
    DS: Extend<DATA>,
  {
    self.data.extend((0..self.cols).map(|_| elem.clone()));
    *self.rows += 1;
    self
  }

  pub fn fill_rows(self, rows: usize, elem: DATA) -> Self
  where
    DATA: Clone,
    DS: Extend<DATA>,
  {
    self.data.extend((0..rows * self.cols).map(|_| elem.clone()));
    *self.rows += rows;
    self
  }

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

  pub fn row_cb<F>(self, mut cb: F) -> crate::Result<Self>
  where
    DS: Push<Input = DATA>,
    F: FnMut(usize) -> DATA,
  {
    for idx in 0..self.cols {
      self.data.push(cb(idx)).map_err(|_| crate::Error::InsufficientCapacity)?;
    }
    *self.rows += 1;
    Ok(self)
  }

  pub fn row_iter<I>(self, i: I) -> Self
  where
    DATA: Default,
    DS: Extend<DATA> + Length<Output = usize>,
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
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.constructor().row_slice(&[1, 2, 3, 4, 5]);
  /// assert_eq!(Ok(a.as_ref()), DrMatrixRef::new(1, 5, &[1, 2, 3, 4, 5][..]));
  /// ```
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
