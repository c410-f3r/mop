#![no_main]

use core::ops::RangeInclusive;
use libfuzzer_sys::fuzz_target;
use mop_blocks::{
  gp::{MpDefinitionsBuilderVec, MpVec},
  ObjDirection,
};

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
  domain: [RangeInclusive<f64>; 2],
  rslts_num: usize,
}

fn obj(_: &[f64; 2]) -> f64 {
  0.0
}

fuzz_target!(|data: Data| {
  let obj_fn: fn(&[f64; 2]) -> f64 = obj;
  let fun = || {
    MpDefinitionsBuilderVec::<_, (ObjDirection, fn(&[f64; 2]) -> f64)>::default()
      .domain(data.domain.clone())
      .push_obj((ObjDirection::Min, obj_fn))?
      .build()
  };

  let mdb = if let Ok(r) = fun() {
    r
  } else {
    return;
  };

  let _problem = MpVec::with_random_solutions(mdb, data.rslts_num);
});
