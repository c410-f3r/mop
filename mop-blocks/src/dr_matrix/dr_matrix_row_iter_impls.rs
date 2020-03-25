use core::{
  marker::PhantomData,
  slice::{from_raw_parts, from_raw_parts_mut},
};

macro_rules! impl_iter (
    ($dr_matrix_iter:ident, $data_ptr:ty, $data_type:ty, $from_raw_parts:ident) => (
#[derive(Debug)]
pub struct $dr_matrix_iter<'a, T> {
    cols: usize,
    curr_row: usize,
    data: $data_ptr,
    phantom: PhantomData<&'a T>,
    rows: usize,
}

impl<'a, T> $dr_matrix_iter<'a, T> {
    pub(crate) fn new(
        rows: usize,
        cols: usize,
        data: $data_ptr,
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
        (
            $dr_matrix_iter {
                curr_row: self.curr_row,
                data: self.data,
                rows: slice_point,
                cols: self.cols,
                phantom: PhantomData,
            },
            $dr_matrix_iter {
                curr_row: slice_point,
                data: self.data,
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
        let data: $data_type = unsafe {
            let offset = ((self.rows - 1) * self.cols) as isize;
            let ptr = self.data.offset(offset);
            $from_raw_parts(ptr, self.cols)
        };
        self.rows -= 1;
        Some(data)
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
        let data: $data_type = unsafe {
            let offset = (self.curr_row * self.cols) as isize;
            let ptr = self.data.offset(offset);
            $from_raw_parts(ptr, self.cols)
        };
        self.curr_row += 1;
        Some(data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows, Some(self.rows))
    }
}

unsafe impl<'a, T> Send for $dr_matrix_iter<'a, T> {}
unsafe impl<'a, T> Sync for $dr_matrix_iter<'a, T> {}

    );
);

impl_iter!(DrMatrixRowIter, *const T, &'a [T], from_raw_parts);
impl_iter!(DrMatrixRowIterMut, *mut T, &'a mut [T], from_raw_parts_mut);
