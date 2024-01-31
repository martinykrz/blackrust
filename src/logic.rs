use rand::seq::SliceRandom;
use std::io;

#[derive(Copy, Clone)]
pub struct Card {
    suit: char,
    rank: char,
}

impl Card {
    fn view_card(&self) {
        if self.suit == '\u{2665}' || self.suit == '\u{2666}' {
            if self.rank == 'T' {
                print!("\x1b[31;49;1m10{}\x1b[0m", self.suit)
            } else {
                print!("\x1b[31;49;1m{}{}\x1b[0m", self.rank, self.suit)
            }
        } else {
            if self.rank == 'T' {
                print!("10{}", self.suit)
            } else {
                print!("{}{}", self.rank, self.suit)
            }
        }
    }
}

pub struct Money {
    wallet: u32,
    bet: u32,
    split_bet: u32,
    last_bet: u32,
}

impl Default for Money {
    fn default() -> Self {
        Money {
            wallet: 0,
            bet: 0,
            split_bet: 0,
            last_bet: 0
        }
    }
}

impl Money {
    fn make_wallet(&mut self) {
        println!("How much money do you have? ");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read value");
        let money = input.trim().parse::<u32>();
        self.wallet = match money {
            Ok(i) => i,
            Err(_) => 0,
        };
    }

    fn make_bet(&mut self, split: bool) {
        if split {
            println!("How much to the split? ");
        } else {
           println!("How much do you bet? ");
        }
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read value");
        let bet = input.trim();
        if split {
            self.split_bet = match bet.parse::<u32>() {
                Ok(i) => i,
                Err(_) => if self.last_bet != 0 { self.last_bet } else { 0 },
            };
        } else {
            self.bet = match bet.parse::<u32>() {
                Ok(i) => i,
                Err(_) => if self.last_bet != 0 { self.last_bet } else { 0 },
            };
        }
        self.last_bet = self.bet;
        self.wallet -= self.bet;
    }

    fn double(&mut self, split: bool) {
        self.wallet -= if split { self.split_bet } else { self.bet };
        if split { self.split_bet *= 2 } else { self.bet *= 2 } ;
    }

    fn win(&mut self, split: bool) {
        self.wallet += if split { self.split_bet } else { self.bet };
    }

    fn view_money(&self) {
        if self.bet != 0 {
            println!("Wallet: {}\nBet: {}", self.wallet, self.bet)
        } else {
            println!("Wallet: {}", self.wallet)
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            cards: {
                let mut deck = Vec::new();
                let ranks: [char;13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'A', 'J', 'Q', 'K'];
                let suits: [char;4] = ['\u{2660}', '\u{2665}', '\u{2663}', '\u{2666}'];
                for suit in suits {
                    for rank in ranks {
                        deck.push(Card{ suit, rank });
                    }
                }
                deck.shuffle(&mut rand::thread_rng());
                deck
            }
        }
    }
}

impl Deck {
    fn hit(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
    split: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Vec::new(),
            split: Vec::new()
        }
    }
}

impl Hand {
    fn add_card(&mut self, card: Card, split: bool) {
        if split {
            self.split.push(card)
        } else {
            self.cards.push(card)
        }
    }

    fn make_split(&mut self) -> bool {
        let mut split: bool = false;
        if self.cards.len() == 2 {
            if self.cards.clone().first().unwrap().rank == self.cards.clone().last().unwrap().rank {
                split = true;
                let last_card: Option<Card> = self.cards.pop();
                match last_card {
                    Some(c) => self.split.push(c),
                    None => {},
                }
            }
        }
        split
    }

    fn get_value(&self) -> (u8, u8) {
        let mut value_main: u8 = 0;
        let mut value_split: u8 = 0;
        let mut has_ace: bool = false;
        for card in self.cards.clone() {
            value_main += match card.rank {
                'T' | 'J' | 'Q' | 'K' => 10,
                'A' => 1,
                _ => (card.rank as u8) - ('0' as u8)
            };
            has_ace |= card.rank == 'A';
        }
        if has_ace && value_main + 10 <= 21 {
            value_main += 10;
        }
        if !self.split.is_empty() {
            has_ace = false;
            for card in self.split.clone() {
                value_split += match card.rank {
                    'T' | 'J' | 'Q' | 'K' => 10,
                    'A' => 1,
                    _ => (card.rank as u8) - ('0' as u8)
                };
                has_ace |= card.rank == 'A';
            }
            if has_ace && value_split + 10 <= 21 {
                value_split += 10;
            }
        }
        (value_main, value_split)
    }

    fn clear_hand(&mut self) {
        self.cards.clear();
        self.split.clear();
    }

    fn is_blackjack(&self) -> bool {
        let len: bool = self.cards.len() == 2;
        let mut case: bool = true;
        for card in self.cards.clone() {
            case &= match card.rank {
                'J' | 'Q' | 'K' => true,
                _ => false,
            };
        }
        len && case
    }

