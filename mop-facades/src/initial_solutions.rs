mod random_initial_solutions;
mod user_initial_solutions;

pub use self::{
  random_initial_solutions::RandomInitialSolutions, user_initial_solutions::UserInitialSolutions,
};
use mop_blocks::mph::Mph;

pub trait InitialSolutions<C, O, OR, S, SD> {
  fn initial_solutions(&mut self, problem: &mut Mph<C, O, OR, S, SD>);
}
