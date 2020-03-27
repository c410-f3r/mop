pub mod binh_and_korn;
pub mod constr;
pub mod cvrp;
pub mod rastrigin;
pub mod schaffer_function_2;
pub mod test_function_4;

use core::ops::Range;
use mop::{blocks::mph::Mph, facades::opt::OptFacade};

pub trait Problem<C, O, OH, OR, S, SD> {
  fn facade(
    &self,
    facade: OptFacade<C, O, OH, OR, S, SD>,
    _problem: &mut Mph<C, O, OR, S, SD>,
  ) -> OptFacade<C, O, OH, OR, S, SD> {
    facade
  }

  fn graph_ranges(&self) -> [Range<OR>; 2];

  fn problem(&self, results_num: usize) -> Mph<C, O, OR, S, SD>;
}
