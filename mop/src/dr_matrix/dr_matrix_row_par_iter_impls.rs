use crate::{
  dr_matrix::{DrMatrixRowIter, DrMatrixRowIterMut},
  ParallelIteratorWrapper, ParallelProducerWrapper,
};
use rayon::iter::{
  plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
  IndexedParallelIterator, ParallelIterator,
};

macro_rules! impl_par_iter {
  ($dr_matrix_row_iter:ident, $return_type:ty) => {
    impl<'any, T> ParallelIterator for ParallelIteratorWrapper<$dr_matrix_row_iter<'any, T>>
    where
      T: Send + Sync,
    {
      type Item = $return_type;

      #[inline]
      fn drive_unindexed<C>(self, consumer: C) -> C::Result
      where
        C: UnindexedConsumer<Self::Item>,
      {
        bridge(self, consumer)
      }

      #[inline]
      fn opt_len(&self) -> Option<usize> {
        Some(self.0.len())
      }
    }

    impl<T> IndexedParallelIterator for ParallelIteratorWrapper<$dr_matrix_row_iter<'_, T>>
    where
      T: Send + Sync,
    {
      #[inline]
      fn drive<C>(self, consumer: C) -> C::Result
      where
        C: Consumer<Self::Item>,
      {
        bridge(self, consumer)
      }

      #[inline]
      fn len(&self) -> usize {
        ExactSizeIterator::len(&self.0)
      }

      #[inline]
      fn with_producer<Cb>(self, callback: Cb) -> Cb::Output
      where
        Cb: ProducerCallback<Self::Item>,
      {
        callback.callback(ParallelProducerWrapper(self.0))
      }
    }

    impl<'any, T> IntoIterator for ParallelProducerWrapper<$dr_matrix_row_iter<'any, T>> {
      type IntoIter = $dr_matrix_row_iter<'any, T>;
      type Item = <Self::IntoIter as Iterator>::Item;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.0
      }
    }

    impl<'any, T> Producer for ParallelProducerWrapper<$dr_matrix_row_iter<'any, T>>
    where
      T: Send + Sync,
    {
      type IntoIter = $dr_matrix_row_iter<'any, T>;
      type Item = <Self::IntoIter as Iterator>::Item;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.0
      }

      #[inline]
      fn split_at(self, i: usize) -> (Self, Self) {
        let (a, b) = self.0.split_at(i);
        (ParallelProducerWrapper(a), ParallelProducerWrapper(b))
      }
    }
  };
}

impl_par_iter!(DrMatrixRowIter, &'any [T]);
impl_par_iter!(DrMatrixRowIterMut, &'any mut [T]);
