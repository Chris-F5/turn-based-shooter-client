type Image = web_sys::HtmlImageElement;

pub struct GraphicsResources {
    tile: Image,
}

impl GraphicsResources {
    pub fn new() -> GraphicsResources {
        GraphicsResources {
            tile: Image::new().unwrap(),
        }
    }
    pub fn load(&mut self) {
        self.tile.set_src("tile.png");
    }
    pub fn get_tile(&mut self) -> &Image {
        &self.tile
    }
}
