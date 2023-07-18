pub mod logic;
use logic::*; 
use ggez::{
    event,
    glam::Vec2,
    conf::WindowMode,
    graphics::{self, Color, DrawParam},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult
};
use std::{
    env,
    path
};

struct MainState {
    image: graphics::Image,
    dst: Vec2,
    scale: Vec2,
    text: String,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::from_path(ctx, "/ace_spades.png")?;
        let scale = Vec2::new(0.10, 0.10);
        let dst = Vec2::new(0.0, 0.0);
        let text = String::new();
        let state = MainState {
            image,
            dst,
            scale,
            text,
        };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (width, height) = ctx.gfx.size();
        let pos_x = (width - (self.image.clone().width() as f32) * self.scale.x) / 2.0;
        let pos_y = (height - (self.image.clone().height() as f32) * self.scale.y) / 2.0;
        self.dst = Vec2::new(pos_x, pos_y);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let value = match input.keycode {
            Some(KeyCode::Key0) => Some('0'),
            Some(KeyCode::Key1) => Some('1'),
            Some(KeyCode::Key2) => Some('2'),
            Some(KeyCode::Key3) => Some('3'),
            Some(KeyCode::Key4) => Some('4'),
            Some(KeyCode::Key5) => Some('5'),
            Some(KeyCode::Key6) => Some('6'),
            Some(KeyCode::Key7) => Some('7'),
            Some(KeyCode::Key8) => Some('8'),
            Some(KeyCode::Key9) => Some('9'),
            _ => None,
        };
        match value {
            Some(c) => self.text.push(c),
            None => ()
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text = graphics::Text::new(self.text.clone());
        canvas.draw(
            &text,
            DrawParam::new()
                .dest(Vec2::new(10.0, 10.0))
        );
        canvas.draw(
            &self.image,
            DrawParam::new()
                .dest(self.dst)
                .scale(self.scale)
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
    let cb = ggez::ContextBuilder::new("card", "martinykrz")
        .window_mode(
            WindowMode::default()
                .resizable(true)
        )
        .add_resource_path(assets_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

fn main() -> ggez::GameResult {
    /* let mut game = Game::default();
    game.play() */
    test_gui()
}
