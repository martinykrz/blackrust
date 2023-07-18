use ggez::{
    event,
    glam::Vec2,
    conf::WindowMode,
    graphics::{self, Color, DrawParam},
    // input::keyboard::{KeyCode, KeyInput},
    Context, GameResult
};
// use rand::prelude::*;
use std::{
    env,
    path
};

struct MainState {
    image: graphics::Image,
    dst: Vec2,
    scale: Vec2,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::from_path(ctx, "/ace_spades.png")?;
        let scale = Vec2::new(0.10, 0.10);
        let dst = Vec2::new(0.0, 0.0);
        let state = MainState {
            image,
            dst,
            scale
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

    /* fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let suits = vec!["clubs", "diamonds", "hearts", "spades"];
        let random = random::<usize>() % suits.clone().len();
        let card = match input.keycode {
            Some(KeyCode::Key2) => Some(format!("/02_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key3) => Some(format!("/03_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key4) => Some(format!("/04_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key5) => Some(format!("/05_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key6) => Some(format!("/06_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key7) => Some(format!("/07_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key8) => Some(format!("/08_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Key9) => Some(format!("/09_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::T) => Some(format!("/10_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::J) => Some(format!("/jack_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::Q) => Some(format!("/queen_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::K) => Some(format!("/king_{}.png", suits.get(random).unwrap())),
            Some(KeyCode::A) => Some(format!("/ace_{}.png", suits.get(random).unwrap())),
            _ => None
        };
        match card {
            Some(c) => self.image = graphics::Image::from_path(ctx, c)?,
            None => self.image = self.image.clone(),
        }
        Ok(())
    } */

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.1, 0.2, 0.3, 1.0])
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
                .resizable(false)
        )
        .add_resource_path(assets_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
