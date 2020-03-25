use alloc::vec::Vec;
use cl_traits::{Push, Storage};
use core::iter::Extend;

pub type DrMatrixRowConstructorMut<'a, DATA> = DrMatrixRowConstructor<'a, &'a mut [DATA]>;
pub type DrMatrixRowConstructorRef<'a, DATA> = DrMatrixRowConstructor<'a, &'a [DATA]>;
pub type DrMatrixRowConstructorVec<'a, T> = DrMatrixRowConstructor<'a, Vec<T>>;

/// Constructs a new valid row in a easy and interactive manner.
///
/// This struct may panic when out of scope. Please see the `Drop` documentation in
/// the [`Trait Implementations`](#implementations) section for more information.
/// #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq)]
pub struct DrMatrixRowConstructor<'a, DS> {
  data: &'a mut DS,
  cols: usize,
  rows: &'a mut usize,
  inserted_elems: usize,
}

impl<'a, DS> DrMatrixRowConstructor<'a, DS>
where
  DS: Push<Input = <DS as Storage>::Item> + Storage,
{
  pub(crate) fn new(rows: &'a mut usize, cols: usize, data: &'a mut DS) -> Self {
    DrMatrixRowConstructor { data, rows, cols, inserted_elems: 0 }
  }

  /// Clones all values of `row` into the current row.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor().copy_values_from_row(&[1, 2, 3, 4, 5]).commit();
  /// assert_eq!(a.as_ref(), DrMatrixRef::new(1, 5, &[1, 2, 3, 4, 5][..]));
  /// ```
  pub fn clone_values_from_row<'b>(mut self, row: &'b [DS::Item]) -> Self
  where
    DS::Item: Clone + 'b,
    DS: Extend<&'b <DS as Storage>::Item>,
  {
    self.inserted_elems += row.len();
    self.data.extend(row);
    self
  }

  /// Commits the row construction, modifying the internal structure.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor()
  ///   .push_value(1)
  ///   .push_value(2)
  ///   .push_value(3)
  ///   .push_value(4)
  ///   .push_value(5)
  ///   .commit();
  /// assert_eq!(a.as_ref(), DrMatrixRef::new(1, 5, &[1, 2, 3, 4, 5][..]));
  /// ```
  ///
  /// # Assertions
  ///
  /// * The number of inserted elements must be equal the number of columns of `Self`.
  ///
  /// ```should_panic
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor().push_value(1).commit();
  /// ```
  pub fn commit(mut self) {
    assert!(
      self.inserted_elems == self.cols,
      "The number of inserted elements must be equal the number of columns of `Self`."
    );
    self.inserted_elems = 0;
    *self.rows += 1;
  }

  /// Copies all values of `row` into the current row.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixRef};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor().copy_values_from_row(&[1, 2, 3, 4, 5]).commit();
  /// assert_eq!(a.as_ref(), DrMatrixRef::new(1, 5, &[1, 2, 3, 4, 5][..]));
  /// ```
  pub fn copy_values_from_row<'b>(mut self, row: &'b [DS::Item]) -> Self
  where
    DS::Item: Copy + 'b,
    DS: Extend<&'b <DS as Storage>::Item>,
  {
    self.inserted_elems += row.len();
    self.data.extend(row);
    self
  }

  /// Pushes a new value.
  ///
  /// # Example
  ///
  /// ```rust
  /// use mop_blocks::{doc_tests::{capacited_dr_matrix_vec, dr_matrix_array}, dr_matrix::DrMatrixVec};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor()
  ///   .push_value(1)
  ///   .push_value(2)
  ///   .push_value(3)
  ///   .push_value(4)
  ///   .push_value(5)
  ///   .commit();
  /// a.row_constructor()
  ///   .push_value(6)
  ///   .push_value(7)
  ///   .push_value(8)
  ///   .push_value(9)
  ///   .push_value(10)
  ///   .commit();
  /// a.row_constructor()
  ///   .push_value(11)
  ///   .push_value(12)
  ///   .push_value(13)
  ///   .push_value(14)
  ///   .push_value(15)
  ///   .commit();
  /// a.row_constructor()
  ///   .push_value(16)
  ///   .push_value(17)
  ///   .push_value(18)
  ///   .push_value(19)
  ///   .push_value(20)
  ///   .commit();
  /// assert_eq!(a.as_ref(), dr_matrix_array().as_ref());
  /// ```
  pub fn push_value(mut self, value: DS::Item) -> Self {
    self.data.push(value);
    self.inserted_elems += 1;
    self
  }
}

impl<'a, DS> Drop for DrMatrixRowConstructor<'a, DS> {
  /// Some measures are taken to ensure a valid format and avoid unexpected runtime behavior.
  ///
  /// # Assertions
  ///
  /// * Every single nonempty instance of `DrMatrixRowConstructor` must end with a call to
  /// the `commit` method.
  ///
  /// ```should_panic
  /// use mop_blocks::{doc_tests::capacited_dr_matrix_vec, dr_matrix::DrMatrixVec};
  /// let mut a = capacited_dr_matrix_vec();
  /// a.row_constructor().push_value(1).push_value(2).push_value(3).push_value(4);
  /// ```
  fn drop(&mut self) {
    if self.inserted_elems > 0 {
      panic!(
        "Every single nonempty instance of `DrMatrixRowConstructor` must
                end with a call to the `commit` method."
      );
    }
  }
}
