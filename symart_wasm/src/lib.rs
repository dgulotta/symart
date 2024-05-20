use image::{Pixel, RgbImage};
use serde::ser::Serialize;
use serde_wasm_bindgen::Serializer;
use symart_base::Design;
use symart_designs::lines::Lines;
use symart_designs::quasitrap::Quasitrap;
use symart_designs::squiggles::Squiggles;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen(getter_with_clone)]
pub struct DesignData {
    pub name: String,
    pub schema: JsValue,
    draw_fn: Box<dyn Fn(HtmlCanvasElement, JsValue) -> Result<(), JsValue>>,
}

#[wasm_bindgen]
impl DesignData {
    pub fn draw(&self, ctx: HtmlCanvasElement, params: JsValue) -> Result<(), JsValue> {
        (self.draw_fn)(ctx, params)
    }
}

fn make_design_data<D: Design + 'static>() -> DesignData {
    let name = D::name().to_owned();
    let ser = Serializer::json_compatible();
    let schema = D::schema().serialize(&ser).unwrap();
    let draw_fn = Box::new(design_draw::<D>);
    DesignData {
        name,
        schema,
        draw_fn,
    }
}

#[wasm_bindgen]
pub fn design_data(name: &str) -> Result<DesignData, JsValue> {
    match name {
        "Lines" => Ok(make_design_data::<Lines>()),
        "Quasitrap" => Ok(make_design_data::<Squiggles>()),
        "Squiggles" => Ok(make_design_data::<Quasitrap>()),
        _ => Err("Unrecognized design".into()),
    }
}

fn draw_image(canvas: HtmlCanvasElement, img: &RgbImage) -> Result<(), JsValue> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let sz = 4 * height * width;
    let mut buf = vec![0; sz];
    for (x, y, pix) in img.enumerate_pixels() {
        let pos = width * (y as usize) + (x as usize);
        let rgba = pix.to_rgba();
        buf[(4 * pos)..(4 * (pos + 1))].copy_from_slice(&rgba.0[..]);
    }
    let data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&buf), img.width(), img.height())?;
    canvas.set_width(img.width());
    canvas.set_height(img.height());
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn design_draw<D: Design>(ctx: HtmlCanvasElement, params: JsValue) -> Result<(), JsValue> {
    let design: D = serde_wasm_bindgen::from_value(params)?;
    let response = design.draw().map_err(|e| e.to_string())?;
    draw_image(ctx, &response.im)
}
