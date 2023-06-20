pub mod logic;
use logic::Game;

fn main() {
    let mut game = Game::default();
    game.play()
}
