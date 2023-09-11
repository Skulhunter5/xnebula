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

    pub fn is_on_same_line(&self, other: Direction) -> bool {
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

    pub fn split(&self, direction: Direction) -> (Self, Self) {
        match direction {
            Direction::Right => (Self {
                x: self.x,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            }, Self {
                x: self.x + self.width / 2,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            }),
            Direction::Left => (Self {
                x: self.x + self.width / 2,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            }, Self {
                x: self.x,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            }),
            Direction::Down => (Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height / 2,
            }, Self {
                x: self.x,
                y: self.y + self.height / 2,
                width: self.width,
                height: self.height / 2,
            }),
            Direction::Up => (Self {
                x: self.x,
                y: self.y + self.height / 2,
                width: self.width,
                height: self.height / 2,
            }, Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height / 2,
            }),
        }
    }

    pub fn split_single(&self, direction: Direction) -> Self {
        match direction {
            Direction::Right => Self {
                x: self.x + self.width / 2,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            },
            Direction::Left => Self {
                x: self.x,
                y: self.y,
                width: self.width / 2,
                height: self.height,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + self.height / 2,
                width: self.width,
                height: self.height / 2,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height / 2,
            }
        }
    }

    /*
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
    }*/
}
