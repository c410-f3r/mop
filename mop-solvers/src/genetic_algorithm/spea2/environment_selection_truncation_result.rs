#[derive(Debug)]
pub struct EnvironmentSelectionTruncationResult<T> {
  pub archive_idx: usize,
  pub distance: T,
}
