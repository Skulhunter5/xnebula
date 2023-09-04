#[derive(PartialEq, Clone)]
#[repr(u8)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Side {
    Left,
    Right,
}

impl Bounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    // TODO: correct for integer division inaccuracies
    pub fn split(&self, side: Side) -> Self {
        match side {
            Side::Left => {
                Self {
                    x: self.x,
                    y: self.y,
                    width: self.width / 2,
                    height: self.height,
                }
            }
            Side::Right => {
                Self {
                    x: self.x + self.width / 2,
                    y: self.y,
                    width: self.width / 2,
                    height: self.height,
                }
            }
        }
    }
}
