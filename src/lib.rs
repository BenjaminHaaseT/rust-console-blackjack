pub mod console;

use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

/// A trait that acts as an interface for any kind of blackjack table struct
pub trait BlackjackTable<P: Player> {
    fn new(starting_balance: f32, n_decks: usize, n_shuffles: u32) -> Self;
    fn place_bet(&self, player: &mut P, bet: f32) -> Result<(), BlackjackGameError>;
    fn play_option(
        &mut self,
        player: &mut P,
        options: &HashMap<i32, String>,
        option: i32,
    ) -> Result<(), BlackjackGameError>;
    fn stand(&self, player: &mut P);
    fn hit(&mut self, player: &mut P);
    fn double_down(&mut self, player: &mut P);
    fn split(&mut self, player: &mut P);
    fn deal_hand(&mut self, player: &mut P);
    fn get_dealers_optimal_final_hand(&mut self) -> u8;
    fn finish_hand(&mut self, player: &mut P);
}

// TODO: implement a struct that acts as a general interface for creating blackjack games and players;

pub trait Player {}

pub trait BlackjackGame {}

/// A general error that can capture lots of different situations when an error is needed
#[derive(Debug)]
pub struct BlackjackGameError {
    message: String,
}

impl BlackjackGameError {
    pub fn new(message: String) -> BlackjackGameError {
        BlackjackGameError { message }
    }
}

impl Display for BlackjackGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BlackjackGameError {}

/// General function for computing the optimal hand at the end of a hand of blackjack.
/// Takes `hand_value` a vector of u8, and returns its optimal value i.e. the greatest value less than
/// or equal to 21, if such a value exists in `hand_value`.
pub fn compute_optimal_hand(hand_value: &Vec<u8>) -> u8 {
    if hand_value.len() == 2 {
        if hand_value[0] > 21 || hand_value[1] > 21 {
            u8::min(hand_value[0], hand_value[1])
        } else {
            u8::max(hand_value[0], hand_value[1])
        }
    } else {
        hand_value[0]
    }
}

/// A struct for representing a single playing card. Comprised of two fields, `suit` and `rank`.
#[derive(PartialEq, Eq, Debug)]
pub struct Card {
    pub suit: &'static str,
    pub rank: &'static str,
    pub val: u8,
}

impl Card {
    /// Creates a new card struct given `suit` and `rank`
    pub fn new(suit: &'static str, rank: &'static str) -> Card {
        let val = Card::get_card_value(rank);
        Card { suit, rank, val }
    }

    /// Associated method that returns a string represnting what a card facedown looks like on the console
    pub fn display_facedown() -> String {
        String::from("|*|")
    }

