use std::collections::VecDeque;

use crate::{
    direction::Direction,
    grid_cell::{Food, GridCell, Segment},
    GRID_SIZE,
};

use ggez::graphics;

/// The eponymous Snake! 🐍
pub(crate) struct Snake {
    alive: bool,
    ate_food: bool,
    body: VecDeque<Segment>,
    /// The current direction the snake is heading in.
    pub direction: Direction,
    /// The requested direction for the snake to head to.
    pub direction_next: Direction,
    /// The direction requested to move to after a request has already been made.
    ///
    /// This is predominantly useful for allowing a very quick double tap of two perpendicular
    /// directions to do a 180, despite the fact that the game itself may only be updating at 8 FPS.
    pub direction_queued: Option<Direction>,
    head: Segment,
}

impl Snake {
    /// Returns `true` if this snake is alive and well.
    /// Returns `false` if it ate itself or a wall.
    pub fn alive(&self) -> bool {
        self.alive
    }

    /// Returns `true` if the snake just landed on a food square.
    /// Resets to `false` every update.
    pub fn ate_food(&self) -> bool {
        self.ate_food
    }

    /// The snake's trailing body. Avoid at all costs, lest ye become one with the ouroboroi.
    pub fn body(&self) -> &VecDeque<Segment> {
        &self.body
    }

    /// Draws this `Snake` to the provided `canvas`.
    /// To be called as part of the [draw](ggez::event::EventHandler::draw)
    /// handler provided by `ggez`.
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        const BODY_COLOR: graphics::Color = graphics::Color::new(0.984, 0.749, 0.141, 1.0);
        const HEAD_COLOR: graphics::Color = graphics::Color::new(0.639, 0.902, 0.208, 1.0);

        self.body.iter().for_each(|segment| {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(segment.position().into())
                    .color(BODY_COLOR),
            )
        });

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head.position().into())
                .color(HEAD_COLOR),
        );
    }

    /// Returns `true` if the snake's head is on top of delicious food, else `false`.
    pub fn eats_food(&self, food: &Food) -> bool {
        self.head.position() == food.position()
    }

    /// Returns `true` if the snake's head is on top of its own delicious tail, else `false`.
    pub fn eats_self(&self) -> bool {
        self.body.contains(&self.head)
    }

    /// The segment at the front of this `Snake`.
    pub fn head(&self) -> &Segment {
        &self.head
    }

    /// The number of grid cells this snake covers.
    pub fn len(&self) -> usize {
        self.body.len() + 1
    }

    /// Updates this snake for a new tick. To be called as part of the
    /// [update](ggez::event::EventHandler::update) handler provided by `ggez`.
    pub fn update(&mut self, food: &Food) {
        if self.direction == self.direction_next {
            if let Some(next_dir) = self.direction_queued {
                self.direction_next = next_dir;
                self.direction_queued = None;
            }
        }

        let new_head_position = self.head.position() + self.direction_next;

        if new_head_position.out_of_bounds() {
            self.alive = false;
            return;
        }

        let new_head = Segment::from(new_head_position);
        let old_head = std::mem::replace(&mut self.head, new_head);
        self.body.push_front(old_head);

        if self.eats_self() {
            self.alive = false;
            return;
        }

        self.ate_food = false;

        if self.eats_food(food) {
            self.ate_food = true;
        } else {
            self.body.pop_back();
        }

        self.direction = self.direction_next;
    }
}

impl Default for Snake {
    fn default() -> Self {
        let body_position = GridCell::new(GRID_SIZE.x / 4, GRID_SIZE.y / 2);
        let body = VecDeque::from([Segment::from(body_position)]);

        let direction = Direction::Right;

        Self {
            alive: true,
            ate_food: false,
            body,
            direction,
            direction_next: direction,
            direction_queued: None,
            head: Segment::from(body_position + direction),
        }
    }
}
