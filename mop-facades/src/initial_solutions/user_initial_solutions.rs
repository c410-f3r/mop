use crate::initial_solutions::InitialSolutions;
use mop_blocks::{mph::Mph, SolutionDomain};

#[derive(Clone, Debug, Default)]
pub struct UserInitialSolutions<F> {
  cb: F,
}

impl<F> UserInitialSolutions<F> {
  pub fn new(cb: F) -> Self {
    UserInitialSolutions { cb }
  }
}

impl<C, F, O, OR, S, SD> InitialSolutions<C, O, OR, S, SD> for UserInitialSolutions<F>
where
  F: FnMut(usize) -> S,
  OR: Default,
  SD: SolutionDomain<S>,
{
  fn initial_solutions(&mut self, op: &mut Mph<C, O, OR, S, SD>) {
    let (defs, results) = op.parts_mut();
    for idx in 0..results.results_num() {
      let mut c = results.constructor();
      c = (0..defs.hard_cstrs().len()).fold(c, |c, _| c.push_hard_cstr(usize::default()));
      c = (0..defs.objs().len()).fold(c, |c, _| c.push_obj(OR::default()));
      c.commit(OR::default(), (self.cb)(idx));
    }
  }
}
