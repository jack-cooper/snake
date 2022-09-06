use ggez::graphics;

use super::GridCell;

const FOOD_COLOR: graphics::Color = graphics::Color::new(0.937, 0.267, 0.267, 1.0);

/// A piece of food for the [Snake](crate::snake::Snake) to eat.
pub(crate) struct Food {
    cell: GridCell,
}

impl Food {
    /// Creates a new `Food`.
    pub fn new(cell: GridCell) -> Self {
        Self { cell }
    }

    /// Draws this `Food` to the provided `canvas`. To be called as part of the [draw](ggez::event::EventHandler::draw) handler
    /// provided by `ggez`.
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.cell.into())
                .color(FOOD_COLOR),
        );
    }

    /// The grid location of this `food`.
    ///
    pub fn position(&self) -> GridCell {
        self.cell
    }
}
