use ggez::glam::IVec2;

use super::GridCell;

/// An individual piece of a snake, occupying a [GridCell].
/// This struct is used for both the head and body.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Segment {
    cell: GridCell,
}

impl Segment {
    /// The grid location of this `Segment`.
    pub fn position(&self) -> GridCell {
        self.cell
    }
}

impl From<(i32, i32)> for Segment {
    fn from((x, y): (i32, i32)) -> Self {
        Self::from(GridCell::new(x, y))
    }
}

impl From<GridCell> for Segment {
    fn from(cell: GridCell) -> Self {
        Self { cell }
    }
}

impl From<IVec2> for Segment {
    fn from(vec2: IVec2) -> Self {
        Self {
            cell: GridCell::from(vec2),
        }
    }
}
