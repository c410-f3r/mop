//! Run any problem by typing `cargo run --features "stdout with-plotters ANY_SUPPORTED_PROBLEM" --release`

#[cfg(feature = "with-plotters")]
use plotters::prelude::*;

#[cfg(feature = "stdout")]
fn print_result<OR, S>(
  result: mop::blocks::gp::MphOrRef<OR, S>,
) -> Result<(), Box<dyn std::error::Error>>
where
  OR: core::fmt::Debug,
  S: core::fmt::Debug,
{
  use std::fmt::Write;
  let mut objs = String::new();
  for (idx, obj) in result.obj_rslts().iter().enumerate() {
    objs.push('o');
    objs.write_fmt(format_args!("{}", idx))?;
    objs.push_str(": ");
    objs.write_fmt(format_args!("{:?}", obj))?;
    objs.push(' ');
  }
  let mut hcs = String::new();
  for (idx, hc) in result.hard_cstr_rslts().iter().enumerate() {
    hcs.push_str("hc");
    hcs.write_fmt(format_args!("{}", idx))?;
    hcs.push_str(": ");
    hcs.write_fmt(format_args!("{:?}", hc))?;
    hcs.push(' ');
  }
  println!("{:?}", result.solution());
  println!("{}", objs);
  println!("{}", hcs);
  println!();
  Ok(())
}

#[cfg(feature = "with-plotters")]
pub fn manage_plotting<'a, A, B, I, T>(
  x: A,
  y: B,
  name: &str,
  iter: I,
) -> Result<(), Box<dyn std::error::Error>>
where
  A: plotters::coord::AsRangedCoord<Value = T>,
  B: plotters::coord::AsRangedCoord<Value = T>,
  I: Iterator<Item = (T, T)> + 'a,
  T: core::fmt::Debug + 'static,
{
  let mut file = std::env::temp_dir();
  file.push("mop");
  std::fs::create_dir_all(&file)?;
  file.push(name);

  let root_area = SVGBackend::new(&file, (800, 600)).into_drawing_area();
  root_area.fill(&WHITE)?;

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption(name, ("sans-serif", 40))
    .build_ranged(x, y)?;

  ctx.configure_mesh().draw()?;

  let data = iter.map(|p| TriangleMarker::new(p, 5, &BLUE));
  ctx.draw_series(data)?;
  Ok(())
}

macro_rules! exec {
  ($($feature:literal, $problem:ty)+) => {
    #[cfg(any($(feature = $feature),+))]
    use {
      mop::{
        blocks::{
          gp::{new_defsb_o_ref, GpOperations, MpVec, MphDefinitionsBuilder, MphMpMph, MphVec},
          objs::MinCstrsRslts,
          quality_comparator::ObjsAvg,
          Pct,
        },
        facades::opt::OptFacade,
        solvers::genetic_algorithm::{
          operators::{
            crossover::MultiPoint, mating_selection::Tournament, mutation::RandomDomainAssignments,
          },
          GeneticAlgorithmParamsBuilder, Spea2,
        },
      },
      mop_problems::Problem,
    };

    #[cfg(any($(feature = $feature),+))]
    const RSLTS_NUM: usize = 250;

    $(
      #[cfg(feature = $feature)]
      {
        let mut mph = MphVec::with_capacity(
          MphDefinitionsBuilder::default()
            .domain(<$problem>::domain())
            .hard_cstrs(<$problem>::hcs().to_vec())
            .name(<$problem>::NAME)
            .objs(<$problem>::objs().to_vec())
            .build()?,
            RSLTS_NUM,
        );
        let (mph_defs, mut mph_rslts) = mph.parts_mut();

        let mcr = MinCstrsRslts::from_gp_hcs(mph_defs);
        let mp_defs_ref = new_defsb_o_ref(mph_defs, mph_rslts).push_obj((&mcr).into()).build()?;
        let mut mp_ref = MpVec::with_random_solutions(mp_defs_ref, 100)?;

        let spea2 = Spea2::new(
          Pct::from_percent(50),
          GeneticAlgorithmParamsBuilder::default()
            .crossover(MultiPoint::new(1, Pct::from_percent(70)))
            .mating_selection(Tournament::new(5, ObjsAvg))
            .mutation(RandomDomainAssignments::new(1, Pct::from_percent(30)))
            .build()?,
          &mp_ref,
          RSLTS_NUM,
        )?;
        let _facade = OptFacade::new(2000)
          .set_quality_comparator(ObjsAvg)
          .set_opt_hooks(())
          .set_stagnation(Pct::from_percent(1), 30)?
          .solve_problem_with(&mut mp_ref, spea2)
          .await?;

        MphMpMph::transfer(&mph_defs, &mut mph_rslts, &mp_ref).await?;

        #[cfg(feature = "stdout")]
        for (result_idx, result) in mph_rslts.iter().enumerate() {
          println!("***** Result #{} *****", result_idx + 1);
          print_result(result)?;
        }
        #[cfg(feature = "stdout")]
        if let Some(curr_best_idx) = _facade.curr_best_idx() {
          if let Some(solution) = mph_rslts.get(curr_best_idx) {
            println!("***** Best result *****");
            print_result(solution)?;
          }
        }

        #[cfg(feature = "with-plotters")]
        if <$problem>::hcs().len() == 2 {
          let [x, y] = <$problem>::GRAPH_RANGES;
          manage_plotting(
            x,
            y,
            &format!("{} - Objectives", mph_defs.name()),
            mph_rslts.iter().map(|r| (r.obj_rslts()[0], r.obj_rslts()[1])),
          ).map_err(|_| mop::blocks::Error::Other("Bad plotting"))?;
        }
      }
    )+
  };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  exec!(
    "binh-and-korn", mop_problems::binh_and_korn::BinhAndKorn
    "constr", mop_problems::constr::Constr
    "cvrp", mop_problems::cvrp::Cvrp
    "rastrigin", mop_problems::rastrigin::Rastrigin
    "schaffer-function-2", mop_problems::schaffer_function_2::SchafferFunction2
    "test-function-4", mop_problems::test_function_4::TestFunction4
  );
  Ok(())
}
