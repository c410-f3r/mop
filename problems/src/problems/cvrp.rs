//! http://vrp.galgos.inf.puc-rio.br/index.php/en/plotted-instances?data=B-n31-k5

#![allow(clippy::range_minus_one)]

use crate::Problem;
use cl_traits::create_array;
use core::ops::{Range, RangeInclusive};
use mop::{
  blocks::{
    mph::{Mph, MphDefinitionsBuilder},
    Cstr, Obj, ObjDirection,
  },
  facades::{initial_solutions::UserInitialSolutions, opt::OptFacade},
};
use ndsparse::csl::CslArrayVec;

const DATA: Data = Data {
  capacity: 100,
  depot: Node { demand: 0, id: 1, x: 17.0, y: 76.0 },
  max_routes: 10,
  max_stops: 10,
  places: [
    Node { demand: 25, id: 2, x: 24.0, y: 6.0 },
    Node { demand: 3, id: 3, x: 96.0, y: 29.0 },
    Node { demand: 13, id: 4, x: 14.0, y: 19.0 },
    Node { demand: 17, id: 5, x: 14.0, y: 32.0 },
    Node { demand: 16, id: 6, x: 0.0, y: 34.0 },
    Node { demand: 9, id: 7, x: 16.0, y: 22.0 },
    Node { demand: 22, id: 8, x: 20.0, y: 26.0 },
    Node { demand: 10, id: 9, x: 22.0, y: 28.0 },
    Node { demand: 16, id: 10, x: 17.0, y: 23.0 },
    Node { demand: 8, id: 11, x: 98.0, y: 30.0 },
    Node { demand: 3, id: 12, x: 30.0, y: 8.0 },
    Node { demand: 16, id: 13, x: 23.0, y: 27.0 },
    Node { demand: 16, id: 14, x: 19.0, y: 23.0 },
    Node { demand: 10, id: 15, x: 34.0, y: 7.0 },
    Node { demand: 24, id: 16, x: 31.0, y: 7.0 },
    Node { demand: 16, id: 17, x: 0.0, y: 37.0 },
    Node { demand: 15, id: 18, x: 19.0, y: 23.0 },
    Node { demand: 14, id: 19, x: 0.0, y: 36.0 },
    Node { demand: 5, id: 20, x: 26.0, y: 7.0 },
    Node { demand: 12, id: 21, x: 98.0, y: 32.0 },
    Node { demand: 2, id: 22, x: 5.0, y: 40.0 },
    Node { demand: 18, id: 23, x: 17.0, y: 26.0 },
    Node { demand: 20, id: 24, x: 21.0, y: 26.0 },
    Node { demand: 15, id: 25, x: 28.0, y: 8.0 },
    Node { demand: 8, id: 26, x: 1.0, y: 35.0 },
    Node { demand: 22, id: 27, x: 27.0, y: 28.0 },
    Node { demand: 15, id: 28, x: 99.0, y: 30.0 },
    Node { demand: 10, id: 29, x: 26.0, y: 28.0 },
    Node { demand: 13, id: 30, x: 17.0, y: 29.0 },
    Node { demand: 19, id: 31, x: 20.0, y: 26.0 },
  ],
};

type Constrain<'a> = RouteCapacityMustNotExceedTruckCapacity<'a>;
type Objective<'a> = MinCost<'a>;
// A 2-D sparse structure or a sparse matrix or a graph storage
type Solution = CslArrayVec<[usize; 2], [usize; 30], [usize; 30], [usize; 31]>;
type SolutionDomain = [RangeInclusive<usize>; 30];

#[derive(Debug)]
pub struct Data {
  capacity: usize,
  depot: Node,
  max_routes: usize,
  max_stops: usize,
  places: [Node; 30],
}

#[derive(Debug)]
pub struct Node {
  demand: usize,
  id: usize,
  x: f64,
  y: f64,
}

pub struct RouteCapacityMustNotExceedTruckCapacity<'a> {
  pub data: &'a Data,
}

impl RouteCapacityMustNotExceedTruckCapacity<'_> {
  fn func<F>(&self, solution: &Solution, mut cb: F)
  where
    F: FnMut(usize, usize),
  {
    for (route_idx, route) in solution.outermost_iter().enumerate() {
      let mut route_capacity = 0;
      for place_idx in route.data().iter().copied() {
        route_capacity += self.data.places[place_idx].demand;
      }
      if route_capacity > self.data.capacity {
        cb(route_idx, route_capacity - self.data.capacity);
      }
    }
  }
}

impl<'a> Cstr<Solution> for RouteCapacityMustNotExceedTruckCapacity<'a> {
  fn reasons(&self, solution: &Solution) -> String {
    let mut reasons = String::new();
    self.func(solution, |route_idx, surplus| {
      reasons.push_str(&format!(
        "Route #{} extrapolates truck capacity by {}\n",
        route_idx + 1,
        surplus
      ))
    });
    reasons
  }

  fn violations(&self, solution: &Solution) -> usize {
    let mut results = 0;
    self.func(solution, |_, _| results += 1);
    results
  }
}

#[derive(Clone, Debug)]
pub struct MinCost<'a> {
  pub data: &'a Data,
}

impl<'a> MinCost<'a> {
  fn euclidian_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
  }
}

impl Obj<f64, Solution> for MinCost<'_> {
  fn obj_direction(&self) -> ObjDirection {
    ObjDirection::Min
  }

  fn result(&self, solution: &Solution) -> f64 {
    let mut cost = 0.0;
    for route in solution.outermost_iter() {
      let mut last_place = &self.data.depot;
      for place_idx in route.data().iter().copied() {
        let distance = Self::euclidian_distance(
          last_place.x,
          last_place.y,
          self.data.places[place_idx].x,
          self.data.places[place_idx].y,
        );
        cost += distance;
        last_place = &self.data.places[place_idx];
      }
    }
    cost
  }
}

pub struct Cvrp;

impl<'a> Problem<Constrain<'a>, Objective<'a>, (), f64, Solution, SolutionDomain> for Cvrp {
  fn facade(
    &self,
    facade: OptFacade<Constrain<'a>, Objective<'a>, (), f64, Solution, SolutionDomain>,
    problem: &mut Mph<Constrain<'a>, Objective<'a>, f64, Solution, SolutionDomain>,
  ) -> OptFacade<Constrain<'a>, Objective<'a>, (), f64, Solution, SolutionDomain> {
    facade.initial_solutions(
      UserInitialSolutions::new(|_| {
        use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
        let mut rng = StdRng::from_entropy();
        let mut random_solution: [usize; 30] = create_array(|_| rng.gen_range(0, 30));
        random_solution.shuffle(&mut rng);
        CslArrayVec::new_controlled_random_with_rand(
          [DATA.max_stops, DATA.max_routes],
          DATA.places.len(),
          &mut rng,
          |_, _| random_solution.iter().copied().next().unwrap(),
        )
      }),
      problem,
    )
  }

  fn graph_ranges(&self) -> [Range<f64>; 2] {
    [-3.0..13.0, -8.0..-4.0]
  }

  fn problem(
    &self,
    results_num: usize,
  ) -> Mph<Constrain<'a>, Objective<'a>, f64, Solution, SolutionDomain> {
    Mph::with_capacity(
      MphDefinitionsBuilder::default()
        .name("CVRP")
        .push_hard_cstr(RouteCapacityMustNotExceedTruckCapacity { data: &DATA })
        .push_obj(MinCost { data: &DATA })
        .solution_domain(create_array(|_| DATA.places.len() - 1..=DATA.places.len() - 1))
        .build(),
      results_num,
    )
  }
}
