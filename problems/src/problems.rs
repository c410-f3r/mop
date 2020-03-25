pub mod binh_and_korn;
pub mod rastrigin;
pub mod schaffer_function_2;
pub mod test_function_4;

use core::ops::Range;
use mop::blocks::mph::Mph;

pub trait Problem<C, O, OR, S, SD> {
  fn graph_ranges(&self) -> [Range<OR>; 2];
  fn problem(&self, results_num: usize) -> Mph<C, O, OR, S, SD>;
}
