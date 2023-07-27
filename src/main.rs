pub mod logic;
// use logic::*; 
use ggez::{
    event,
    glam::Vec2,
    conf::WindowMode,
    graphics::{self, Color, DrawParam},
    // input::keyboard::{KeyCode, KeyInput},
    Context, GameResult
};
use std::{
    env,
    path
};

#[derive(Clone)]
struct ImageCard {
    image: graphics::Image,
    dst: Vec2,
}

struct MainState {
    card_1: ImageCard,
    card_2: ImageCard,
    card_3: ImageCard,
    card_4: ImageCard,
    scale: Vec2,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let scale = Vec2::new(0.10, 0.10);
        let dst = Vec2::new(0.0, 0.0);
        let card_1 = ImageCard {
            image: graphics::Image::from_path(ctx, "/ace_spades.png")?,
            dst,
        };
        let card_2 = ImageCard {
            image: graphics::Image::from_path(ctx, "/ace_hearts.png")?,
            dst,
        };
        let card_3 = ImageCard {
            image: graphics::Image::from_path(ctx, "/ace_clubs.png")?,
            dst,
        };
        let card_4 = ImageCard {
            image: graphics::Image::from_path(ctx, "/ace_diamonds.png")?,
            dst,
        };
        let state = MainState {
            card_1,
            card_2,
            card_3,
            card_4,
            scale,
        };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // General formula
        // 1. width + 0.0 - (image_width) / 2.0
        // 2. width - 150.0 - (image_width) / 2.0 ; width + 150.0 - (image_width) / 2.0
        // 3. width - 300.0 - (image_width) / 2.0 ; width + 0.0 - (image_width) / 2.0 ; width + 300.0 - (image_width) / 2.0
        let (width, height) = ctx.gfx.size();
        let card_1_pos_x = (width - 450.0 - (self.card_1.clone().image.width() as f32) * self.scale.x) / 2.0;
        let card_2_pos_x = (width - 150.0 - (self.card_2.clone().image.width() as f32) * self.scale.x) / 2.0;
        let card_3_pos_x = (width + 150.0 - (self.card_3.clone().image.width() as f32) * self.scale.x) / 2.0;
        let card_4_pos_x = (width + 450.0 - (self.card_3.clone().image.width() as f32) * self.scale.x) / 2.0;
        let pos_y = height - (self.card_1.clone().image.height() as f32) * self.scale.y;
        self.card_1.dst = Vec2::new(card_1_pos_x, pos_y);
        self.card_2.dst = Vec2::new(card_2_pos_x, pos_y);
        self.card_3.dst = Vec2::new(card_3_pos_x, pos_y);
        self.card_4.dst = Vec2::new(card_4_pos_x, pos_y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.1, 0.2, 0.3, 1.0])
        );
        canvas.draw(
            &self.card_1.image,
            DrawParam::new()
                .dest(self.card_1.dst)
                .scale(self.scale)
        );
        canvas.draw(
            &self.card_2.image,
            DrawParam::new()
                .dest(self.card_2.dst)
                .scale(self.scale)
        );
        canvas.draw(
            &self.card_3.image,
            DrawParam::new()
                .dest(self.card_3.dst)
                .scale(self.scale)
        );
        canvas.draw(
            &self.card_4.image,
            DrawParam::new()
                .dest(self.card_4.dst)
                .scale(self.scale)
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
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
