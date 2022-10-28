use ggez::{event::EventHandler, graphics, input::keyboard, timer, Context, GameResult};

use crate::{
    direction::Direction,
    grid_cell::{Food, GridCell, Segment},
    snake::Snake,
    GRID_SIZE, WINDOW_SIZE,
};

/// Stores all 2 of the objects in the world.
pub struct World {
    food: Food,
    snake: Snake,
}

impl World {
    /// Creates a new `World`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if there is a currently alive snake in the world, else returns `false`.
    fn active(&self) -> bool {
        self.snake.alive()
    }

    /// Resets the current game and starts a new one.
    fn restart(&mut self) {
        *self = Self::default()
    }

    /// The current score of the game, which is equal to the number of food squares eaten.
    fn score(&self) -> usize {
        self.snake.len() - 2
    }

    /// Spawns a new piece of food in the world.
    fn spawn_food(&mut self) {
        let available_cells: Vec<GridCell> = (0..GRID_SIZE.x)
            .zip(0..GRID_SIZE.y)
            .map(Segment::from)
            .filter(|segment| {
                !self.snake.body().contains(segment) && !(self.snake.head() == segment)
            })
            .map(|segment| segment.position())
            .collect();

        let random_cell = available_cells[fastrand::usize(..available_cells.len())];

        self.food = Food::new(random_cell);
    }
}

impl Default for World {
    fn default() -> Self {
        let random_position =
            GridCell::new(fastrand::i32(0..GRID_SIZE.x), fastrand::i32(0..GRID_SIZE.y));

        Self {
            snake: Snake::default(),
            food: Food::new(random_position),
        }
    }
}

impl EventHandler<ggez::GameError> for World {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let target_fps = usize::max(8, self.snake.len());

        while ctx.time.check_update_time(target_fps as u32) {
            if !self.active() {
                continue;
            }

            self.snake.update(&self.food);

            if self.snake.ate_food() {
                self.spawn_food();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        const CLEAR_COLOR: graphics::Color = graphics::Color::new(0.09, 0.09, 0.09, 1.0);

        let canvas = if self.active() {
            let mut canvas = graphics::Canvas::from_frame(ctx, CLEAR_COLOR);

            self.snake.draw(&mut canvas);
            self.food.draw(&mut canvas);

            canvas
        } else {
            let mut canvas = graphics::Canvas::from_frame(ctx, CLEAR_COLOR);

            let mut score_display = graphics::Text::new(format!(
                "Game over! You scored: {}\n\n[Enter] to play again.",
                self.score()
            ));

            score_display.set_bounds(WINDOW_SIZE);
            score_display.set_layout(graphics::TextLayout::center());
            score_display.set_wrap(true);

            canvas.draw(&score_display, graphics::DrawParam::from(WINDOW_SIZE / 2.0));

            canvas
        };

        canvas.finish(ctx)?;

        timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        use keyboard::KeyCode;

        if let Some(direction) = input
            .keycode
            .and_then(|keycode| Direction::try_from(keycode).ok())
        {
            if self.snake.direction_next != self.snake.direction
                && self.snake.direction_next != direction.inverse()
            {
                self.snake.direction_queued = Some(direction);
            } else if self.snake.direction != direction.inverse() {
                self.snake.direction_next = direction;
            }
        }

        if let Some(KeyCode::Return) = input.keycode {
            self.restart();
        }

        Ok(())
    }
}
