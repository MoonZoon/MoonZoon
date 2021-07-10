// ------ Viewport ------

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Viewport {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

// ------ Viewport ------

#[derive(Debug, Clone, Copy)]
pub struct Scene {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Scene {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
