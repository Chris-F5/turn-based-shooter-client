use super::Image;
use turn_based_shooter_shared::map::TileAppearance;

pub struct GraphicsResources {
    test_tile: Image,
}

impl GraphicsResources {
    pub fn new() -> GraphicsResources {
        GraphicsResources {
            test_tile: Image::new().unwrap(),
        }
    }
    pub fn load(&mut self) {
        self.test_tile.set_src("tile.png");
    }
    pub fn get_tile_image(&mut self, tile: &TileAppearance) -> &Image {
        match tile {
            TileAppearance::TestWhite => &self.test_tile,
        }
    }
}
