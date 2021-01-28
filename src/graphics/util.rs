#[derive(Clone, Debug)]
pub struct ScreenPos {
    pub x: f64,
    pub y: f64,
}
impl ScreenPos {
    pub fn new(x: f64, y: f64) -> ScreenPos {
        ScreenPos { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct WorldPos {
    pub x: f64,
    pub y: f64,
}
impl WorldPos {
    pub fn new(x: f64, y: f64) -> WorldPos {
        WorldPos { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}
impl TilePos {
    pub fn new(x: u32, y: u32) -> TilePos {
        TilePos { x, y }
    }
    pub fn world_pos(&self) -> WorldPos {
        WorldPos::new(self.x as f64, self.y as f64)
    }
}

#[derive(Clone, Debug)]
pub struct ScreenRect {
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_size: f64,
    pub y_size: f64,
}
impl ScreenRect {
    pub fn new(x_pos: f64, y_pos: f64, x_size: f64, y_size: f64) -> ScreenRect {
        ScreenRect {
            x_pos,
            y_pos,
            x_size,
            y_size,
        }
    }
}
