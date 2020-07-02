use alloc::vec::Vec;
use mop_blocks::gp::MpOrs;

#[derive(Clone, Debug)]
pub struct ArchUnionPopul<OR, ORS, SS> {
  pub props: Vec<Properties<OR>>,
  pub rslts: MpOrs<ORS, SS>,
}

#[derive(Clone, Debug)]
pub struct Properties<T> {
  pub fitness: T,
  pub result_idx: usize,
  pub strength: T,
}
