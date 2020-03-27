#![deny(rust_2018_idioms)]
#![doc(test(attr(forbid(
  unused_variables,
  unused_assignments,
  unused_mut,
  unused_attributes,
  dead_code
))))]
#![forbid(missing_debug_implementations)]

#[cfg(all(not(feature = "with_futures"), feature = "with_wasm_bindgen"))]
mod wasm_bindgen;
