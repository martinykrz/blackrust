use std::io::{self, BufRead};

struct Card {
    suit: char,
    rank: char,
}

impl Card {
    fn view_card(&self) {
        if self.suit == '\u{2665}' || self.suit == '\u{2666}' {
            print!("\x1b[31;49;1m{}{}\x1b[0m", self.rank, self.suit)
        } else {
            print!("{}{}", self.rank, self.suit)
        }
    }
}

struct Money {
    wallet: u32,
    bet: u32,
    last_bet: u32,
}

impl Money {
    fn make_bet(&self, bet: u32) {
        self.bet = bet;
        self.last_bet = self.bet;
        self.wallet -= self.bet;
    }

    fn double(&self) {
        self.wallet -= self.bet;
        self.bet *= 2;
    }

    fn win(&self) {
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

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn hit(&self) -> Card {
        self.cards.pop().unwrap()
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn add_card(&self, card: Card) {
        self.cards.push(card)
    }

    fn get_value(&self) -> u8 {
        let mut value: u8 = 0;
        let mut has_ace: bool = false;
        for card in self.cards {
            value += match card.rank {
                'T' | 'J' | 'Q' | 'K' => 10,
                'A' => 1,
                _ => card.rank.to_digit(10).unwrap() as u8
            };
            has_ace |= card.rank == 'A';
        }
        if has_ace && value + 10 <= 21 {
            value += 10;
        }
        value
    }

    fn clear_hand(&self) {
        self.cards.clear()
    }

    fn same_value(&self) -> bool {
        if self.cards.len() == 2 {
            self.cards[0].rank == self.cards[1].rank
        }
    }

    fn is_blackjack(&self) -> bool {
        let mut len: bool = self.cards.len() == 2;
        let mut case: bool = true;
        for card in self.cards {
            case &= match card.rank {
                'J' | 'Q' | 'K' => true,
                _ => false,
            };
        }
        len && case
    }

    fn view_hand(&self) {
        for card in self.cards {
            card.view_card();
            print!(", ");
        }
    }
}

struct Game {
    deck: Deck,
    money: Money,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl Game {
    fn player_turn(&self) {
        while self.player_hand.get_value() < 21 {
            let stdin = io::stdin();
            let mut choice = String::new();
            stdin
                .lock()
                .read_line(&mut choice)
                .unwrap();
            match *choice
                .trim()
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .to_lowercase() {
                    'h' => {
                        self.player_hand
                            .add_card(self.deck.hit());
                        print!("Player's hand: ");
                        self.player_hand.view_hand();
                    }
                    //TODO
                }
        }
    }
}
