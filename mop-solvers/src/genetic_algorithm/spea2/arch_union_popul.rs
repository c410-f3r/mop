use alloc::vec::Vec;
use mop_blocks::gp::MpOrs;

#[derive(Clone, Debug)]
pub(crate) struct ArchUnionPopul<OR, ORS, SS> {
  pub(crate) props: Vec<Properties<OR>>,
  pub(crate) rslts: MpOrs<ORS, SS>,
}

#[derive(Clone, Debug)]
pub(crate) struct Properties<T> {
  pub(crate) fitness: T,
  pub(crate) result_idx: usize,
  pub(crate) strength: T,
}
