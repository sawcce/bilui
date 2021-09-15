pub struct Style {
    margin: u64,
    padding: u64,
}

#[derive(Copy, Clone)]
pub struct Size {
    pub x: u32,
    pub y: u32,
}

impl Size {
    pub fn default() -> Size {
        return Size {
            x:0,
            y:0,
        }
    }

    pub fn new(width: u32, height: u32) -> Size {
        return Size {
            x: width,
            y: height,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Computed {
    pub size: Size,
}


impl Computed {
    pub fn new() -> Computed {
        return Computed {
            size: Size::default(),
        }
    }
}