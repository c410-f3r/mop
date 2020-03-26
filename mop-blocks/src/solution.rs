use core::mem::swap;

/// Solution is a set/tuple of flattened and indexed variables.
pub trait Solution {
  fn has_var(&self, idx: usize) -> bool;

  fn inter_swap(&mut self, other: &mut Self, idx: usize);

  fn intra_swap(&mut self, a: usize, b: usize);

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn len(&self) -> usize;
}

macro_rules! array_impls {
  ($($N:expr),+) => {
    $(
      impl<T> Solution for [T; $N] {
        fn has_var(&self, idx: usize) -> bool {
          idx < self.len()
        }

        fn inter_swap(&mut self, other: &mut Self, idx: usize) {
          assert!(idx < self.len());
          swap(&mut self[idx], &mut other[idx]);
        }

        fn intra_swap(&mut self, a: usize, b: usize) {
          self.swap(a, b);
        }

        fn len(&self) -> usize {
          $N
        }
      }

      #[cfg(feature = "with_arrayvec")]
      impl<T> Solution for arrayvec::ArrayVec<[T; $N]> {
        fn has_var(&self, idx: usize) -> bool {
          idx < self.len()
        }

        fn inter_swap(&mut self, other: &mut Self, idx: usize) {
          assert!(idx < self.len());
          swap(&mut self[idx], &mut other[idx]);
        }

        fn intra_swap(&mut self, a: usize, b: usize) {
          self.swap(a, b);
        }

        fn len(&self) -> usize {
          self.len()
        }
      }

      #[cfg(feature = "with_ndsparse")]
      impl<DATA, DS, IS, OS> Solution for ndsparse::csl::Csl<[usize; $N], DS, IS, OS>
      where
        DS: AsMut<[DATA]> + AsRef<[DATA]> + cl_traits::Storage<Item = DATA>,
        IS: AsRef<[usize]>,
        OS: AsRef<[usize]>,
      {
        fn has_var(&self, idx: usize) -> bool {
          idx < self.len()
        }

        fn inter_swap(&mut self, other: &mut Self, idx: usize) {
          assert!(idx < self.len());
          swap(&mut self.data_mut()[idx], &mut other.data_mut()[idx]);
        }

        fn intra_swap(&mut self, a: usize, b: usize) {
          self.data_mut().swap(a, b);
        }

        fn len(&self) -> usize {
          self.data().len()
        }
      }
    )+
  }
}

array_impls!(
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32
);
