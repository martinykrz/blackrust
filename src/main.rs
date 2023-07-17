/* pub mod logic;
use logic::Game; */
pub mod gui;
use gui::test_gui;

fn main() -> ggez::GameResult {
    /* let mut game = Game::default();
    game.play() */
    test_gui()
}
