#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod direction;
mod grid_cell;
mod snake;
mod world;

pub use world::World;

use glam::{IVec2, Vec2};

/// The size of the world, i.e. the number of unique locations on a given axis.
const GRID_SIZE: IVec2 = IVec2::from_array([30, 20]);
/// The size of an individual grid cell in px, used for drawing.
const GRID_CELL_SIZE: IVec2 = IVec2::from_array([32, 32]);

/// The desired size of the window, in px.
pub const WINDOW_SIZE: Vec2 = Vec2::from_array([
    (GRID_SIZE.x * GRID_CELL_SIZE.x) as f32,
    (GRID_SIZE.y * GRID_CELL_SIZE.y) as f32,
]);
