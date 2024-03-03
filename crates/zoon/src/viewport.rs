#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
}
