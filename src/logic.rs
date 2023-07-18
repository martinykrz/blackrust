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

    fn view_card_image(&self) -> String {
        let suit = match self.suit {
            '\u{2260}' => "spades",
            '\u{2263}' => "clubs",
            '\u{2265}' => "hearts",
            '\u{2266}' => "diamonds",
            _ => ""
        };
        let rank = match self.rank {
            'T' => "10".to_string(),
            'J' => "jack".to_string(),
            'Q' => "queen".to_string(),
            'K' => "king".to_string(),
            'A' => "ace".to_string(),
            _ => format!("0{}", self.rank)
        };
        format!("/{}_{}.png", rank, suit)
    }
}

struct Money {
    wallet: u32,
    bet: u32,
    last_bet: u32,
}

impl Default for Money {
    fn default() -> Self {
        Money {
            wallet: 0,
            bet: 0,
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

    fn make_bet(&mut self) {
        println!("How much do you bet? ");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read value");
        let bet = input.trim();
        self.bet = match bet.parse::<u32>() {
            Ok(i) => i,
            Err(_) => if self.last_bet != 0 { self.last_bet } else { 0 },
        };
        self.last_bet = self.bet;
        self.wallet -= self.bet;
    }

    fn double(&mut self) {
        self.wallet -= self.bet;
        self.bet *= 2;
    }

    fn win(&mut self) {
        self.wallet += self.bet;
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
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Vec::new()
        }
    }
}

impl Hand {
    fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn get_value(&self) -> u8 {
        let mut value: u8 = 0;
        let mut has_ace: bool = false;
        for card in self.cards.clone() {
            value += match card.rank {
                'T' | 'J' | 'Q' | 'K' => 10,
                'A' => 1,
                _ => (card.rank as u8) - ('0' as u8)
            };
            has_ace |= card.rank == 'A';
        }
        if has_ace && value + 10 <= 21 {
            value += 10;
        }
        value
    }

    fn clear_hand(&mut self) {
        self.cards.clear()
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
        for card in self.cards.clone() {
            card.view_card();
            print!(", ");
        }
        println!();
    }

    fn view_hand_images(&self) -> Vec<String> {
        self.cards
            .iter()
            .map(|card| card.view_card_image())
            .collect()
    }
}

pub struct Game {
    deck: Deck,
    money: Money,
    player_hand: Hand,
    dealer_hand: Hand,
    /* Game Status
     * -1: dealers win
     * 0: tie
     * 1: player win
    */
    game_status: i8,
}

impl Default for Game {
    fn default() -> Self {
        Game { 
            deck: Deck::default(), 
            money: Money::default(), 
            player_hand: Hand::default(), 
            dealer_hand: Hand::default(), 
            game_status: 0
        }
    }
}

impl Game {
    fn player_turn(&mut self) {
        while self.player_hand.get_value() < 21 {
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
                        .add_card(self.deck.hit());
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('d') => {
                    self.money.double();
                    self.player_hand
                        .add_card(self.deck.hit());
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('s') => break,
                _ => continue
            }
        }
    }

    fn dealer_turn(&mut self) {
        while self.dealer_hand.get_value() < 17 {
            self.dealer_hand.add_card(self.deck.hit());
        }
        println!("Dealer's hand: ");
        self.dealer_hand.view_hand();
    }

    fn determine_winner(&mut self) {
        if self.player_hand.is_blackjack() {
            if self.dealer_hand.is_blackjack() {
                self.game_status = 0;
                println!("It's a tie.");
            }
            else {
                self.game_status = 1;
                println!("You win.");
                self.money.win();
            }
        } else {
            let player: u8 = self.player_hand.get_value();
            let dealer: u8 = self.dealer_hand.get_value();
            if player <= 21 && dealer != 21 {
                if dealer > 21 || (dealer < 21 && player > dealer) {
                    self.game_status = 1;
                    println!("You win.");
                    self.money.win();
                } else {
                    self.game_status = -1;
                    println!("You lose.");
                }
            } else if player == dealer {
                self.game_status = 0;
                println!("It's a tie.");
            } else {
                self.game_status = -1;
                println!("You lose.");
            }
        }
    }

    fn init_game(&mut self) {
        self.player_hand.clear_hand();
        self.dealer_hand.clear_hand();
        for _ in 0..2 {
            self.player_hand.add_card(self.deck.hit());
            self.dealer_hand.add_card(self.deck.hit());
        }
        if self.deck.cards.len() != 0 {
            self.money.make_bet();
            self.money.view_money();
            println!("Dealer's hand: ");
            self.dealer_hand.cards.first().unwrap().view_card();
            println!("\nPlayer's hand: ");
            self.player_hand.view_hand();
        } else {
            println!("Game Over!");
        }
    }

    pub fn play(&mut self) {
        while self.money.wallet > 0 {
            self.init_game();
            self.player_turn();
            if self.player_hand.get_value() <= 21 {
                self.dealer_turn();
            }
            self.determine_winner();
        }
    }
}
