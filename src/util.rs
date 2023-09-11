#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }

    pub fn is_along_same_axis(&self, other: Direction) -> bool {
        match self {
            Direction::Right => other == Direction::Right || other == Direction::Left,
            Direction::Left => other == Direction::Left || other == Direction::Right,
            Direction::Down => other == Direction::Down || other == Direction::Up,
            Direction::Up => other == Direction::Up || other == Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
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

    pub fn split(&self, direction: Direction, proportions: f32) -> (Self, Self) {
        match direction {
            Direction::Right => {
                let w1 = (self.width as f32 * proportions) as i32;
                (Self {
                    x: self.x,
                    y: self.y,
                    width: w1,
                    height: self.height,
                }, Self {
                    x: self.x + w1,
                    y: self.y,
                    width: self.width - w1,
                    height: self.height,
                })
            },
            Direction::Left => {
                let w1 = (self.width as f32 * (1.0 - proportions)) as i32;
                (Self {
                    x: self.x + w1,
                    y: self.y,
                    width: self.width - w1,
                    height: self.height,
                }, Self {
                    x: self.x,
                    y: self.y,
                    width: w1,
                    height: self.height,
                })
            },
            Direction::Down => {
                let h1 = (self.height as f32 * proportions) as i32;
                (Self {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: h1,
                }, Self {
                    x: self.x,
                    y: self.y + h1,
                    width: self.width,
                    height: self.height - h1,
                })
            },
            Direction::Up => {
                let h1 = (self.height as f32 * (1.0 - proportions)) as i32;
                (Self {
                    x: self.x,
                    y: self.y + h1,
                    width: self.width,
                    height: self.height - h1,
                }, Self {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: h1,
                })
            },
        }
    }
}
