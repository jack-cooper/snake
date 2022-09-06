mod food;
mod segment;

pub(crate) use {food::Food, segment::Segment};

use std::ops::Add;

use ggez::graphics;
use glam::IVec2;

use crate::{direction::Direction, GRID_CELL_SIZE, GRID_SIZE};

/// A single location within the world.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct GridCell {
    position: IVec2,
}

impl GridCell {
    /// Creates a new `GridCell`.
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: IVec2::new(x, y),
        }
    }

    /// Returns `true` if this `GridCell` lies outside the visible window, else returns `false`.
    pub fn out_of_bounds(&self) -> bool {
        self.position.x < 0
            || self.position.x >= GRID_SIZE.x
            || self.position.y < 0
            || self.position.y >= GRID_SIZE.y
    }
}

impl Add<Direction> for GridCell {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        Self {
            position: self.position + direction.to_ivec2(),
        }
    }
}

impl From<IVec2> for GridCell {
    fn from(position: IVec2) -> Self {
        Self { position }
    }
}

impl From<GridCell> for graphics::Rect {
    fn from(GridCell { position }: GridCell) -> Self {
        graphics::Rect::new_i32(
            (position.x * GRID_CELL_SIZE.x) as i32,
            (position.y * GRID_CELL_SIZE.y) as i32,
            GRID_CELL_SIZE.x as i32,
            GRID_CELL_SIZE.y as i32,
        )
    }
}
