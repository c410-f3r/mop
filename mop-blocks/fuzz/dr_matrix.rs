#![no_main]

use libfuzzer_sys::fuzz_target;
use mop_blocks::dr_matrix::DrMatrixVec;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
  rows: usize,
  cols: usize,
  data: Vec<i32>,
  row: usize,
  value: [usize; 2],
}

fuzz_target!(|data: Data| {
  let m = if let Ok(r) = DrMatrixVec::new(data.rows, data.cols, data.data) { r } else { return };

  let _ = m.row(data.row);
  let _ = m.value(data.value[0], data.value[1]);

  //_row_iter_next(&m);
  //_row_iter_next_back(&m);
  //_row_iter_split_at(&m);
});

fn _row_iter_next(m: &DrMatrixVec<i32>) {
  m.row_iter().enumerate().for_each(|(row_idx, row)| assert_eq!(Some(row), m.row(row_idx)));
}

fn _row_iter_next_back(m: &DrMatrixVec<i32>) {
  let mut i = m.row_iter();
  let mut idx = m.rows();
  while let Some(row) = i.next_back() {
    idx -= 1;
    assert_eq!(Some(row), m.row(idx));
  }
}

fn _row_iter_split_at(m: &DrMatrixVec<i32>) {
  let split_in = if m.rows() > 3 { 3 } else { m.rows() };
  let (first_half, second_half) = m.row_iter().split_at(split_in);
  let is_first_ok = first_half.enumerate().all(|(row_idx, row)| Some(row) == m.row(row_idx));
  let all = |(row_idx, row)| Some(row) == m.row(row_idx + split_in);
  let is_second_ok = second_half.enumerate().all(all);
  assert!(is_first_ok && is_second_ok);
}
