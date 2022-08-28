#[macro_use]
mod macros;
mod errors;
mod render;

use libosu::prelude::Beatmap as LibosuBeatmap;
use rosu_pp::Beatmap as RosuBeatmap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Beatmap {
    inner: LibosuBeatmap,
    rosu_beatmap: RosuBeatmap,
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
        Ok(Self {
            inner,
            rosu_beatmap,
        })
    }

    #[wasm_bindgen]
    pub fn preview_time(&self) -> i32 {
        self.inner.preview_time.0
    }

    #[wasm_bindgen]
    pub fn title(&self) -> String {
        self.inner.title.clone()
    }

    #[wasm_bindgen]
    pub fn artist(&self) -> String {
        self.inner.artist.clone()
    }

    #[wasm_bindgen]
    pub fn difficulty_name(&self) -> String {
        self.inner.difficulty_name.clone()
    }

    #[wasm_bindgen]
    pub fn game_mode(&self) -> u32 {
        self.inner.mode as u32
    }

    #[wasm_bindgen]
    pub fn pp_calc() {}
}
