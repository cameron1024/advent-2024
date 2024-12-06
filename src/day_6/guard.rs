#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

/// A combination of the position of a guard, and the direction they are facing
#[derive(Debug, Clone, Copy, Default)]
pub struct GuardPos {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl GuardPos {
    /// Returns a [`GuardPos`] representing the location after one move forward
    ///
    /// If this move would move the guard off the map, `None` is returned
    pub fn move_forward(&self, width: usize, height: usize) -> Option<Self> {
        let would_leave = match self.direction {
            Direction::Up => self.y == 0,
            Direction::Down => self.y == height - 1,
            Direction::Left => self.x == 0,
            Direction::Right => self.x == width - 1,
        };

        if would_leave {
            return None;
        }

        let mut copy = *self;
        match self.direction {
            Direction::Up => copy.y -= 1,
            Direction::Down => copy.y += 1,
            Direction::Left => copy.x -= 1,
            Direction::Right => copy.x += 1,
        }

        Some(copy)
    }
}
