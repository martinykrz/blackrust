use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

pub enum GameStatus {
    Win,
    Tie,
    Lose,
}

pub enum Decision {
    Stand,
    Hit,
    Double,
    Split,
    None,
}

struct Values {
    main: u8,
    split: u8
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Card {
    rank: char,
    suit: char,
    value: u8
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
                        let value: u8 = match rank {
                            'T' | 'J' | 'Q' | 'K' => 10,
                            'A' => 1,
                            _ => (rank as u8) - ('0' as u8)
                        };
                        deck.push(Card{ rank, suit, value });
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
        let last_card = self.cards.pop().unwrap();
        self.cards.push(last_card.clone());
        self.cards.shuffle(&mut rand::thread_rng());
        last_card
    }
}

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
    split: Vec<Card>,
    ace_cards: bool,
    ace_split: bool,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Vec::new(),
            split: Vec::new(),
            ace_cards: false,
            ace_split: false
        }
    }
}

impl Hand {
    fn add_card(&mut self, card: Card, split: bool) {
        if split {
            self.ace_split |= card.rank == 'A';
            self.split.push(card);
            self.split.sort_by_key(|c| c.value)
        } else {
            self.ace_cards |= card.rank == 'A';
            self.cards.push(card);
            self.cards.sort_by_key(|c| c.value)
        }
    }

    fn make_split(&mut self) -> bool {
        let mut res: bool = false;
        if self.cards.len() == 2 {
            if self.cards.clone().first().unwrap().rank == self.cards.clone().last().unwrap().rank {
                res = true;
                let last_card: Option<Card> = self.cards.pop();
                match last_card {
                    Some(c) => {
                        self.ace_split |= c.rank == 'A';
                        self.split.push(c);
                    },
                    None => {},
                }
            }
        }
        res
    }

    fn get_value(&mut self) -> Values {
        let mut values: Values = Values {
            main: 0,
            split: 0
        };
        for card in self.cards.clone() {
            values.main += card.value;
        }
        if self.ace_cards && values.main + 10 <= 21 {
            values.main += 10;
        }
        if !self.split.is_empty() {
            for card in self.split.clone() {
                values.split += card.value;
            }
            if self.ace_split && values.split + 10 <= 21 {
                values.split += 10;
            }
        }
        values
    }

    fn clear_hand(&mut self) {
        self.cards.clear();
        self.split.clear();
    }

    fn is_blackjack(&mut self) -> bool {
        let mut res: bool = self.get_value().main == 21 || self.get_value().split == 21;
        for card in self.cards.clone() {
            res &= match card.rank {
                'A' | 'J' | 'Q' | 'K' => true,
                _ => false,
            };
        }
        self.cards.len() == 2 && res
    }

    pub fn view_hand(&mut self) {
        if self.split.is_empty() {
            for card in self.cards.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().main);
        } else {
            print!("Hand 1: ");
            for card in self.cards.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().main);
            print!("Hand 2: ");
            for card in self.split.clone() {
                card.view_card();
                print!(", ");
            }
            println!("\nValue: {}", self.get_value().split);
        }
    }
}