    /// Returns the numeric value of the playing card. Note aces default to the value of 1, and consequently,
    /// any logic that treats aces as multiple values i.e. 1 or 11 needs to be handled outside of this struct
    pub fn get_card_value(rank: &str) -> u8 {
        match u8::from_str(rank) {
            Err(_) if rank == "A" => 1,
            Err(_) => 10,
            Ok(r) => r,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

/// A simple struct that acts as a collection of playing cards of type Card.
pub struct Deck {
    cards: Vec<Rc<Card>>,
    n_decks: usize,
    deck_pos: usize,
    shuffle_flag_pos: usize,
    pub shuffle_flag: bool,
}

/// A struct to represent a deck of cards, is basically a collection of card structs that implements some specific logic related to a game of blackjack
impl Deck {
    /// An associated function that aids in the building of a deck of cards
    fn build_card_deck(n_decks: usize) -> Vec<Rc<Card>> {
        let mut cards = Vec::with_capacity(n_decks * 52);
        for _i in 0..n_decks {
            for suit in SUITS {
                for rank in RANKS {
                    cards.push(Rc::new(Card::new(suit, rank)));
                }
            }
        }
        cards
    }

    /// Creates and returns a new Deck struct
    pub fn new(n_decks: usize) -> Deck {
        assert!(n_decks > 0, "Cannot have a deck with zero cards");
        let cards = Self::build_card_deck(n_decks);
        let n_cards = cards.len();
        let shuffle_flag_pos = f32::floor(((n_cards - 1) as f32) * 0.8) as usize;

        Deck {
            cards,
            n_decks,
            deck_pos: 0,
            shuffle_flag_pos,
            shuffle_flag: true,
        }
    }

    /// Shuffles the deck of cards to simulate the random behavior of a shuffled deck of cards
    pub fn shuffle(&mut self, n_shuffles: u32) {
        assert!(n_shuffles > 0);
        let mut rng = rand::thread_rng();
        for _ in 0..n_shuffles {
            for i in 0..(self.cards.len() / 2) {
                let random_idx = rng.gen_range(0..self.cards.len());
                self.cards.swap(i, random_idx);
            }
        }
        self.deck_pos = 0;
        self.shuffle_flag = false;
    }

    /// Returns the next card, i.e. the card that is at the top of the deck of cards
    pub fn get_next_card(&mut self) -> Option<Rc<Card>> {
        if self.deck_pos < self.cards.len() {
            let next_card = Some(Rc::clone(&self.cards[self.deck_pos]));
            self.deck_pos += 1;
            if self.deck_pos == self.shuffle_flag_pos {
                self.shuffle_flag = true;
            }
            return next_card;
        }

        None
    }
}

// struct PlayersBlackjackHandCLI {
//     hand: Vec<Vec<Rc<Card>>>,
//     hand_values: Vec<Vec<u8>>,
//     bets: Vec<u32>,
//     hand_str: Vec<String>,
//     hand_values_str: Vec<String>,
//     bets_str: Vec<String>,
// }

// impl PlayersBlackjackHandCLI {
//     /// Creates a new PlayersBlackjackHand struct
//     fn new() -> Self {
//         let hand = vec![vec![]];
//         let hand_values = vec![vec![]];
//         let bets = vec![];
//         let hand_str = vec![String::new()];
//         let hand_values_str = vec![String::new()];
//         let bets_str = vec![];

//         PlayersBlackjackHandCLI {
//             hand,
//             hand_values,
//             bets,
//             hand_str,
//             hand_values_str,
//             bets_str,
//         }
//     }

//     /// Takes in `bet` and updates the state of the bet associated with a particular hand.
//     fn place_bet(&mut self, bet: u32) {
//         self.bets.push(bet);
//         self.bets_str.push(bet.to_string());
//     }

//     /// Sets the bet of the current hand to 0 and returns the value of the current bet.
//     fn lose_bet(&mut self, hand_idx: usize) -> u32 {
//         let res = self.bets[hand_idx];
//         self.bets[hand_idx] = 0;
//         res
//     }

//     /// Simple function to check whether or not the current hand i.e. the hand at index `hand_idx` can split.
//     /// The function will panice if `hand_idx` is not a valid index or the hand vector is empty.
//     fn can_split(&self, hand_idx: usize) -> bool {
//         self.hand[hand_idx].len() == 2 && self.hand[hand_idx][0].rank == self.hand[hand_idx][1].rank
//     }

//     /// Simple fucntion to check whether the current hand i.e. the hand at index `hand_idx` can double down.
//     /// The function will panic if `hand_idx` is not a valid index, or hand_values vector is empty.
//     fn can_double_down(&self, hand_idx: usize) -> bool {
//         if self.hand_values[hand_idx].len() == 2 {
//             hand_idx == 0
//                 && self.hand[hand_idx].len() == 2
//                 && ((self.hand_values[hand_idx][0] == 9
//                     || self.hand_values[hand_idx][0] == 10
//                     || self.hand_values[hand_idx][0] == 11)
//                     || (self.hand_values[hand_idx][1] == 9
//                         || self.hand_values[hand_idx][1] == 10
//                         || self.hand_values[hand_idx][1] == 11))
//         } else {
//             hand_idx == 0
//                 && self.hand[hand_idx].len() == 2
//                 && (self.hand_values[hand_idx][0] == 9
//                     || self.hand_values[hand_idx][0] == 10
//                     || self.hand_values[hand_idx][0] == 11)
//         }
//     }

//     /// Receive a new card, add it to the players hand and update the string representing
//     /// the current hand as well
//     fn receive_card(&mut self, card: Rc<Card>, hand_idx: usize) {
//         self.hand[hand_idx].push(Rc::clone(&card));
//         // Now update string representing hand
//         if self.hand_str[hand_idx].is_empty() {
//             self.hand_str[hand_idx].push_str(card.to_string().as_str());
//         } else {
//             self.hand_str[hand_idx].push_str(format!(" {}", card.to_string()).as_str());
//         }
//     }

//     /// Implement the logic for doubling down on a bet, updates the bets and the bets_str for display purposes.
//     /// Returns the value of the current bet for updating the players balance.
//     fn double_down(&mut self, hand_idx: usize) -> u32 {
//         let cur_bet = self.bets[hand_idx];
//         self.bets[hand_idx] += cur_bet;
//         self.bets_str[hand_idx] = self.bets[hand_idx].to_string();
//         cur_bet
//     }

//     /// Implements the logic for splitting a valid hand. Returns the value of the current bet to update the players balance.
//     fn split(&mut self, hand_idx: usize) -> u32 {
//         // Get current bet and add another to the vector that keeps track of the number of bets, also push another bet to the vector of strings
//         // representing the current total number of bets the player has at the table
//         let cur_bet = self.bets[hand_idx];
//         self.bets.insert(hand_idx + 1, cur_bet);
//         self.bets_str.insert(hand_idx + 1, cur_bet.to_string());

//         // Get the card that will be the geneis of the new hand, reset the hand value for the current hand and
//         // push a new vector onto hand_values representing the new empty hand value
//         let new_hand = self.hand[hand_idx].pop().unwrap();
//         self.hand.insert(hand_idx + 1, vec![new_hand]);

//         self.hand_values[hand_idx].clear();
//         self.hand_values.insert(hand_idx + 1, vec![]);

//         self.hand_values_str[hand_idx].clear();
//         self.hand_values_str.insert(hand_idx + 1, String::new());

//         // Now update the hand_str and hand_values_str
//         self.hand_str[hand_idx] = self.hand[hand_idx][0].to_string();
//         self.hand_str
//             .insert(hand_idx + 1, self.hand[hand_idx + 1][0].to_string());

//         cur_bet
//     }

//     /// Compute the value of the players current hand and update the formatted string
//     /// representing the value of the current hand as well
//     fn compute_hand_value(&mut self, hand_idx: usize) {
//         if self.hand[hand_idx].len() == 2 {
//             self.hand_values[hand_idx].push(self.hand[hand_idx].iter().map(|c| c.val).sum::<u8>());

//             // Need to check if we have more than one possible value for the given hand
//             if self.hand[hand_idx][0].rank == "A" || self.hand[hand_idx][1].rank == "A" {
//                 let alternative_hand_val = self.hand_values[hand_idx]
//                     .last()
//                     .expect("hand should not be empty")
//                     + 10;

//                 self.hand_values[hand_idx].push(alternative_hand_val);
//             }
//         } else {
//             let new_card_val = self.hand[hand_idx]
//                 .last()
//                 .expect("hand should not be empty")
//                 .val;
//             self.hand_values[hand_idx][0] += new_card_val;
//             if self.hand_values[hand_idx].len() == 2 {
//                 self.hand_values[hand_idx][1] += new_card_val;
//             } else if self.hand_values[hand_idx][0] <= 11 && new_card_val == 1 {
//                 // Check if we need another hand value to represent all possible values of the hand
//                 let alternative_hand = self.hand_values[hand_idx][0] + 10;
//                 self.hand_values[hand_idx].push(alternative_hand);
//             }
//         }

//         // Now update the string that represents the value
//         self.hand_values_str[hand_idx] = if self.hand_values[hand_idx].len() == 2 {
//             if self.hand_values[hand_idx][0] > 21 || self.hand_values[hand_idx][1] > 21 {
//                 format!(
//                     "{}",
//                     u8::min(self.hand_values[hand_idx][0], self.hand_values[hand_idx][1])
//                 )
//             } else {
//                 format!(
//                     "{}/{}",
//                     self.hand_values[hand_idx][0], self.hand_values[hand_idx][1]
//                 )
//             }
//         } else {
//             self.hand_values[hand_idx][0].to_string()
//         };
//     }

//     /// Checks if the players current hand is a blackjack, ensures it must be a natural blackjack by
//     /// checking that `self.hand_idx` is equal to zero i.e. it is the first hand dealt to the player, not
//     /// a hand dealt after splitting
//     fn is_blackjack(&self, hand_idx: usize) -> bool {
//         hand_idx == 0
//             && self.hand[0].len() == 2
//             && ((self.hand[0][0].rank == "A" && self.hand[0][1].val == 10)
//                 || (self.hand[0][0].val == 10 && self.hand[0][1].rank == "A"))
//     }

//     /// Checks whether the current hand has busted or not
//     fn busted(&self, hand_idx: usize) -> bool {
//         if self.hand_values[hand_idx].len() == 2 {
//             self.hand_values[hand_idx][0] > 21 && self.hand_values[hand_idx][1] > 21
//         } else {
//             self.hand_values[hand_idx][0] > 21
//         }
//     }

//     /// Resets the hand to a new empty hand
//     fn reset(&mut self) {
//         self.hand = vec![vec![]];
//         self.hand_values = vec![vec![]];
//         self.bets.clear();
//         self.hand_str = vec![String::from("")];
//         self.hand_values_str = vec![String::from("")];
//         self.bets_str.clear();
//     }
// }

// pub struct PlayerCLI {
//     name: String,
//     balance: f32,
//     bj_hand: PlayersBlackjackHandCLI,
//     hand_idx: usize,
// }

// impl PlayerCLI {
//     /// Creates a new player struct
//     pub fn new(name: String, balance: f32) -> PlayerCLI {
//         PlayerCLI {
//             name,
//             balance,
//             bj_hand: PlayersBlackjackHandCLI::new(),
//             hand_idx: 0usize,
//         }
//     }

//     /// Gets the optimal hand of a player for valid bet the player has. If the player has no more valid bets, then
//     /// the method returns None
//     pub fn get_optimal_hands(&self) -> Option<HashMap<usize, u8>> {
//         let mut res = HashMap::new();
//         for i in 0..self.bj_hand.bets.len() {
//             if self.bj_hand.bets[i] > 0 {
//                 res.insert(i, compute_optimal_hand(&self.bj_hand.hand_values[i]));
//             }
//         }
//         match res.is_empty() {
//             true => None,
//             false => Some(res),
//         }
//     }

//     /// Only icreases players hand index by 1 in order to signal that this hand is finished.
//     pub fn stand(&mut self) {
//         self.hand_idx += 1;
//     }

//     /// Provides a boolean flag signaling whether or not the player is finished with their hand
//     pub fn turn_is_over(&self) -> bool {
//         self.hand_idx == self.bj_hand.bets.len()
//     }

//     /// Takes `bet` representing a bet at a blackjack table, and updates the balance then passes the value along to
//     /// the players PlayersBlackjackHand struct to execute the necessary logic for that struct as well
//     pub fn place_bet(&mut self, bet: f32) -> Result<(), BlackjackGameError> {
//         if bet > self.balance {
//             return Err(BlackjackGameError {
//                 message: "Insufficient funds to place that bet".to_string(),
//             });
//         }
//         self.balance -= bet;
//         self.bj_hand.place_bet(bet as u32);
//         Ok(())
//     }

//     /// Returns the value of the current bet and resets its value to 0 for post processing
//     /// Increases the players hand_idx by 1, to signal this hand is finished.
//     pub fn lose_bet(&mut self) -> u32 {
//         let bet = self.bj_hand.lose_bet(self.hand_idx);
//         self.hand_idx += 1;
//         bet
//     }

//     /// Queries the players hand struct to see what the valid options are for the player to takes
//     /// function will panic if the players current hand has busted or the player has not placed any bets
//     pub fn get_playing_options(&self) -> HashMap<i32, String> {
//         assert!(!self.bj_hand.busted(self.hand_idx), "hand should be over");
//         assert!(
//             !self.bj_hand.bets.is_empty(),
//             "player should have placed a bet"
//         );
//         let mut playing_options = HashMap::new();
//         playing_options.insert(1, "stand".to_string());
//         playing_options.insert(2, "hit".to_string());
//         let mut playing_option = 3;

//         if self.bj_hand.can_split(self.hand_idx)
//             && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
//             && self.bj_hand.hand.len() < 4
//         {
//             playing_options.insert(playing_option, "split".to_string());
//             playing_option += 1;
//         }

//         if self.bj_hand.can_double_down(self.hand_idx)
//             && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
//             && self.bj_hand.hand.len() == 1
//         {
//             playing_options.insert(playing_option, "double down".to_string());
//         }

//         playing_options
//     }

//     /// Wrapper method for self.bj_hand.receive_card()
//     pub fn receive_card(&mut self, card: Rc<Card>) {
//         self.bj_hand.receive_card(card, self.hand_idx);
//     }

//     /// Method that allows the player to double down on a bet
//     pub fn double_down(&mut self) {
//         let cur_bet = self.bj_hand.double_down(self.hand_idx);
//         self.balance -= cur_bet as f32;
//     }

//     /// Method that allwos the player to split their current hand, assumes all the conditions necessary for a valid split have been met
//     pub fn split(&mut self, card1: Rc<Card>, card2: Rc<Card>) {
//         let cur_bet = self.bj_hand.split(self.hand_idx);
//         self.balance -= cur_bet as f32;
//         // Deal a the new cards to each new hand respectively, and compute their hand values
//         self.bj_hand.receive_card(card1, self.hand_idx);
//         self.bj_hand.compute_hand_value(self.hand_idx);
//         self.bj_hand.receive_card(card2, self.hand_idx + 1);
//         self.bj_hand.compute_hand_value(self.hand_idx + 1);
//     }

//     /// Returns whether or not the player has a blackjack or not, again is a wrapper method for self.bj_hand.is_blackjack()
//     pub fn has_blackjack(&self) -> bool {
//         self.bj_hand.is_blackjack(self.hand_idx)
//     }

//     /// Returns a boolean whether or not the player has busted or not, is a wrapper method for self.bj_hand.busted()
//     pub fn busted(&self) -> bool {
//         self.bj_hand.busted(self.hand_idx)
//     }

//     /// Computes the hand value of the current hand, and updates the state of self.bj_hand. Acts as a wrapper for self.bj_hand.compute_value()
//     pub fn compute_hand_value(&mut self) {
//         self.bj_hand.compute_hand_value(self.hand_idx);
//     }

//     /// A simple getter method that returns the players vector of bets
//     pub fn bets(&self) -> &Vec<u32> {
//         &self.bj_hand.bets
//     }

//     /// Resets all of the necessary fields so the player can play another hand of blackjack
//     pub fn reset(&mut self) {
//         self.hand_idx = 0;
//         self.bj_hand.reset();
//     }
// }

// struct DealersBlackjackHand {
//     hand: Vec<Rc<Card>>,
//     hand_value: Vec<u8>,
//     hand_str: String,
//     hand_value_str: String,
// }

// impl DealersBlackjackHand {
//     /// Creates a new `DealersBlackjackHand` struct
//     fn new() -> Self {
//         let hand = vec![];
//         let hand_value = vec![];
//         let hand_str = String::new();
//         let hand_value_str = String::new();

//         DealersBlackjackHand {
//             hand,
//             hand_value,
//             hand_str,
//             hand_value_str,
//         }
//     }

//     /// Checks whether the dealers hand has busted or not
//     fn busted(&self) -> bool {
//         if self.hand_value.len() == 2 {
//             self.hand_value[0] > 21 && self.hand_value[1] > 21
//         } else {
//             self.hand_value[0] > 21
//         }
//     }

//     /// Checks whether or not the dealers hand has busted
//     fn is_blackjack(&self) -> bool {
//         self.hand.len() == 2
//             && ((self.hand[0].rank == "A" && self.hand[1].val == 10)
//                 || (self.hand[0].val == 10 && self.hand[1].rank == "A"))
//     }

//     /// Computes the dealers hand value, and updates the string that represents that value for display
//     /// via standard output ie console
//     fn compute_hand_value(&mut self) {
//         if self.hand.len() == 2 {
//             self.hand_value.push(self.hand.iter().map(|c| c.val).sum());

//             // We need to check if there is an alternative hand value possible
//             if self.hand[0].rank == "A" || self.hand[1].rank == "A" {
//                 let alternative_hand_val = self.hand_value[0] + 10;
//                 self.hand_value.push(alternative_hand_val);
//             }
//         } else {
//             let new_card_val = self.hand.last().expect("hand should not be empty").val;
//             self.hand_value[0] += new_card_val;
//             if self.hand_value.len() == 2 {
//                 self.hand_value[1] += new_card_val;
//             } else if self.hand_value[0] <= 11 && new_card_val == 1 {
//                 let alternative_hand = self.hand_value[0] + 10;
//                 self.hand_value.push(alternative_hand);
//             }
//         }

//         // Update the string that will be displayed
//         self.hand_value_str = if self.hand_value.len() == 2 {
//             if self.hand_value[0] > 21 || self.hand_value[1] > 21 {
//                 format!("{}", u8::min(self.hand_value[0], self.hand_value[1]))
//             } else if self.hand_value[0] == 21 || self.hand_value[1] == 21 {
//                 format!("21")
//             } else {
//                 format!("{}/{}", self.hand_value[0], self.hand_value[1])
//             }
//         } else {
//             self.hand_value[0].to_string()
//         }
//     }

//     /// Receive a new card, `card` which will be pushed to dealers hand
//     /// function will also update the value of `self.hand_str`, the string representing the cards to be printed to the console
//     fn receive_card(&mut self, card: Rc<Card>) {
//         self.hand.push(Rc::clone(&card));
//         if self.hand_str.is_empty() {
//             self.hand_str.push_str(card.to_string().as_str());
//         } else {
//             self.hand_str
//                 .push_str(format!(" {}", card.to_string()).as_str());
//         }
//     }

//     /// Method for computing the optimal, valid final hand according to the rules of blackjack
//     fn compute_optimal_final_hand(&mut self, deck: &mut Deck) -> u8 {
//         if self.hand_value.len() == 1 {
//             while self.hand_value[0] < 17 && self.hand_value.len() < 2 {
//                 self.receive_card(deck.get_next_card().unwrap());
//                 self.compute_hand_value();
//             }
//         }

//         // Check to see if we have a hand multiple hand values
//         while self.hand_value.len() == 2 && self.hand_value[0] < 17 && self.hand_value[1] < 17 {
//             self.receive_card(deck.get_next_card().unwrap());
//             self.compute_hand_value();
//         }

//         // Ensure the dealer has a valid hand no less than 17
//         while self.hand_value.len() == 2
//             && ((self.hand_value[0] < 17 && self.hand_value[1] > 21)
//                 || (self.hand_value[0] > 21 && self.hand_value[1] < 17))
//         {
//             self.receive_card(deck.get_next_card().unwrap());
//             self.compute_hand_value();
//         }

//         // Now we are sure the dealer has drawn enough cards to either bust or have a valid hand according to rules of blackjack
//         compute_optimal_hand(&self.hand_value)
//     }

//     /// Resets the dealers hand to play another round
//     fn reset(&mut self) {
//         self.hand.clear();
//         self.hand_value.clear();
//         self.hand_str.clear();
//         self.hand_value_str.clear();
//     }
// }

pub use crate::console::{
    player::ConsolePlayer, table::ConsoleBlackjackTable, ConsoleBlackjackGame,
};

pub fn run() -> std::io::Result<()> {
    let player = ConsolePlayer::new(String::from("Rick Sanchez"), 500.0);
    let table = ConsoleBlackjackTable::new(500000000.0, 6, 7);
    let mut game = ConsoleBlackjackGame::new(5, player, table);
    game.play()?;

    Ok(())
}
