use crate::GraphicsContext;
use turn_based_shooter_shared::map::{Map, TilePos};

pub struct Battle {
    map: Map,
}
impl Battle {
    pub fn new(map: Map) -> Battle {
        Battle { map }
    }
    pub fn draw(&self, ctx: &mut GraphicsContext) {
        for y in 0..self.map.y_size() {
            for x in 0..self.map.x_size() {
                let pos = TilePos::new(x, y);
                let tile = self.map.get_tile(&pos);
                ctx.draw_tile(&pos, tile.appearance());
            }
        }
    }
}
