pub mod logic;
use logic::*; 

fn main() {
    let mut game: Game = Game::default();
    let mut input: String = String::new();
    println!("Player-Machine [1] or Machine-Machine [2]: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read value");
    let choice = input.trim().parse::<usize>();
    if let Ok(i) = choice {
        if i == 1 {
            game.play()
        } else if i == 2 {
            println!("Put limit: ");
            let mut value: String = String::new();
            std::io::stdin()
                .read_line(&mut value)
                .expect("failed to read limit value");
            let limit: u32 = match value.trim().parse() {
                Ok(i) => i,
                Err(_) => 0
            };
            let stadistics: Vec<GameStatus> = game.basic_strategy_play(limit);
            let mut wins: u32 = 0;
            let mut loses: u32 = 0;
            let mut ties: u32 = 0;
            if stadistics.is_empty() {
                println!("Error: list empty")
            } else {
                for status in stadistics {
                    match status {
                        GameStatus::Win => wins += 1,
                        GameStatus::Tie => ties += 1,
                        GameStatus::Lose => loses += 1
                    }
                }
                println!("Wins: {}, Ties: {}, Loses: {}", wins, ties, loses)
            }
        }
    }
}
