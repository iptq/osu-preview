use js_sys::Float32Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
  HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader,
};

use crate::errors::{Error, Result};
use crate::Beatmap;

#[wasm_bindgen]
pub struct Renderer {
  canvas: HtmlCanvasElement,
  ctx: WebGl2RenderingContext,
  sprite_fsh: WebGlShader,
  sprite_vsh: WebGlShader,
}

#[wasm_bindgen]
impl Renderer {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: HtmlCanvasElement) -> Result<Renderer, JsValue> {
    let ctx = canvas
      .get_context("webgl2")?
      .unwrap()
      .dyn_into::<WebGl2RenderingContext>()?;

    let sprite_fsh = include_str!("../shaders/sprite.fsh");
    let sprite_fsh = compile_shader(
      &ctx,
      WebGl2RenderingContext::FRAGMENT_SHADER,
      sprite_fsh,
    )?;
    let sprite_vsh = compile_shader(
      &ctx,
      WebGl2RenderingContext::VERTEX_SHADER,
      include_str!("../shaders/sprite.vsh"),
    )?;

    Ok(Renderer {
      canvas,
      ctx,
      sprite_fsh,
      sprite_vsh,
    })
  }

  #[wasm_bindgen]
  pub fn render(&self, beatmap: &Beatmap) -> Result<(), JsValue> {
    self.render_inner(beatmap).map_err(|err| match err {
      Error::Js(js) => js,
      Error::Other(other) => JsError::new(&format!("{}", other)).into(),
    })
  }

  fn render_inner(&self, beatmap: &Beatmap) -> Result<()> {
    let program = link_program(&self.ctx, &self.sprite_vsh, &self.sprite_fsh)?;
    self.ctx.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location =
      self.ctx.get_attrib_location(&program, "position");
    let buffer = self.ctx.create_buffer().ok_or("Failed to create buffer")?;
    self
      .ctx
      .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
      let positions_array_buf_view = Float32Array::view(&vertices);

      self.ctx.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &positions_array_buf_view,
        WebGl2RenderingContext::STATIC_DRAW,
      );
    }

    let vao = self
      .ctx
      .create_vertex_array()
      .ok_or("Could not create vertex array object")?;
    self.ctx.bind_vertex_array(Some(&vao));

    self.ctx.vertex_attrib_pointer_with_i32(
      0,
      3,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );
    self
      .ctx
      .enable_vertex_attrib_array(position_attribute_location as u32);

    self.ctx.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&self.ctx, vert_count);

    Ok(())
  }
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
  context: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context.get_program_info_log(&program).unwrap_or_else(|| {
        String::from("Unknown error creating program object")
      }),
    )
  }
}
