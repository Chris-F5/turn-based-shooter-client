use super::{ScreenPos, ScreenRect, TilePos, WorldPos};

pub struct IsoCamera {
    tile_pixel_width: u32,
    tile_width: f64,
    tile_sprite_width: f64,
    scale: f64,
}

impl IsoCamera {
    pub fn new(tile_pixel_width: u32, scale: f64) -> IsoCamera {
        let mut cam = IsoCamera {
            tile_pixel_width,
            scale: 0.0,
            tile_width: 0.0,
            tile_sprite_width: 0.0,
        };
        cam.set_scale(scale);
        cam
    }
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
        self.tile_width = (self.tile_pixel_width - 2) as f64 * scale;
        self.tile_sprite_width = self.tile_pixel_width as f64 * scale;
    }
    pub fn scale(&self) -> f64 {
        self.scale
    }
    pub fn world_to_screen(&self, world_pos: &WorldPos) -> ScreenPos {
        let x = (world_pos.y + world_pos.x) * self.tile_width / 2.0;
        let y = (world_pos.y - world_pos.x) * self.tile_width / -4.0;
        ScreenPos::new(x, y)
    }
    pub fn tile_screen_rect(&self, tile_pos: &TilePos) -> ScreenRect {
        let screen_pos = self.world_to_screen(&tile_pos.world_pos());
        ScreenRect::new(
            screen_pos.x,
            screen_pos.y,
            self.tile_sprite_width,
            self.tile_sprite_width * 0.5,
        )
    }
}
