use core::{marker::PhantomData, mem};

macro_rules! impl_iter (
    ($dr_matrix_iter:ident, $data_ptr:ty, $data_type:ty, $split_at:ident) => (
#[derive(Debug)]
pub struct $dr_matrix_iter<'a, T> {
    cols: usize,
    curr_row: usize,
    data: $data_type,
    phantom: PhantomData<&'a T>,
    rows: usize,
}

impl<'a, T> $dr_matrix_iter<'a, T> {
    pub(crate) fn new(
        rows: usize,
        cols: usize,
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

impl<'a, T> DoubleEndedIterator for $dr_matrix_iter<'a, T> {
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

impl<'a, T> ExactSizeIterator for $dr_matrix_iter<'a, T> {
}

impl<'a, T> Iterator for $dr_matrix_iter<'a, T> {
    type Item = $data_type;

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows, Some(self.rows))
    }
}

    );
);

impl_iter!(DrMatrixRowIter, *const T, &'a [T], split_at);
impl_iter!(DrMatrixRowIterMut, *mut T, &'a mut [T], split_at_mut);
