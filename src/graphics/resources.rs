use super::Image;
use turn_based_shooter_shared::battle::map::TileAppearance;

pub struct GraphicsResources {
    white_tile: Image,
    black_tile: Image,
    white_dot: Image,
    black_dot: Image,
}

impl GraphicsResources {
    pub fn new() -> GraphicsResources {
        GraphicsResources {
            white_tile: Image::new().unwrap(),
            black_tile: Image::new().unwrap(),
            white_dot: Image::new().unwrap(),
            black_dot: Image::new().unwrap(),
        }
    }
    pub fn load(&mut self) {
        self.white_tile.set_src("white_tile.png");
        self.black_tile.set_src("black_tile.png");
        self.white_dot.set_src("white_dot.png");
        self.black_dot.set_src("black_dot.png");
    }
    pub fn white_dot(&self) -> &Image {
        &self.white_dot
    }
    pub fn black_dot(&self) -> &Image {
        &self.black_dot
    }
    pub fn get_tile_image(&self, tile: &TileAppearance) -> &Image {
        match tile {
            TileAppearance::TestWhite => &self.white_tile,
            TileAppearance::TestBlack => &self.black_tile,
        }
    }
}
