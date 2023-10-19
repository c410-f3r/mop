use core::mem::swap;

/// Solution is a set/tuple of flattened and indexed variables.
pub trait Solution {
  const MAX_LEN: usize;

  fn has_var(&self, idx: usize) -> bool;

  fn inter_swap(&mut self, other: &mut Self, idx: usize);

  fn intra_swap(&mut self, a: usize, b: usize);

  #[inline]
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn len(&self) -> usize;
}

impl<T, const N: usize> Solution for [T; N] {
  const MAX_LEN: usize = N;

  #[inline]
  fn has_var(&self, idx: usize) -> bool {
    idx < self.len()
  }

  #[inline]
  fn inter_swap(&mut self, other: &mut Self, idx: usize) {
    assert!(idx < self.len());
    swap(&mut self[idx], &mut other[idx]);
  }

  #[inline]
  fn intra_swap(&mut self, a: usize, b: usize) {
    self.swap(a, b);
  }

  #[inline]
  fn len(&self) -> usize {
    N
  }
}

impl<T, const N: usize> Solution for arrayvec::ArrayVec<T, N> {
  const MAX_LEN: usize = N;

  #[inline]
  fn has_var(&self, idx: usize) -> bool {
    idx < self.len()
  }

  #[inline]
  fn inter_swap(&mut self, other: &mut Self, idx: usize) {
    assert!(idx < self.len());
    swap(&mut self[idx], &mut other[idx]);
  }

  #[inline]
  fn intra_swap(&mut self, a: usize, b: usize) {
    self.swap(a, b);
  }

  #[inline]
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "ndstruct")]
impl<DATA, DS, IS, OS, const D: usize> Solution for ndstruct::csl::Csl<DS, IS, OS, D>
where
  DS: AsMut<[DATA]> + AsRef<[DATA]> + cl_aux::SingleTypeStorage<Item = DATA>,
  IS: AsRef<[usize]>,
  OS: AsRef<[usize]>,
{
  const MAX_LEN: usize = D;

  #[inline]
  fn has_var(&self, idx: usize) -> bool {
    idx < self.len()
  }

  #[inline]
  fn inter_swap(&mut self, other: &mut Self, idx: usize) {
    assert!(idx < self.len());
    swap(&mut self.data_mut()[idx], &mut other.data_mut()[idx]);
  }

  #[inline]
  fn intra_swap(&mut self, a: usize, b: usize) {
    self.data_mut().swap(a, b);
  }

  #[inline]
  fn len(&self) -> usize {
    self.data().len()
  }
}
