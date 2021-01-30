use super::Image;
use super::ScreenRect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}
impl Canvas {
    pub fn new(canvas_attrib_id: &str) -> Canvas {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_attrib_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.set_image_smoothing_enabled(false);

        Canvas { canvas, ctx }
    }
    pub fn draw_image(&mut self, image: &Image, rect: &ScreenRect) {
        self.ctx
            .draw_image_with_html_image_element_and_dw_and_dh(
                image,
                rect.x_pos,
                rect.y_pos,
                rect.x_size,
                rect.y_size,
            )
            .unwrap();
    }
    pub fn clear(&mut self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }
}
