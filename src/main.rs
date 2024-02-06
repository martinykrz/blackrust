pub mod logic;
use logic::*; 

fn main() {
    let mut game: Game = Game::default();
    game.play()
}