    fn view_hand(&self) {
        if self.split.is_empty() {
            for card in self.cards.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().0);
        } else {
            print!("Hand 1: ");
            for card in self.cards.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().0);
            print!("Hand 2: ");
            for card in self.split.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().1);
        }
    }
}

pub enum GameStatus {
    Start,
    Win,
    Tie,
    Lose,
}

pub struct Game {
    deck: Deck,
    money: Money,
    player_hand: Hand,
    dealer_hand: Hand,
    game_status: GameStatus,
}

impl Default for Game {
    fn default() -> Self {
        Game { 
            deck: Deck::default(), 
            money: Money::default(), 
            player_hand: Hand::default(), 
            dealer_hand: Hand::default(), 
            game_status: GameStatus::Start,
        }
    }
}

impl Game {
    fn player_turn(&mut self) {
        let mut split: bool = false;
        while self.player_hand.get_value().0 < 21 {
            println!("Hit, sPlit, Stand or Double? ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read value");
            let choice = input
                .trim()
                .parse::<char>();
            match choice {
                Ok('h') => {
                    self.player_hand
                        .add_card(self.deck.hit(), split);
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('d') => {
                    self.money.double(split);
                    self.player_hand
                        .add_card(self.deck.hit(), split);
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('p') => {
                    split = self.player_hand.make_split();
                    if split {
                        self.money.make_bet(split);
                        println!("Player's hand: ");
                        self.player_hand.view_hand();
                    } else {
                        println!("Conditions not met to split!");
                        continue;
                    }
                },
                Ok('s') => break,
                _ => continue
            }
        }
        if split {
            println!("Split Hand");
            while self.player_hand.get_value().1 < 21 {
                println!("Hit, Stand or Double? ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read value");
                let choice = input
                    .trim()
                    .parse::<char>();
                match choice {
                    Ok('h') => {
                        self.player_hand
                            .add_card(self.deck.hit(), split);
                        println!("Player's hand: ");
                        self.player_hand.view_hand();
                    },
                    Ok('d') => {
                        self.money.double(split);
                        self.player_hand
                            .add_card(self.deck.hit(), split);
                        println!("Player's hand: ");
                        self.player_hand.view_hand();
                    },
                    Ok('s') => break,
                    _ => continue
                }
            }
        }
    }

    fn dealer_turn(&mut self) {
        while self.dealer_hand.get_value().0 < 17 {
            self.dealer_hand.add_card(self.deck.hit(), false);
        }
        println!("Dealer's hand: ");
        self.dealer_hand.view_hand();
    }

    fn determine_winner(&mut self) {
        if self.player_hand.is_blackjack() {
            if self.dealer_hand.is_blackjack() {
                self.game_status = GameStatus::Tie;
                println!("It's a tie.");
            }
            else {
                self.game_status = GameStatus::Win;
                println!("You win.");
                self.money.win(false);
            }
        } else {
            let player: (u8, u8) = self.player_hand.get_value();
            let dealer: u8 = self.dealer_hand.get_value().0;
            let main_player: u8 = player.0;
            let other_player: u8 = player.1;
            if main_player <= 21 && dealer != 21 {
                if dealer > 21 || (dealer < 21 && main_player > dealer) {
                    self.game_status = GameStatus::Win;
                    println!("You win.");
                    self.money.win(false);
                } else {
                    self.game_status = GameStatus::Lose;
                    println!("You lose.");
                }
            } else if main_player == dealer {
                self.game_status = GameStatus::Tie;
                println!("It's a tie.");
            } else {
                self.game_status = GameStatus::Lose;
                println!("You lose.");
            }
            if other_player != 0 {
                if other_player <= 21 && dealer != 21 {
                    if dealer > 21 || (dealer < 21 && other_player > dealer) {
                        self.game_status = GameStatus::Win;
                        println!("You win.");
                        self.money.win(true);
                    } else {
                        self.game_status = GameStatus::Lose;
                        println!("You lose.");
                    }
                } else if other_player == dealer {
                    self.game_status = GameStatus::Tie;
                    println!("It's a tie.");
                } else {
                    self.game_status = GameStatus::Lose;
                    println!("You lose.");
                }
            }
        }
    }

    fn init_game(&mut self) {
        self.player_hand.clear_hand();
        self.dealer_hand.clear_hand();
        for _ in 0..2 {
            self.player_hand.add_card(self.deck.hit(), false);
            self.dealer_hand.add_card(self.deck.hit(), false);
        }
        if self.deck.cards.len() != 0 {
            self.money.make_bet(false);
            self.money.view_money();
            println!("Dealer's hand: ");
            self.dealer_hand.cards
                .first()
                .unwrap()
                .view_card();
            println!("\nPlayer's hand: ");
            self.player_hand.view_hand();
        } else {
            println!("Game Over!");
        }
    }

    pub fn play(&mut self) {
        self.money.make_wallet();
        while self.money.wallet > 0 {
            self.init_game();
            self.player_turn();
            if self.player_hand.get_value().0 <= 21 || self.player_hand.get_value().1 <= 21 {
                self.dealer_turn();
            }
            self.determine_winner();
        }
    }
}
