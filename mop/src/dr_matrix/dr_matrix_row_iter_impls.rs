use core::{marker::PhantomData, mem};

macro_rules! impl_iter (
    ($dr_matrix_iter:ident, $data_ptr:ty, $data_type:ty, $split_at:ident) => (
#[derive(Debug)]
pub struct $dr_matrix_iter<'any, T> {
    cols: usize,
    curr_row: usize,
    data: $data_type,
    phantom: PhantomData<&'any T>,
    rows: usize,
}

impl<'any, T> $dr_matrix_iter<'any, T> {
    pub(crate) fn new(
        [rows, cols]: [usize; 2],
        data: $data_type,
    ) -> Self {
        Self {
            cols,
            curr_row: 0,
            data,
            phantom: PhantomData,
            rows,
        }
    }

    #[inline]
    pub fn split_at(self, idx: usize) -> (Self, Self) {
        let current_len = self.rows - self.curr_row;
        assert!(idx <= current_len);
        let slice_point = self.curr_row + idx;
        let (data_head, data_tail) = self.data.$split_at((slice_point - self.curr_row) * self.cols);
        (
            $dr_matrix_iter {
                curr_row: self.curr_row,
                data: data_head,
                rows: slice_point,
                cols: self.cols,
                phantom: PhantomData,
            },
            $dr_matrix_iter {
                curr_row: slice_point,
                data: data_tail,
                cols: self.cols,
                rows: self.rows,
                phantom: PhantomData,
            }
        )
    }
}

impl<T> DoubleEndedIterator for $dr_matrix_iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.rows {
            return None;
        }
        let data = mem::take(&mut self.data);
        let (data_head, data_tail) = data.$split_at(data.len() - self.cols);
        self.data = data_head;
        self.rows -= 1;
        Some(data_tail)
    }
}

impl<T> ExactSizeIterator for $dr_matrix_iter<'_, T> {
}

impl<'any, T> Iterator for $dr_matrix_iter<'any, T> {
    type Item = $data_type;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.rows {
            return None;
        }
        let data = mem::take(&mut self.data);
        let (data_head, data_tail) = data.$split_at(self.cols);
        self.data = data_tail;
        self.curr_row += 1;
        Some(data_head)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows, Some(self.rows))
    }
}

    );
);

impl_iter!(DrMatrixRowIter, *const T, &'any [T], split_at);
impl_iter!(DrMatrixRowIterMut, *mut T, &'any mut [T], split_at_mut);
