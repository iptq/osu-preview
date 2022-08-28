mod timing_point;

use std::pin::Pin;

use libosu::{
  beatmap::Beatmap as LibosuBeatmap, timing::TimingPoint as LibosuTimingPoint,
};
use rosu_pp::{
  Beatmap as RosuBeatmap, BeatmapExt, GradualPerformanceAttributes,
};
use wasm_bindgen::prelude::*;

use self::timing_point::TimingPoint;

#[wasm_bindgen]
pub struct Beatmap {
  inner: LibosuBeatmap,
  rosu_beatmap: Pin<Box<RosuBeatmap>>,
  timing_points: Vec<JsValue>,
}

#[wasm_bindgen]
impl Beatmap {
  #[wasm_bindgen]
  pub fn new(data: String) -> Result<Beatmap, JsError> {
    let inner = LibosuBeatmap::parse(data.as_bytes())
      .map_err(|err| JsError::new(&format!("{}", err)))?;

    let rosu_beatmap = inner
      .convert_to_rosu_beatmap()
      .map_err(|err| JsError::new(&format!("{}", err)))?;

    let timing_points = inner
      .timing_points
      .iter()
      .map(|tp| JsValue::from(TimingPoint::from_libosu(tp)))
      .collect();

    Ok(Self {
      inner,
      rosu_beatmap: Box::pin(rosu_beatmap),
      timing_points,
    })
  }

  #[wasm_bindgen]
  pub fn preview_time(&self) -> i32 { self.inner.preview_time.0 }

  #[wasm_bindgen]
  pub fn title(&self) -> String { self.inner.title.clone() }

  #[wasm_bindgen]
  pub fn artist(&self) -> String { self.inner.artist.clone() }

  #[wasm_bindgen]
  pub fn difficulty_name(&self) -> String { self.inner.difficulty_name.clone() }

  #[wasm_bindgen]
  pub fn game_mode(&self) -> u32 { self.inner.mode as u32 }

  #[wasm_bindgen]
  pub fn pp_calc_at(&mut self, ms: u32) {
    let pp_calc = self.rosu_beatmap.gradual_performance(0);
  }

  #[wasm_bindgen]
  pub fn timing_points(&self) -> Vec<JsValue> { self.timing_points.clone() }
}
