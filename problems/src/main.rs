//! Run any problem by typing `cargo run --features "stdout with_plotters ANY_SUPPORTED_PROBLEM" --release`

mod problems;

#[cfg(feature = "with_plotters")]
use plotters::prelude::*;
use problems::Problem;

#[cfg(feature = "stdout")]
fn print_result<OR, S>(result: mop::blocks::mph::MphOrRef<OR, S>)
where
  OR: core::fmt::Debug,
  S: core::fmt::Debug,
{
  use std::fmt::Write;
  let mut objs = String::new();
  for (idx, obj) in result.objs().iter().enumerate() {
    objs.push('o');
    objs.write_fmt(format_args!("{}", idx)).unwrap();
    objs.push_str(": ");
    objs.write_fmt(format_args!("{:?}", obj)).unwrap();
    objs.push(' ');
  }
  objs.push_str("AVG: ");
  objs.write_fmt(format_args!("{:?}", result.objs_avg())).unwrap();
  let mut hcs = String::new();
  for (idx, hc) in result.hard_cstrs().iter().enumerate() {
    hcs.push_str("hc");
    hcs.write_fmt(format_args!("{}", idx)).unwrap();
    hcs.push_str(": ");
    hcs.write_fmt(format_args!("{:?}", hc)).unwrap();
    hcs.push(' ');
  }
  println!("{:?}", result.solution());
  println!("{}", objs);
  println!("{}", hcs);
  println!();
}

#[cfg(feature = "with_plotters")]
pub fn manage_plotting<'a, A, B, I, T>(x: A, y: B, name: &str, iter: I)
where
  A: plotters::coord::AsRangedCoord<Value = T>,
  B: plotters::coord::AsRangedCoord<Value = T>,
  I: Iterator<Item = (T, T)> + 'a,
  T: core::fmt::Debug + 'static,
{
  let mut file = std::env::temp_dir();
  file.push("mop");
  std::fs::create_dir_all(&file).unwrap();
  file.push(name);

  let root_area = SVGBackend::new(&file, (800, 600)).into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption(name, ("sans-serif", 40))
    .build_ranged(x, y)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  let data = iter.map(|p| TriangleMarker::new(p, 5, &BLUE));
  ctx.draw_series(data).unwrap();
}

macro_rules! exec {
  ($($feature:literal, $problem:expr)+) => {
    #[cfg(any($(feature = $feature),+))]
    use mop::{
      blocks::Pct,
      facades::opt::OptFacadeBuilder,
      solvers::{
        genetic_algorithm::{
          operators::{
            crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
          },
          GeneticAlgorithmParamsBuilder, Spea2,
        },
        quality_comparator::ParetoComparator,
      },
    };

    $(
      #[cfg(feature = $feature)]
      {
        let results_num = 200;
        let mut problem = $problem.problem(results_num);

        let facade = OptFacadeBuilder::default()
          .max_iterations(199)
          .opt_hooks(())
          .stagnation_percentage(Pct::from_percent(1))
          .stagnation_threshold(10)
          .build();

          let spea2 = Spea2::new(
            Pct::from_percent(50),
            GeneticAlgorithmParamsBuilder::default()
              .crossover(MultiPoint::new(1, Pct::from_percent(70)))
              .mating_selection(Tournament::new(4, ParetoComparator::default()))
              .mutation(RandomDomainAssignments::new(1, Pct::from_percent(30)))
              .build(),
            &problem,
            ParetoComparator::default(),
          );

        $problem.facade(facade, &mut problem).solve_problem_with(&mut problem, spea2).await;

        #[cfg(feature = "stdout")]
        {
          for (result_idx, result) in problem.results().iter().enumerate() {
            println!("***** Result #{} *****", result_idx + 1);
            print_result(result);
          }

          println!("***** Best result *****");
          print_result(problem.results().best().unwrap());
        }

        #[cfg(feature = "with_plotters")]
        {
          if problem.definitions().objs().len() == 2 {
            let [x, y] = $problem.graph_ranges();
            manage_plotting(
              x,
              y,
              &format!("{} - Objectives", problem.definitions().name()),
              problem.results().iter().map(|r| (r.objs()[0], r.objs()[1])),
            );
          }
        }
      }
    )+
  };
}

#[tokio::main]
async fn main() {
  exec!(
    "binh_and_korn", problems::binh_and_korn::BinhAndKorn
    "constr", problems::constr::Constr
    "cvrp", problems::cvrp::Cvrp
    "rastrigin", problems::rastrigin::Rastrigin
    "schaffer_function_2", problems::schaffer_function_2::SchafferFunction2
    "test_function_4", problems::test_function_4::TestFunction4
  );
}
