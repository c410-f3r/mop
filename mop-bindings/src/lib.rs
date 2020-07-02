#![doc(test(attr(forbid(
  unused_variables,
  unused_assignments,
  unused_mut,
  unused_attributes,
  dead_code
))))]

#[cfg(all(not(feature = "with-futures"), feature = "with-wasm_bindgen"))]
mod wasm_bindgen;
