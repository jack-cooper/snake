use ggez::{conf, event, ContextBuilder, GameResult};
use snake::{World, WINDOW_SIZE};

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("snake", "Jack Cooper")
        .window_setup(conf::WindowSetup::default().title("Snake"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_SIZE.x, WINDOW_SIZE.y))
        .build()?;

    event::run(ctx, event_loop, World::new())
}
