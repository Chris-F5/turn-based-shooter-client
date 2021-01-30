use super::Image;
use turn_based_shooter_shared::map::TileAppearance;

pub struct GraphicsResources {
    white_tile: Image,
    black_tile: Image,
}

impl GraphicsResources {
    pub fn new() -> GraphicsResources {
        GraphicsResources {
            white_tile: Image::new().unwrap(),
            black_tile: Image::new().unwrap(),
        }
    }
    pub fn load(&mut self) {
        self.white_tile.set_src("white_tile.png");
        self.black_tile.set_src("black_tile.png");
    }
    pub fn get_tile_image(&mut self, tile: &TileAppearance) -> &Image {
        match tile {
            TileAppearance::TestWhite => &self.white_tile,
            TileAppearance::TestBlack => &self.black_tile,
        }
    }
}
