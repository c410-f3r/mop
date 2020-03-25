use alloc::vec::Vec;
use mop_blocks::mph::MphOrs;

#[derive(Clone, Debug)]
pub struct ArchUnionPopul<OR, S> {
  pub props: Vec<Properties<OR>>,
  pub results: MphOrs<OR, S>,
}

#[derive(Clone, Debug)]
pub struct Properties<T> {
  pub fitness: T,
  pub result_idx: usize,
  pub strength: T,
}
