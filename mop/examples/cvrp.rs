//! http://vrp.galgos.inf.puc-rio.br/index.php/en/plotted-instances?data=B-n31-k5

mod common;

#[cfg(feature = "ndstruct")]
mod module {
  use crate::common::Problem;
  use core::ops::{Range, RangeInclusive};
  use mop::{Cstr, Obj, ObjDirection};

  type CslArrayVec<DATA, const D: usize, const NNZ: usize, const OFFS: usize> = ndstruct::csl::Csl<
    arrayvec::ArrayVec<DATA, NNZ>,
    arrayvec::ArrayVec<usize, NNZ>,
    arrayvec::ArrayVec<usize, OFFS>,
    D,
  >;

  const DATA: Data = Data {
    capacity: 100,
    depot: Node { demand: 0, x: 17.0, y: 76.0 },
    places: [
      Node { demand: 25, x: 24.0, y: 6.0 },
      Node { demand: 3, x: 96.0, y: 29.0 },
      Node { demand: 13, x: 14.0, y: 19.0 },
      Node { demand: 17, x: 14.0, y: 32.0 },
      Node { demand: 16, x: 0.0, y: 34.0 },
      Node { demand: 9, x: 16.0, y: 22.0 },
      Node { demand: 22, x: 20.0, y: 26.0 },
      Node { demand: 10, x: 22.0, y: 28.0 },
      Node { demand: 16, x: 17.0, y: 23.0 },
      Node { demand: 8, x: 98.0, y: 30.0 },
      Node { demand: 3, x: 30.0, y: 8.0 },
      Node { demand: 16, x: 23.0, y: 27.0 },
      Node { demand: 16, x: 19.0, y: 23.0 },
      Node { demand: 10, x: 34.0, y: 7.0 },
      Node { demand: 24, x: 31.0, y: 7.0 },
      Node { demand: 16, x: 0.0, y: 37.0 },
      Node { demand: 15, x: 19.0, y: 23.0 },
      Node { demand: 14, x: 0.0, y: 36.0 },
      Node { demand: 5, x: 26.0, y: 7.0 },
      Node { demand: 12, x: 98.0, y: 32.0 },
      Node { demand: 2, x: 5.0, y: 40.0 },
      Node { demand: 18, x: 17.0, y: 26.0 },
      Node { demand: 20, x: 21.0, y: 26.0 },
      Node { demand: 15, x: 28.0, y: 8.0 },
      Node { demand: 8, x: 1.0, y: 35.0 },
      Node { demand: 22, x: 27.0, y: 28.0 },
      Node { demand: 15, x: 99.0, y: 30.0 },
      Node { demand: 10, x: 26.0, y: 28.0 },
      Node { demand: 13, x: 17.0, y: 29.0 },
      Node { demand: 19, x: 20.0, y: 26.0 },
    ],
  };

  type Domain = [RangeInclusive<usize>; 30];
  // A 2-D sparse structure or a sparse matrix or a graph storage
  type Solution = CslArrayVec<usize, 2, 30, 31>;

  #[derive(Debug)]
  struct Data {
    capacity: usize,
    depot: Node,
    places: [Node; 30],
  }

  #[derive(Debug)]
  struct Node {
    demand: usize,
    x: f64,
    y: f64,
  }

  #[allow(dead_code)]
  #[derive(Clone, Debug)]
  struct RouteCapacityMustNotExceedTruckCapacity {
    data: &'static Data,
  }

  impl RouteCapacityMustNotExceedTruckCapacity {
    fn func<F>(&self, solution: &Solution, mut cb: F)
    where
      F: FnMut(usize, usize),
    {
      #[allow(
        // Dimension is greater than 0
        clippy::unwrap_used
      )]
      for (route_idx, route) in solution.outermost_line_iter().unwrap().enumerate() {
        let mut route_capacity = 0;
        for place_idx in route.data().iter().copied() {
          route_capacity += DATA.places[place_idx].demand;
        }
        if route_capacity > DATA.capacity {
          cb(route_idx, route_capacity - DATA.capacity);
        }
      }
    }
  }

  impl Cstr<Solution> for RouteCapacityMustNotExceedTruckCapacity {
    #[inline]
    fn reasons(&self, solution: &Solution) -> String {
      let mut reasons = String::new();
      self.func(solution, |route_idx, surplus| {
        reasons.push_str(&format!(
          "Route #{} extrapolates truck capacity by {}\n",
          route_idx + 1,
          surplus
        ));
      });
      reasons
    }

    #[inline]
    fn violations(&self, solution: &Solution) -> usize {
      let mut ret = 0;
      self.func(solution, |_, _| ret += 1);
      ret
    }
  }

  #[derive(Clone, Debug)]
  #[allow(dead_code)]
  struct MinCost {
    data: &'static Data,
  }

  impl MinCost {
    fn euclidian_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
      ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
  }

  impl Obj<f64, Solution> for MinCost {
    #[inline]
    fn obj_direction(&self) -> ObjDirection {
      ObjDirection::Min
    }

    #[inline]
    fn result(&self, solution: &Solution) -> f64 {
      let mut cost = 0.0;
      #[allow(
        // Dimension is greater than 0
        clippy::unwrap_used
      )]
      for route in solution.outermost_line_iter().unwrap() {
        let mut last_place = &DATA.depot;
        for place_idx in route.data().iter().copied() {
          let distance = Self::euclidian_distance(
            last_place.x,
            last_place.y,
            DATA.places[place_idx].x,
            DATA.places[place_idx].y,
          );
          cost += distance;
          last_place = &DATA.places[place_idx];
        }
      }
      cost
    }
  }

  #[derive(Debug)]
  struct Cvrp;

  impl Problem<Domain, Solution, 1, 1> for Cvrp {
    const GRAPH_RANGES: [Range<f64>; 2] = [-3.0..13.0, -8.0..-4.0];
    const NAME: &'static str = "CVRP";

    type Hcs = RouteCapacityMustNotExceedTruckCapacity;
    type Objs = MinCost;

    #[inline]
    fn domain() -> Domain {
      [
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
        45..=45,
      ]
    }

    #[inline]
    fn hcs() -> [Self::Hcs; 1] {
      [RouteCapacityMustNotExceedTruckCapacity { data: &DATA }]
    }

    #[inline]
    fn objs() -> [Self::Objs; 1] {
      [MinCost { data: &DATA }]
    }
  }

  pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate::exec!("cvrp", Cvrp);
    Ok(())
  }
}

#[cfg(not(feature = "ndstruct"))]
mod module {
  pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  module::main()
}
