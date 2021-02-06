use super::{ScreenPos, ScreenRect};
use turn_based_shooter_shared::battle::{TilePos, WorldPos};

pub struct IsoCamera {
    position: WorldPos,
    tile_pixel_width: u32,
    tile_width: f64,
    tile_sprite_width: f64,
    scale: f64,
}

impl IsoCamera {
    pub fn new(tile_pixel_width: u32, scale: f64) -> IsoCamera {
        let mut cam = IsoCamera {
            position: WorldPos::new(0.0, 5.0),
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
        let rel_world_x = world_pos.x - self.position.x;
        let rel_world_y = world_pos.y - self.position.y;
        let screen_x = (rel_world_y + rel_world_x) * self.tile_width / 2.0;
        let screen_y = (rel_world_y - rel_world_x) * self.tile_width / -4.0;
        ScreenPos::new(screen_x, screen_y)
    }
    pub fn screen_to_world(&self, screen_pos: &ScreenPos) -> WorldPos {
        let rel_world_x = (-2.0 * screen_pos.y + screen_pos.x) / self.tile_width;
        let rel_world_y = (-2.0 * screen_pos.y + screen_pos.x) / self.tile_width;
        let world_x = rel_world_x + self.position.x;
        let world_y = rel_world_y + self.position.y;
        WorldPos::new(world_x, world_y)
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
    pub fn position(&mut self) -> &mut WorldPos {
        &mut self.position
    }
}
