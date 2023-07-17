use ggez::{
    event,
    glam::Vec2,
    conf::WindowMode,
    graphics::{self, Color, DrawParam},
    Context, GameResult
};
use std::{
    env,
    path
};

struct MainState {
    image: graphics::Image
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::from_path(
            ctx, 
            "/ace_spades.png"
        )?;
        let state = MainState {
            image,
        };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let dst = Vec2::new(20.0, 20.0);
        let scl = Vec2::new(0.25, 0.25);
        canvas.draw(
            &self.image,
            DrawParam::new()
                .dest(dst)
                .scale(scl)
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn test_gui() -> GameResult {
    let assets_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let mut win_mode = WindowMode::default();
    win_mode.width = 1020.0;
    win_mode.height = 720.0;
    let cb = ggez::ContextBuilder::new("card", "martinykrz")
        .window_mode(win_mode)
        .add_resource_path(assets_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
