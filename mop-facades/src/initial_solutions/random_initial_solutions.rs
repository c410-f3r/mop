use crate::initial_solutions::InitialSolutions;
use mop_blocks::{mph::Mph, SolutionDomain};
use rand::{distributions::uniform::SampleUniform, rngs::StdRng, SeedableRng};

#[derive(Clone, Debug, Default)]
pub struct RandomInitialSolutions {}

impl<C, O, OR, S, SD> InitialSolutions<C, O, OR, S, SD> for RandomInitialSolutions
where
  OR: Default + SampleUniform,
  SD: SolutionDomain<S>,
{
  fn initial_solutions(&mut self, op: &mut Mph<C, O, OR, S, SD>) {
    let (defs, results) = op.parts_mut();
    let mut rng = StdRng::from_entropy();
    for _ in 0..results.results_num() {
      let mut c = results.constructor();
      c = (0..defs.hard_cstrs().len()).fold(c, |c, _| c.push_hard_cstr(usize::default()));
      c = (0..defs.objs().len()).fold(c, |c, _| c.push_obj(OR::default()));
      let domain = defs.solution_domain();
      c.commit(OR::default(), domain.new_random_solution(&mut rng));
    }
  }
}
