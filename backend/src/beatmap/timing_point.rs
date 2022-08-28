use libosu::timing::TimingPoint as LibosuTimingPoint;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TimingPoint {}

impl TimingPoint {
  pub fn from_libosu(tp: &LibosuTimingPoint) -> TimingPoint { TimingPoint {} }
}

#[wasm_bindgen]
impl TimingPoint {}
