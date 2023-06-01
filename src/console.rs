/// This module contains all the necessary structs for a blackjack game played on the console.
use crate::{
    compute_optimal_hand, BlackjackGameError, Card, DealersBlackjackHand, Deck, Player,
    PlayersBlackjackHand,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

pub trait DisplayablePlayersBlackjackHand {
    fn display_hand(&self);
}

impl DisplayablePlayersBlackjackHand for PlayersBlackjackHand {
    /// Displays the players current blackjack hand in the console printed in a nice looking format
    fn display_hand(&self) {
        let mut formatted_hand_values_str = vec![];
        let mut formatted_bet_str = vec![];
        for (i, h) in self.hand_str.iter().enumerate() {
            let width = h.len();
            let bets_width = width - 1;
            formatted_hand_values_str.push(format!("{:<width$}", self.hand_values_str[i]));
            formatted_bet_str.push(format!("${:<bets_width$}", self.bets_str[i]));
        }

        let (formatted_hand_str, formatted_hand_values_str, formatted_bet_str) = (
            self.hand_str.join(" | "),
            formatted_hand_values_str.join(" | "),
            formatted_bet_str.join(" | "),
        );

        let bet_tag = if self.bets.len() > 1 {
            "Bets:".to_string()
        } else {
            "Bet:".to_string()
        };

        println!("{:<10}{}", "You:", formatted_hand_str);
        println!("{:<10}{}", "Value:", formatted_hand_values_str);
        println!("{:<10}{}", bet_tag, formatted_bet_str);
    }
}

pub trait DisplayablePlayer {
    fn display_hand(&self);
    fn display_balance(&self);
}

impl DisplayablePlayer for Player {
    /// A wrapper method for self.bj_hand.display_hand(),  displays the players hand in a nice way
    fn display_hand(&self) {
        self.bj_hand.display_hand();
    }

    /// Displays the players balance to the console
    fn display_balance(&self) {
        println!("{:<10}${}", "Balance:", self.balance)
    }
}

// trait DisplayableDealersBlackjackHand {}

// /// This struct reprsents a players blackjack hand and implements all the internal logic details needed to
// /// keep track of how the hand will be displayed via the console, the value(s) associated with the hand etc...
// /// This struct is meant as a helper to abstract logic out of other structs and reduce the complexity of code.
// pub struct PlayersBlackjackHandCLI {
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

// / Displays the players current blackjack hand in the console printed in a nice looking format
// fn display_hand(&self) {
//     let mut formatted_hand_values_str = vec![];
//     let mut formatted_bet_str = vec![];
//     for (i, h) in self.hand_str.iter().enumerate() {
//         let width = h.len();
//         let bets_width = width - 1;
//         formatted_hand_values_str.push(format!("{:<width$}", self.hand_values_str[i]));
//         formatted_bet_str.push(format!("${:<bets_width$}", self.bets_str[i]));
//     }

//     let (formatted_hand_str, formatted_hand_values_str, formatted_bet_str) = (
//         self.hand_str.join(" | "),
//         formatted_hand_values_str.join(" | "),
//         formatted_bet_str.join(" | "),
//     );

//     let bet_tag = if self.bets.len() > 1 {
//         "Bets:".to_string()
//     } else {
//         "Bet:".to_string()
//     };

//     println!("{:<10}{}", "You:", formatted_hand_str);
//     println!("{:<10}{}", "Value:", formatted_hand_values_str);
//     println!("{:<10}{}", bet_tag, formatted_bet_str);
// }

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
//             self.hand_values[hand_idx].push(
//                 self.hand[hand_idx]
//                     .iter()
//                     .map(|c| c.get_card_value())
//                     .sum::<u8>(),
//             );

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
//                 .get_card_value();
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
//             && ((self.hand[0][0].rank == "A" && self.hand[0][1].get_card_value() == 10)
//                 || (self.hand[0][0].get_card_value() == 10 && self.hand[0][1].rank == "A"))
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

// /// A player at a blackjack table, acts as somewhat of a wrapper for the PlayersBlackjackHand struct,
// /// i.e. an interface for interacting with the PlayersBlackjackHand struct. Its main purpose is to keep track of what hand the player is
// /// currently betting on given that the player might have multiple bets on the table if the player decides to split a valid hand
// // TODO: Implement split/doubledown methods
// pub struct PlayerCLI {
//     name: String,
//     balance: f32,
//     bj_hand: PlayersBlackjackHandCLI,
//     hand_idx: usize,
// }

// // TODO: Implement a method to deal with updating the players balance
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

//     /// Displays the players balance to the console
//     pub fn display_balance(&self) {
//         println!("{:<10}${}", "Balance:", self.balance)
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

//     /// A wrapper method for self.bj_hand.display_hand(),  displays the players hand in a nice way
//     fn display_hand(&self) {
//         self.bj_hand.display_hand();
//     }

//     /// Resets all of the necessary fields so the player can play another hand of blackjack
//     pub fn reset(&mut self) {
//         self.hand_idx = 0;
//         self.bj_hand.reset();
//     }
// }

// impl Player for PlayerCLI {
//     /// Creates a new player struct
//     fn new(name: String, balance: f32) -> PlayerCLI {
//         PlayerCLI {
//             name,
//             balance,
//             bj_hand: PlayersBlackjackHandCLI::new(),
//             hand_idx: 0usize,
//         }
//     }

//     /// Gets the optimal hand of a player for valid bet the player has. If the player has no more valid bets, then
//     /// the method returns None
//     fn get_optimal_hands(&self) -> Option<HashMap<usize, u8>> {
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
//     fn stand(&mut self) {
//         self.hand_idx += 1;
//     }

//     /// Provides a boolean flag signaling whether or not the player is finished with their hand
//     fn turn_is_over(&self) -> bool {
//         self.hand_idx == self.bj_hand.bets.len()
//     }

//     /// Takes `bet` representing a bet at a blackjack table, and updates the balance then passes the value along to
//     /// the players PlayersBlackjackHand struct to execute the necessary logic for that struct as well
//     fn place_bet(&mut self, bet: f32) -> Result<(), BlackjackGameError> {
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
//     fn lose_bet(&mut self) -> u32 {
//         let bet = self.bj_hand.lose_bet(self.hand_idx);
//         self.hand_idx += 1;
//         bet
//     }

//     /// Queries the players hand struct to see what the valid options are for the player to takes
//     /// function will panic if the players current hand has busted or the player has not placed any bets
//     fn get_playing_options(&self) -> HashMap<i32, String> {
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
//     fn receive_card(&mut self, card: Rc<Card>) {
//         self.bj_hand.receive_card(card, self.hand_idx);
//     }

//     /// Method that allows the player to double down on a bet
//     fn double_down(&mut self) {
//         let cur_bet = self.bj_hand.double_down(self.hand_idx);
//         self.balance -= cur_bet as f32;
//     }

//     /// Method that allwos the player to split their current hand, assumes all the conditions necessary for a valid split have been met
//     fn split(&mut self, card1: Rc<Card>, card2: Rc<Card>) {
//         let cur_bet = self.bj_hand.split(self.hand_idx);
//         self.balance -= cur_bet as f32;
//         // Deal a the new cards to each new hand respectively, and compute their hand values
//         self.bj_hand.receive_card(card1, self.hand_idx);
//         self.bj_hand.compute_hand_value(self.hand_idx);
//         self.bj_hand.receive_card(card2, self.hand_idx + 1);
//         self.bj_hand.compute_hand_value(self.hand_idx + 1);
//     }

//     /// Returns whether or not the player has a blackjack or not, again is a wrapper method for self.bj_hand.is_blackjack()
//     fn has_blackjack(&self) -> bool {
//         self.bj_hand.is_blackjack(self.hand_idx)
//     }

//     /// Returns a boolean whether or not the player has busted or not, is a wrapper method for self.bj_hand.busted()
//     fn busted(&self) -> bool {
//         self.bj_hand.busted(self.hand_idx)
//     }

//     /// Computes the hand value of the current hand, and updates the state of self.bj_hand. Acts as a wrapper for self.bj_hand.compute_value()
//     fn compute_hand_value(&mut self) {
//         self.bj_hand.compute_hand_value(self.hand_idx);
//     }

//     /// Resets all of the necessary fields so the player can play another hand of blackjack
//     fn reset(&mut self) {
//         self.hand_idx = 0;
//         self.bj_hand.reset();
//     }
// }

pub trait DisplayableDealersBlackjackHand {
    fn display_hand_value(&self);
    fn display_hand_without_hole(&self);
    fn display_hand(&self);
}

impl DisplayableDealersBlackjackHand for DealersBlackjackHand {
    /// Prints a string representing the value of the dealers hand to the console
    fn display_hand_value(&self) {
        println!("{:<10}{}", "Value", self.hand_value_str);
    }

    /// Display dealers hand without revealing the hole card i.e at the begginning of a hand
    fn display_hand_without_hole(&self) {
        println!(
            "{:<10}{} {}",
            "Dealer:",
            Card::display_facedown(),
            self.hand[1]
        );
    }

    /// Print the value of dealers hand to the console, formatted in a nice way
    fn display_hand(&self) {
        println!("{:<10}{}", "Dealer:", self.hand_str);
    }
}

// /// A struct to represent the internal logic of the dealers blackjack hand. Meant as a helper struct, not to be instantiated directly.
// pub struct DealersBlackjackHandCLI {
//     hand: Vec<Rc<Card>>,
//     hand_value: Vec<u8>,
//     hand_str: String,
//     hand_value_str: String,
// }

// impl DealersBlackjackHandCLI {
//     /// Creates a new `DealersBlackjackHand` struct
//     fn new() -> Self {
//         let hand = vec![];
//         let hand_value = vec![];
//         let hand_str = String::new();
//         let hand_value_str = String::new();

//         DealersBlackjackHandCLI {
//             hand,
//             hand_value,
//             hand_str,
//             hand_value_str,
//         }
//     }

//     /// Prints a string representing the value of the dealers hand to the console
//     fn display_hand_value(&self) {
//         println!("{:<10}{}", "Value", self.hand_value_str);
//     }

//     /// Display dealers hand without revealing the hole card i.e at the begginning of a hand
//     fn display_hand_without_hole(&self) {
//         println!(
//             "{:<10}{} {}",
//             "Dealer:",
//             Card::display_facedown(),
//             self.hand[1]
//         );
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
//             && ((self.hand[0].rank == "A" && self.hand[1].get_card_value() == 10)
//                 || (self.hand[0].get_card_value() == 10 && self.hand[1].rank == "A"))
//     }

//     /// Computes the dealers hand value, and updates the string that represents that value for display
//     /// via standard output ie console
//     fn compute_hand_value(&mut self) {
//         if self.hand.len() == 2 {
//             self.hand_value
//                 .push(self.hand.iter().map(|c| c.get_card_value()).sum());

//             // We need to check if there is an alternative hand value possible
//             if self.hand[0].rank == "A" || self.hand[1].rank == "A" {
//                 let alternative_hand_val = self.hand_value[0] + 10;
//                 self.hand_value.push(alternative_hand_val);
//             }
//         } else {
//             let new_card_val = self
//                 .hand
//                 .last()
//                 .expect("hand should not be empty")
//                 .get_card_value();
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

//     /// Print the value of dealers hand to the console, formatted in a nice way
//     fn display_hand(&self) {
//         println!("{:<10}{}", "Dealer:", self.hand_str);
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

// //     /// Resets the dealers hand to play another round
// //     fn reset(&mut self) {
// //         self.hand.clear();
// //         self.hand_value.clear();
// //         self.hand_str.clear();
// //         self.hand_value_str.clear();
// //     }
// // }

// trait DisplayableBlackjackTable {
//     fn display_playing_options(&self, options: &HashMap<i32, String>, player: &Player);
//     fn display_end_of_hand_state(
//         &self,
//         player: &Player,
//         result_messages: Vec<String>,
//         winnings: f32,
//     );
// }

// impl DisplayableBlackjackTable for BlackjackTable {
//     /// Takes a HashMap<i32, String> of numbered options and prints the options formatted nicely.
//     fn display_playing_options(&self, options: &HashMap<i32, String>, player: &Player) {
//         let display_tag = if player.bj_hand.hand.len() >= 2 {
//             format!("Your options (hand #{}):", player.hand_idx + 1)
//         } else {
//             String::from("You options: ")
//         };
//         println!();
//         println!("{}", display_tag);
//         for i in 1..=(options.len() as i32) {
//             println!("\t{}: {}", i, &options[&i]);
//         }
//     }

//     /// A method that will display the state of the game on the console at the end of a hand
//     fn display_end_of_hand_state(
//         &self,
//         player: &Player,
//         result_messages: Vec<String>,
//         winnings: f32,
//     ) {
//         println!("{}", "-".to_string().repeat(80));
//         self.dealers_hand.display_hand();
//         self.dealers_hand.display_hand_value();
//         println!("\n\n");
//         player.display_hand();
//         player.display_balance();

//         // For readability
//         println!("\n");

//         for msg in result_messages {
//             println!("{msg}");
//         }
//         if self.dealers_hand.busted() {
//             println!("Dealer busted");
//         }
//         println!("Winnings: ${winnings:2.2}");
//     }
// }

/// / A struct to implement the logic for a game of blackjack played over the console. Contains all the appropraite methods that imlement the
/// / typical valid rules of a blackjack game. Intended to interact with a player struct.
pub struct BlackjackTableCLI {
    deck: Deck,
    balance: f32,
    dealers_hand: DealersBlackjackHand,
    n_shuffles: u32,
}

impl BlackjackTableCLI {
    /// Creates a new instance of a BlackjackTableCLI struct
    pub fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> Self {
        let deck = Deck::new(n_decks);

        BlackjackTableCLI {
            deck,
            balance: starting_balance,
            dealers_hand: DealersBlackjackHand::new(),
            n_shuffles,
        }
    }

    /// Takes a Player struct, `player` and places a bet
    fn place_bet(&self, player: &mut Player, bet: f32) -> Result<(), BlackjackGameError> {
        if bet <= 0.0 {
            return Err(BlackjackGameError {
                message: "Bet must be a positive amount".to_string(),
            });
            // return Err("Bet must be a positive amount".to_string());
        } else if self.balance < 1.5 * bet {
            return Err(BlackjackGameError {
                message: "Insufficient table balance to payout bet".to_string(),
            });
        }
        player.place_bet(bet)
    }

    /// Takes a HashMap<i32, String> of numbered options and prints the options formatted nicely.
    fn display_playing_options(&self, options: &HashMap<i32, String>, player: &Player) {
        let display_tag = if player.bj_hand.hand.len() >= 2 {
            format!("Your options (hand #{}):", player.hand_idx + 1)
        } else {
            String::from("You options: ")
        };
        println!();
        println!("{}", display_tag);
        for i in 1..=(options.len() as i32) {
            println!("\t{}: {}", i, &options[&i]);
        }
    }

    /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
    /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
    fn play_option(
        &mut self,
        player: &mut Player,
        options: &HashMap<i32, String>,
        option: i32,
    ) -> Result<(), BlackjackGameError> {
        match options[&option].as_str() {
            "stand" => Ok(self.stand(player)),
            "hit" => Ok(self.hit(player)),
            "split" => Ok(self.split(player)),
            "double down" => Ok(self.double_down(player)),
            _ => Err(BlackjackGameError {
                message: format!("{} is not a valid option", option),
            }),
        }
    }

    /// Takes a Player struct `player` and changes its state via its stand method
    fn stand(&self, player: &mut Player) {
        player.stand();
        if !player.turn_is_over() {
            println!("{}", "-".to_string().repeat(80));
            self.dealers_hand.display_hand_without_hole();
            println!("\n\n");
            player.bj_hand.display_hand();
            player.display_balance();
        }
    }

    /// Takes a Player `player` and changes the state `players`'s hand by dealing another card.
    /// The function then computes if the player has busted or not and adjusts the bets of the player accordingly
    fn hit(&mut self, player: &mut Player) {
        player.receive_card(self.deck.get_next_card().unwrap());
        player.compute_hand_value();

        println!("{}", "-".to_string().repeat(80));
        self.dealers_hand.display_hand_without_hole();
        println!("\n\n");
        player.bj_hand.display_hand();
        player.display_balance();

        if player.busted() {
            println!("Busted, you lost the bet");
            self.balance += player.lose_bet() as f32;
        }
    }

    /// Method to implement the logic for doubling down on a bet
    fn double_down(&mut self, player: &mut Player) {
        // Call the double_down() method of the player, and deal them another card
        player.double_down();
        player.receive_card(self.deck.get_next_card().unwrap());
        player.compute_hand_value();

        if !player.busted() {
            player.stand();
        } else {
            println!("{}", "-".to_string().repeat(80));
            self.dealers_hand.display_hand_without_hole();
            println!("\n\n");
            player.bj_hand.display_hand();
            player.display_balance();
            println!("Busted, you lost the bet");
            self.balance += player.lose_bet() as f32;
        }
    }

    /// Method to execute the logic for a player to split
    fn split(&mut self, player: &mut Player) {
        player.split(
            self.deck.get_next_card().unwrap(),
            self.deck.get_next_card().unwrap(),
        );
        println!("{}", "-".to_string().repeat(80));
        self.dealers_hand.display_hand_without_hole();
        println!("\n\n");
        player.bj_hand.display_hand();
        player.display_balance();
    }

    /// Implments the logic that deals the initial cards at the start of a hand, checks if
    /// dealer has a blackjack and whether or not `player` has a blackjack, and then executes the appropriate logic
    fn deal_hand(&mut self, player: &mut Player) {
        assert!(
            !player.bj_hand.bets.is_empty(),
            "bet must be placed by the player before proceeding"
        );

        // Check if deck needs to be shuffled
        if self.deck.shuffle_flag {
            println!("Shuffling...");
            self.deck.shuffle(self.n_shuffles);
        }

        // Deal cards to player and dealer
        player.receive_card(self.deck.get_next_card().unwrap());

        self.dealers_hand
            .receive_card(self.deck.get_next_card().unwrap());

        player.receive_card(self.deck.get_next_card().unwrap());

        self.dealers_hand
            .receive_card(self.deck.get_next_card().unwrap());

        player.compute_hand_value();
        self.dealers_hand.compute_hand_value();

        // Check if dealer has blackjack or not, then perform the appropriate logic
        println!("{:-<80}", "");
        if self.dealers_hand.is_blackjack() {
            // Display state of table, no need to keep dealers hole card hidden
            self.dealers_hand.display_hand();
            self.dealers_hand.display_hand_value();
            println!("\n\n");
            player.bj_hand.display_hand();
            player.display_balance();
            println!();

            // Check if player has blackjack
            let mut result_str = String::from("Dealer has blackjack: ");
            if player.has_blackjack() {
                result_str.push_str("you pushed");
                player.balance += player.bj_hand.bets.pop().unwrap() as f32;
            } else {
                result_str.push_str("you lost the bet");
                self.balance += player.bj_hand.bets.pop().unwrap() as f32;
            }
            println!("{result_str}");
        } else {
            self.dealers_hand.display_hand_without_hole();
            println!("\n\n");
            player.bj_hand.display_hand();
            player.display_balance();

            // Check if player has a blackjack
            if player.has_blackjack() {
                let winnings = 1.5 * (player.bets()[0] as f32);
                self.balance -= winnings;
                player.balance += winnings + (player.bj_hand.bets.pop().unwrap() as f32);
                println!("You got blackjack, winnings: {:2.2}", winnings);
            }
        }
    }

    /// A method for computing and returning the optimal hand for the dealer at the end of a hand of blackjack.
    /// The dealers draws cards according to the rules of blackjack, then the optimal hand once a hand with a value of no less than 17 is achieved
    fn get_dealers_optimal_final_hand(&mut self) -> u8 {
        self.dealers_hand.compute_optimal_final_hand(&mut self.deck)
    }

    /// This method will complete a hand of blackjack, it will check `player` optimal hand(s) against the dealer and payout bets accordingly
    /// A call to this method will also reset the state of `player` and the dealer to have empty hands i.e. `player` and dealer will be in a state to play another round
    fn finish_hand(&mut self, player: &mut Player) {
        // Compute players optimal hands, if they have any bets remaining at the table
        // if the player has no remaining bets then, just skip to reseting dealer/player
        if let Some(players_optimal_hands) = player.get_optimal_hands() {
            let dealers_optimal_hand = self.get_dealers_optimal_final_hand();
            let mut winnings: f32 = 0.0;
            let mut refunded_bets: f32 = 0.0;
            let result_messages = if player.bets().len() > 1 {
                let mut res = vec![];

                for (i, bet) in player.bets().iter().enumerate() {
                    if *bet > 0 {
                        if dealers_optimal_hand > 21
                            || players_optimal_hands[&i] > dealers_optimal_hand
                        {
                            res.push(format!("You won bet #{}: ${}", i + 1, *bet));
                            self.balance -= *bet as f32;
                            winnings += *bet as f32;
                        } else if players_optimal_hands[&i] == dealers_optimal_hand {
                            res.push(format!("You pushed bet #{}: ${}", i + 1, *bet));
                            refunded_bets += *bet as f32;
                        } else {
                            self.balance += *bet as f32;
                            res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
                        }
                    } else {
                        res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
                    }
                }
                res
            } else {
                if dealers_optimal_hand > 21 || players_optimal_hands[&0] > dealers_optimal_hand {
                    self.balance -= player.bets()[0] as f32;
                    winnings += player.bets()[0] as f32;
                    vec![String::from("You won the bet")]
                } else if players_optimal_hands[&0] == dealers_optimal_hand {
                    refunded_bets += player.bets()[0] as f32;
                    vec![String::from("You pushed")]
                } else {
                    self.balance += player.bets()[0] as f32;
                    vec![String::from("You lost the bet")]
                }
            };

            self.display_end_of_hand_state(&player, result_messages, winnings);
            player.balance += 2.0 * winnings;
            player.balance += refunded_bets;
        };

        self.dealers_hand.reset();
        player.reset();
    }

    /// A method that will display the state of the game on the console at the end of a hand
    fn display_end_of_hand_state(
        &self,
        player: &Player,
        result_messages: Vec<String>,
        winnings: f32,
    ) {
        println!("{}", "-".to_string().repeat(80));
        self.dealers_hand.display_hand();
        self.dealers_hand.display_hand_value();
        println!("\n\n");
        player.display_hand();
        player.display_balance();

        // For readability
        println!("\n");

        for msg in result_messages {
            println!("{msg}");
        }
        if self.dealers_hand.busted() {
            println!("Dealer busted");
        }
        println!("Winnings: ${winnings:2.2}");
    }
}

// impl BlackjackTable<PlayerCLI> for BlackjackTableCLI {
//     fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> Self {
//         let deck = Deck::new(n_decks);

//         BlackjackTableCLI {
//             deck,
//             balance: starting_balance,
//             dealers_hand: DealersBlackjackHandCLI::new(),
//             n_shuffles,
//         }
//     }

//     /// Takes a Player struct, `player` and places a bet
//     fn place_bet(&self, player: &mut PlayerCLI, bet: f32) -> Result<(), BlackjackGameError> {
//         if bet <= 0.0 {
//             return Err(BlackjackGameError {
//                 message: "Bet must be a positive amount".to_string(),
//             });
//             // return Err("Bet must be a positive amount".to_string());
//         } else if self.balance < 1.5 * bet {
//             return Err(BlackjackGameError {
//                 message: "Insufficient table balance to payout bet".to_string(),
//             });
//         }
//         player.place_bet(bet)
//     }

//     /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
//     /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
//     fn play_option(
//         &mut self,
//         player: &mut PlayerCLI,
//         options: &HashMap<i32, String>,
//         option: i32,
//     ) -> Result<(), BlackjackGameError> {
//         match options[&option].as_str() {
//             "stand" => Ok(self.stand(player)),
//             "hit" => Ok(self.hit(player)),
//             "split" => Ok(self.split(player)),
//             "double down" => Ok(self.double_down(player)),
//             _ => Err(BlackjackGameError {
//                 message: format!("{} is not a valid option", option),
//             }),
//         }
//     }

//     /// Takes a Player struct `player` and changes its state via its stand method
//     fn stand(&self, player: &mut PlayerCLI) {
//         player.stand();
//         if !player.turn_is_over() {
//             println!("{}", "-".to_string().repeat(80));
//             self.dealers_hand.display_hand_without_hole();
//             println!("\n\n");
//             player.bj_hand.display_hand();
//             player.display_balance();
//         }
//     }

//     /// Takes a Player `player` and changes the state `players`'s hand by dealing another card.
//     /// The function then computes if the player has busted or not and adjusts the bets of the player accordingly
//     fn hit(&mut self, player: &mut PlayerCLI) {
//         player.receive_card(self.deck.get_next_card().unwrap());
//         player.compute_hand_value();

//         println!("{}", "-".to_string().repeat(80));
//         self.dealers_hand.display_hand_without_hole();
//         println!("\n\n");
//         player.bj_hand.display_hand();
//         player.display_balance();

//         if player.busted() {
//             println!("Busted, you lost the bet");
//             self.balance += player.lose_bet() as f32;
//         }
//     }

//     /// Method to implement the logic for doubling down on a bet
//     fn double_down(&mut self, player: &mut PlayerCLI) {
//         // Call the double_down() method of the player, and deal them another card
//         player.double_down();
//         player.receive_card(self.deck.get_next_card().unwrap());
//         player.compute_hand_value();

//         if !player.busted() {
//             player.stand();
//         } else {
//             println!("{}", "-".to_string().repeat(80));
//             self.dealers_hand.display_hand_without_hole();
//             println!("\n\n");
//             player.bj_hand.display_hand();
//             player.display_balance();
//             println!("Busted, you lost the bet");
//             self.balance += player.lose_bet() as f32;
//         }
//     }

//     /// Method to execute the logic for a player to split
//     fn split(&mut self, player: &mut PlayerCLI) {
//         player.split(
//             self.deck.get_next_card().unwrap(),
//             self.deck.get_next_card().unwrap(),
//         );
//         println!("{}", "-".to_string().repeat(80));
//         self.dealers_hand.display_hand_without_hole();
//         println!("\n\n");
//         player.bj_hand.display_hand();
//         player.display_balance();
//     }

//     /// Implments the logic that deals the initial cards at the start of a hand, checks if
//     /// dealer has a blackjack and whether or not `player` has a blackjack, and then executes the appropriate logic
//     fn deal_hand(&mut self, player: &mut PlayerCLI) {
//         assert!(
//             !player.bj_hand.bets.is_empty(),
//             "bet must be placed by the player before proceeding"
//         );

//         // Check if deck needs to be shuffled
//         if self.deck.shuffle_flag {
//             println!("Shuffling...");
//             self.deck.shuffle(self.n_shuffles);
//         }

//         // Deal cards to player and dealer
//         player.receive_card(self.deck.get_next_card().unwrap());

//         self.dealers_hand
//             .receive_card(self.deck.get_next_card().unwrap());

//         player.receive_card(self.deck.get_next_card().unwrap());

//         self.dealers_hand
//             .receive_card(self.deck.get_next_card().unwrap());

//         player.compute_hand_value();
//         self.dealers_hand.compute_hand_value();

//         // Check if dealer has blackjack or not, then perform the appropriate logic
//         println!("{:-<80}", "");
//         if self.dealers_hand.is_blackjack() {
//             // Display state of table, no need to keep dealers hole card hidden
//             self.dealers_hand.display_hand();
//             self.dealers_hand.display_hand_value();
//             println!("\n\n");
//             player.bj_hand.display_hand();
//             player.display_balance();
//             println!();

//             // Check if player has blackjack
//             let mut result_str = String::from("Dealer has blackjack: ");
//             if player.has_blackjack() {
//                 result_str.push_str("you pushed");
//                 player.balance += player.bj_hand.bets.pop().unwrap() as f32;
//             } else {
//                 result_str.push_str("you lost the bet");
//                 self.balance += player.bj_hand.bets.pop().unwrap() as f32;
//             }
//             println!("{result_str}");
//         } else {
//             self.dealers_hand.display_hand_without_hole();
//             println!("\n\n");
//             player.bj_hand.display_hand();
//             player.display_balance();

//             // Check if player has a blackjack
//             if player.has_blackjack() {
//                 let winnings = 1.5 * (player.bets()[0] as f32);
//                 self.balance -= winnings;
//                 player.balance += winnings + (player.bj_hand.bets.pop().unwrap() as f32);
//                 println!("You got blackjack, winnings: {:2.2}", winnings);
//             }
//         }
//     }

//     /// A method for computing and returning the optimal hand for the dealer at the end of a hand of blackjack.
//     /// The dealers draws cards according to the rules of blackjack, then the optimal hand once a hand with a value of no less than 17 is achieved
//     fn get_dealers_optimal_final_hand(&mut self) -> u8 {
//         self.dealers_hand.compute_optimal_final_hand(&mut self.deck)
//     }

//     /// This method will complete a hand of blackjack, it will check `player` optimal hand(s) against the dealer and payout bets accordingly
//     /// A call to this method will also reset the state of `player` and the dealer to have empty hands i.e. `player` and dealer will be in a state to play another round
//     fn finish_hand(&mut self, player: &mut PlayerCLI) {
//         // Compute players optimal hands, if they have any bets remaining at the table
//         // if the player has no remaining bets then, just skip to reseting dealer/player
//         if let Some(players_optimal_hands) = player.get_optimal_hands() {
//             let dealers_optimal_hand = self.get_dealers_optimal_final_hand();
//             let mut winnings: f32 = 0.0;
//             let mut refunded_bets: f32 = 0.0;
//             let result_messages = if player.bets().len() > 1 {
//                 let mut res = vec![];

//                 for (i, bet) in player.bets().iter().enumerate() {
//                     if *bet > 0 {
//                         if dealers_optimal_hand > 21
//                             || players_optimal_hands[&i] > dealers_optimal_hand
//                         {
//                             res.push(format!("You won bet #{}: ${}", i + 1, *bet));
//                             self.balance -= *bet as f32;
//                             winnings += *bet as f32;
//                         } else if players_optimal_hands[&i] == dealers_optimal_hand {
//                             res.push(format!("You pushed bet #{}: ${}", i + 1, *bet));
//                             refunded_bets += *bet as f32;
//                         } else {
//                             self.balance += *bet as f32;
//                             res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
//                         }
//                     } else {
//                         res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
//                     }
//                 }
//                 res
//             } else {
//                 if dealers_optimal_hand > 21 || players_optimal_hands[&0] > dealers_optimal_hand {
//                     self.balance -= player.bets()[0] as f32;
//                     winnings += player.bets()[0] as f32;
//                     vec![String::from("You won the bet")]
//                 } else if players_optimal_hands[&0] == dealers_optimal_hand {
//                     refunded_bets += player.bets()[0] as f32;
//                     vec![String::from("You pushed")]
//                 } else {
//                     self.balance += player.bets()[0] as f32;
//                     vec![String::from("You lost the bet")]
//                 }
//             };

//             self.display_end_of_hand_state(&player, result_messages, winnings);
//             player.balance += 2.0 * winnings;
//             player.balance += refunded_bets;
//         };

//         self.dealers_hand.reset();
//         player.reset();
//     }
// }

/// A struct for implementing the control flow logic/error checking of a blackjack game played via the console
pub struct BlackjackGameCLI {
    table: BlackjackTableCLI,
    player: Player,
    minimum_bet: u32,
}

impl BlackjackGameCLI {
    /// Returns a new BlackjackGameCLI
    pub fn new(minimum_bet: u32, player: Player, table: BlackjackTableCLI) -> BlackjackGameCLI {
        BlackjackGameCLI {
            minimum_bet,
            player,
            table,
        }
    }

    /// Plays a game of blackjack
    pub fn play(&mut self) -> std::io::Result<()> {
        // The main game loop
        'main: loop {
            // get a valid bet from the player via console
            let bet = loop {
                println!("Please enter a valid bet, minimum ${}", self.minimum_bet);
                let mut players_entered_bet = String::new();
                if let Err(e) = std::io::stdin().read_line(&mut players_entered_bet) {
                    println!("Error parsing bet, {e}");
                    continue;
                }
                match u32::from_str(players_entered_bet.trim()) {
                    Err(e) => {
                        println!("{e}");
                        println!("Error parsing entered amount as an integer, please ensure entered amount is parseable as a non-negative integer");
                        continue;
                    }
                    Ok(n) if n < self.minimum_bet => {
                        println!("Minimum bet is ${}", self.minimum_bet);
                        continue;
                    }
                    Ok(n) => break n,
                }
            };

            // Place the bet and esure that the bet is valid
            if let Err(e) = self.table.place_bet(&mut self.player, bet as f32) {
                println!("{e}");
                continue;
            }
            // deal hand
            self.table.deal_hand(&mut self.player);

            while !self.player.turn_is_over() {
                let options = self.player.get_playing_options();
                self.table.display_playing_options(&options, &self.player);
                'outer: loop {
                    let option = 'validation_loop: loop {
                        let mut users_input = String::new();
                        std::io::stdin().read_line(&mut users_input)?;

                        let choice = match i32::from_str(users_input.trim()) {
                            Ok(n) => n,
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        };

                        if options.contains_key(&choice) {
                            break 'validation_loop choice;
                        } else {
                            println!("Please enter a valid option");
                            continue;
                        }
                    };

                    match self.table.play_option(&mut self.player, &options, option) {
                        Ok(()) => break 'outer,
                        Err(e) => {
                            println!("error: {e}");
                            continue;
                        }
                    }
                }
            }

            self.table.finish_hand(&mut self.player);
            println!("\n");
            println!("Play another round? (y/n): ");
            let mut users_input = String::new();
            loop {
                std::io::stdin().read_line(&mut users_input)?;
                if users_input.trim().to_lowercase() == "y"
                    || users_input.trim().to_lowercase() == "n"
                {
                    break;
                }
                println!("please enter a valid choice");
                users_input = String::new();
            }

            if users_input.trim() == "y" {
                continue;
            }

            break 'main;
        }

        Ok(())
    }
}
