use ggez::{glam::IVec2, input::keyboard::KeyCode};

/// A direction which a [Snake](crate::snake::Snake) can travel in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    /// The direction opposite the current direction.
    pub fn inverse(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }

    /// Converts this `Direction` to a 2D Vector.
    pub fn to_ivec2(self) -> IVec2 {
        match self {
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
            Direction::Right => IVec2::X,
            Direction::Up => IVec2::NEG_Y,
        }
    }
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();

    fn try_from(key: KeyCode) -> Result<Self, Self::Error> {
        match key {
            KeyCode::Down | KeyCode::S => Ok(Direction::Down),
            KeyCode::Left | KeyCode::A => Ok(Direction::Left),
            KeyCode::Right | KeyCode::D => Ok(Direction::Right),
            KeyCode::Up | KeyCode::W => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