pub struct Game {
    deck: Deck,
    money: Money,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl Default for Game {
    fn default() -> Self {
        Game { 
            deck: Deck::default(), 
            money: Money::default(), 
            player_hand: Hand::default(), 
            dealer_hand: Hand::default(), 
        }
    }
}

impl Game {
    fn player_decision(&mut self, split: bool) -> bool {
        let mut res: bool = false;
        let value: u8 = if split { 
            self.player_hand.get_value().split 
        } else { 
            self.player_hand.get_value().main 
        };
        while value < 21 {
            if split { 
                println!("Hit, Stand or Double? ") 
            } else { 
                println!("Hit, sPlit, Stand or Double? ") 
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read value");
            let choice = input
                .trim()
                .parse::<char>();
            match choice {
                Ok('h') => {
                    self.player_hand.add_card(self.deck.hit(), split);
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('d') => {
                    self.money.double(split);
                    self.player_hand.add_card(self.deck.hit(), split);
                    println!("Player's hand: ");
                    self.player_hand.view_hand();
                },
                Ok('p') => {
                    if !split {
                        if self.player_hand.make_split() {
                            res = true;
                            self.money.make_bet(true);
                            println!("Player's hand: ");
                            self.player_hand.view_hand();
                        } else {
                            println!("Conditions not met to split!");
                            continue;
                        }
                    } else {
                        println!("Can't split a split!");
                        continue;
                    }
                },
                Ok('s') => break,
                _ => continue
            }
        }
        res
    }

    fn player_turn(&mut self) -> bool {
        let split: bool = self.player_decision(false);
        if split {
            let _ = self.player_decision(true);
        }
        split
    }

    fn dealer_turn(&mut self) {
        while self.dealer_hand.get_value().main < 17 {
            self.dealer_hand.add_card(self.deck.hit(), false);
        }
        println!("Dealer's hand: ");
        self.dealer_hand.view_hand();
    }

    fn winner(&mut self, output: bool, split: bool) -> GameStatus {
        let mut status: GameStatus = GameStatus::Lose;
        let player: Values = self.player_hand.get_value();
        let dealer: u8 = self.dealer_hand.get_value().main;
        let value = if split { player.split } else { player.main };
        let mut wins: bool = value <= 21 && (dealer > 21 || value > dealer);
        wins |= self.player_hand.is_blackjack() && !self.dealer_hand.is_blackjack();
        if wins {
            status = GameStatus::Win;
            self.money.win(split);
        } else if dealer > 21 || value == dealer {
            status = GameStatus::Tie;
        }
        if output {
            match status {
                GameStatus::Win => println!("You win."),
                GameStatus::Tie => println!("It's a tie."),
                GameStatus::Lose => println!("You lose.")
            }
        }
        status
    }

    fn determine_winner(&mut self, output: bool, split: bool) -> Vec<GameStatus> {
        let mut status: Vec<GameStatus> = vec![self.winner(output, false)];
        if split {
            status.push(self.winner(output, true))
        }
        status
    }

    fn init_game(&mut self) {
        self.player_hand.clear_hand();
        self.dealer_hand.clear_hand();
        for _ in 0..2 {
            self.player_hand.add_card(self.deck.hit(), false);
            self.dealer_hand.add_card(self.deck.hit(), false);
        }
        self.money.make_bet(false);
        self.money.view_money();
        println!("Dealer's hand: ");
        self.dealer_hand.cards
            .first()
            .unwrap()
            .view_card();
        println!("\nPlayer's hand: ");
        self.player_hand.view_hand();
    }

    //TODO: Apply split values evaluation
    fn basic_strategy_hard_totals(&mut self) -> Decision {
        let face_up_card: Option<&Card> = self.dealer_hand.cards.first();
        let player_value = self.player_hand.get_value().main;
        if let Some(card) = face_up_card {
            match card.rank {
                '2' => { 
                    if player_value >= 13 { 
                        Decision::Stand 
                    } else if player_value == 12 || player_value <= 9 {
                        Decision::Hit
                    } else {
                        Decision::Double
                    }
                },
                '3' => {
                    if player_value >= 13 {
                        Decision::Stand
                    } else if player_value == 12 || player_value <= 8 {
                        Decision::Hit
                    } else {
                        Decision::Double
                    }
                },
                '4' | '5' | '6' => {
                    if player_value >= 12 {
                        Decision::Stand
                    } else if player_value <= 8 {
                        Decision::Hit
                    } else {
                        Decision::Double
                    }
                },
                '7' | '8' | '9' => {
                    if player_value >= 17 {
                        Decision::Stand
                    } else if player_value == 11 || player_value == 10 {
                        Decision::Double
                    } else {
                        Decision::Hit
                    }
                },
                _ => {
                    if player_value >= 17 {
                        Decision::Stand
                    } else if player_value == 11 {
                        Decision::Double
                    } else {
                        Decision::Hit
                    }
                }
            }
        } else {
            Decision::None
        }
    }

    //TODO: Apply split values evaluation but with each part
    fn basic_strategy_soft_totals(&mut self) -> Decision {
        let face_up_card: Option<&Card> = self.dealer_hand.cards.first();
        let other_card: Option<&Card> = self.player_hand.cards.last();
        if let Some(dealer_card) = face_up_card {
            match dealer_card.rank {
                '2' => {
                    if let Some(card) = other_card {
                        if card.value > 7 {
                            Decision::Stand
                        } else if card.value == 7 {
                            Decision::Double
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                },
                '3' => {
                    if let Some(card) = other_card {
                        if card.value > 7 {
                            Decision::Stand
                        } else if card.value > 5 {
                            Decision::Double
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                },
                '4' => {
                    if let Some(card) = other_card {
                        if card.value > 7 {
                            Decision::Stand
                        } else if card.value > 3 {
                            Decision::Double
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                },
                '5' => {
                    if let Some(card) = other_card {
                        if card.value > 7 {
                            Decision::Stand
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                },
                '6' => {
                    if let Some(card) = other_card {
                        if card.value == 9 {
                            Decision::Stand
                        } else {
                            Decision::Double
                        }
                    } else {
                        Decision::None
                    }
                },
                '7' | '8' => {
                    if let Some(card) = other_card {
                        if card.value > 6 {
                            Decision::Stand
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                },
                _ => {
                    if let Some(card) = other_card {
                        if card.value > 7 {
                            Decision::Stand
                        } else {
                            Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                }
            }    
        } else {
            Decision::None
        }
    }

    fn basic_stategy_pairs(&mut self) -> Decision {
        let face_up_card: Option<&Card> = self.dealer_hand.cards.first();
        let other_card: Option<&Card> = self.player_hand.cards.last();
        if let Some(dealer_card) = face_up_card {
            match dealer_card.rank {
                '2' | '3' | '4' => {
                    if let Some(card) = other_card {
                        match card.value {
                            10 => Decision::Stand,
                            5 => Decision::Double,
                            4 => Decision::Hit,
                            _ => Decision::Split
                        }
                    } else {
                        Decision::None
                    }
                },
                '5' | '6' => {
                    if let Some(card) = other_card {
                        match card.value {
                            10 => Decision::Stand,
                            5 => Decision::Double,
                            _ => Decision::Split
                        }
                    } else {
                        Decision::None
                    }
                },
                '7' => {
                    if let Some(card) = other_card {
                        match card.value {
                            10 | 9 => Decision::Stand,
                            6 | 4 => Decision::Hit,
                            5 => Decision::Double,
                            _ => Decision::Split
                        }
                    } else {
                        Decision::None
                    }
                },
                '8' | '9' => {
                    if let Some(card) = other_card {
                        match card.value {
                            10 => Decision::Stand,
                            5 => Decision::Double,
                            7 | 6 | 4 | 3 | 2 => Decision::Hit,
                            _ => Decision::Split
                        }
                    } else {
                        Decision::None
                    }
                }
                _ => {
                    if let Some(card) = other_card {
                        match card.value {
                            10 | 9 => Decision::Stand,
                            1 | 8 => Decision::Split,
                            _ => Decision::Hit
                        }
                    } else {
                        Decision::None
                    }
                }
            }
        } else {
            Decision::None
        }
    } 

    fn decision_making(&mut self) {
        let mut final_decision: Decision;
        let player_value = self.player_hand.get_value();
        while player_value.main < 21 || player_value.split < 21 {
            if self.player_hand.cards.len() == 2 {
                let first_card = self.player_hand
                    .cards
                    .first()
                    .unwrap();
                let second_card = self.player_hand
                    .cards
                    .last()
                    .unwrap();
                if first_card.rank == second_card.rank {
                    final_decision = self.basic_stategy_pairs();
                } else if self.player_hand.ace_cards {
                    final_decision = self.basic_strategy_soft_totals();
                } else {
                    final_decision = self.basic_strategy_hard_totals();
                }
            } else {
                final_decision = self.basic_strategy_hard_totals();
            }
            match final_decision {
                Decision::Hit => {
                    self.player_hand.add_card(self.deck.hit(), false)
                },
                Decision::Double => {
                    self.money.double(false);
                    self.player_hand.add_card(self.deck.hit(), false)
                },
                Decision::Split => {
                    self.player_hand.make_split();
                    if !self.player_hand.split.is_empty() {
                        self.money.split_bet = rand::thread_rng().gen_range(1..=self.money.wallet);
                    } else {
                        continue;
                    }
                },
                _ => break
            }
        }
    }

    pub fn play(&mut self) {
        self.money.make_wallet();
        while self.money.wallet > 0 {
            self.init_game();
            let split: bool = self.player_turn();
            if self.player_hand.get_value().main <= 21 || self.player_hand.get_value().split <= 21 {
                self.dealer_turn();
            }
            let _ = self.determine_winner(true, split);
        }
    }

    pub fn basic_strategy_play(&mut self, limit: u32) -> Vec<GameStatus> {
        let mut status: Vec<GameStatus> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut games: u32 = 1;
        self.money.wallet = rng.gen_range(100..=u32::MAX);
        while self.money.wallet > 0 && games < limit {
            self.player_hand.clear_hand();
            self.dealer_hand.clear_hand();
            for _ in 0..2 {
                self.player_hand.add_card(self.deck.hit(), false);
                self.dealer_hand.add_card(self.deck.hit(), false);
            }
            self.money.bet = rng.gen_range(1..=self.money.wallet);
            self.decision_making();
            if self.player_hand.get_value().main <= 21 || self.player_hand.get_value().split <= 21 {
                while self.dealer_hand.get_value().main < 17 {
                    self.dealer_hand.add_card(self.deck.hit(), false);
                }       
            }
            status.push(self.winner(false, false));
            games += 1;
        }
        status
    }
}
