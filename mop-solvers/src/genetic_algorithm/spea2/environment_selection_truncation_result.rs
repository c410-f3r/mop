#[derive(Debug)]
pub(crate) struct EnvironmentSelectionTruncationResult<T> {
  pub(crate) archive_idx: usize,
  pub(crate) distance: T,
}
