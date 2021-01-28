mod camera;
mod canvas;
mod resources;
mod util;

pub use util::{ScreenPos, ScreenRect, TilePos, WorldPos};

use camera::IsoCamera;
use canvas::Canvas;
use resources::GraphicsResources;

pub struct GraphicsContext {
    camera: IsoCamera,
    canvas: Canvas,
    resources: GraphicsResources,
}

impl GraphicsContext {
    pub fn new(html_canvas_attrib_id: &str) -> GraphicsContext {
        let mut resources = GraphicsResources::new();
        resources.load();
        GraphicsContext {
            camera: IsoCamera::new(34, 5.0),
            canvas: Canvas::new(html_canvas_attrib_id),
            resources,
        }
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    pub fn draw_tile(&mut self, pos: &TilePos) {
        let tile = self.resources.get_tile();
        let rect = self.camera.tile_screen_rect(pos);
        self.canvas.draw_image(tile, &rect);
    }
}
