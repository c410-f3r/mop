use crate::dr_matrix::{DrMatrix, DrMatrixVec};
use cl_traits::{Push, Storage};
use quickcheck::Arbitrary;
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

impl<DATA, DS> Arbitrary for DrMatrix<DS>
where
  DS: Clone + Default + Push<Input = DATA> + Storage<Item = DATA> + Send + 'static,
  Standard: Distribution<DATA>,
  DATA: Arbitrary,
{
  #[inline]
  fn arbitrary<G>(g: &mut G) -> Self
  where
    G: quickcheck::Gen,
  {
    Self::new_random_with_rand(g.gen_range(0, g.size()), g.gen_range(0, g.size()), g, |g, _, _| {
      g.gen()
    })
  }
}

#[quickcheck_macros::quickcheck]
fn dr_matrix_row_iter_next(dm: DrMatrixVec<i32>) -> bool {
  dm.row_iter().enumerate().all(|(row_idx, row)| row == dm.row(row_idx))
}

#[quickcheck_macros::quickcheck]
fn dr_matrix_row_iter_next_back(dm: DrMatrixVec<i32>) -> bool {
  let mut i = dm.row_iter();
  let mut idx = dm.rows();
  while let Some(row) = i.next_back() {
    idx -= 1;
    if row != dm.row(idx) {
      return false;
    }
  }
  true
}

#[quickcheck_macros::quickcheck]
fn dr_matrix_row_iter_split_at(dm: DrMatrixVec<i32>) -> bool {
  let split_in = if dm.rows() > 3 { 3 } else { dm.rows() };
  let (first_half, second_half) = dm.row_iter().split_at(split_in);
  let is_first_ok = first_half.enumerate().all(|(row_idx, row)| row == dm.row(row_idx));
  let is_second_ok =
    second_half.enumerate().all(|(row_idx, row)| row == dm.row(row_idx + split_in));
  is_first_ok && is_second_ok
}
