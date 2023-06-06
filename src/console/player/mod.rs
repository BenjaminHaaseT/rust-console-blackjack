mod players_hand;
use crate::{compute_optimal_hand, BlackjackGameError, Card, Player};
use players_hand::ConsolePlayersBlackjackHand;
use std::collections::HashMap;
use std::rc::Rc;

/// A struct that represents a player that plays via the console
pub struct ConsolePlayer {
    name: String,
    pub balance: f32,
    pub bj_hand: ConsolePlayersBlackjackHand,
    pub hand_idx: usize,
}

impl ConsolePlayer {
    /// Creates a new player struct
    pub fn new(name: String, balance: f32) -> ConsolePlayer {
        ConsolePlayer {
            name,
            balance,
            bj_hand: ConsolePlayersBlackjackHand::new(),
            hand_idx: 0usize,
        }
    }

    /// Gets the optimal hand of a player for valid bet the player has. If the player has no more valid bets, then
    /// the method returns None
    pub fn get_optimal_hands(&self) -> Option<HashMap<usize, u8>> {
        let mut res = HashMap::new();
        for i in 0..self.bj_hand.bets.len() {
            if self.bj_hand.bets[i] > 0 {
                res.insert(i, compute_optimal_hand(&self.bj_hand.hand_values[i]));
            }
        }
        match res.is_empty() {
            true => None,
            false => Some(res),
        }
    }

    /// Only icreases players hand index by 1 in order to signal that this hand is finished.
    pub fn stand(&mut self) {
        self.hand_idx += 1;
    }

    /// Provides a boolean flag signaling whether or not the player is finished with their hand
    pub fn turn_is_over(&self) -> bool {
        self.hand_idx == self.bj_hand.bets.len()
    }

    /// Takes `bet` representing a bet at a blackjack table, and updates the balance then passes the value along to
    /// the players PlayersBlackjackHand struct to execute the necessary logic for that struct as well
    pub fn place_bet(&mut self, bet: f32) -> Result<(), BlackjackGameError> {
        if bet > self.balance {
            return Err(BlackjackGameError {
                message: "Insufficient funds to place that bet".to_string(),
            });
        }
        self.balance -= bet;
        self.bj_hand.place_bet(bet as u32);
        Ok(())
    }

    /// Returns the value of the current bet and resets its value to 0 for post processing
    /// Increases the players hand_idx by 1, to signal this hand is finished.
    pub fn lose_bet(&mut self) -> u32 {
        let bet = self.bj_hand.lose_bet(self.hand_idx);
        self.hand_idx += 1;
        bet
    }

    /// Queries the players hand struct to see what the valid options are for the player to takes
    /// function will panic if the players current hand has busted or the player has not placed any bets
    pub fn get_playing_options(&self) -> HashMap<i32, String> {
        assert!(!self.bj_hand.busted(self.hand_idx), "hand should be over");
        assert!(
            !self.bj_hand.bets.is_empty(),
            "player should have placed a bet"
        );
        let mut playing_options = HashMap::new();
        playing_options.insert(1, "stand".to_string());
        playing_options.insert(2, "hit".to_string());
        let mut playing_option = 3;

        if self.bj_hand.can_split(self.hand_idx)
            && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
            && self.bj_hand.hand.len() < 4
        {
            playing_options.insert(playing_option, "split".to_string());
            playing_option += 1;
        }

        if self.bj_hand.can_double_down(self.hand_idx)
            && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
            && self.bj_hand.hand.len() == 1
        {
            playing_options.insert(playing_option, "double down".to_string());
        }

        playing_options
    }

    /// Wrapper method for self.bj_hand.receive_card()
    pub fn receive_card(&mut self, card: Rc<Card>) {
        self.bj_hand.receive_card(card, self.hand_idx);
    }

    /// Method that allows the player to double down on a bet
    pub fn double_down(&mut self) {
        let cur_bet = self.bj_hand.double_down(self.hand_idx);
        self.balance -= cur_bet as f32;
    }

    /// Method that allwos the player to split their current hand, assumes all the conditions necessary for a valid split have been met
    pub fn split(&mut self, card1: Rc<Card>, card2: Rc<Card>) {
        let cur_bet = self.bj_hand.split(self.hand_idx);
        self.balance -= cur_bet as f32;
        // Deal a the new cards to each new hand respectively, and compute their hand values
        self.bj_hand.receive_card(card1, self.hand_idx);
        self.bj_hand.compute_hand_value(self.hand_idx);
        self.bj_hand.receive_card(card2, self.hand_idx + 1);
        self.bj_hand.compute_hand_value(self.hand_idx + 1);
    }

    /// Returns whether or not the player has a blackjack or not, again is a wrapper method for self.bj_hand.is_blackjack()
    pub fn has_blackjack(&self) -> bool {
        self.bj_hand.is_blackjack(self.hand_idx)
    }

    /// Returns a boolean whether or not the player has busted or not, is a wrapper method for self.bj_hand.busted()
    pub fn busted(&self) -> bool {
        self.bj_hand.busted(self.hand_idx)
    }

    /// Computes the hand value of the current hand, and updates the state of self.bj_hand. Acts as a wrapper for self.bj_hand.compute_value()
    pub fn compute_hand_value(&mut self) {
        self.bj_hand.compute_hand_value(self.hand_idx);
    }

    /// A simple getter method that returns the players vector of bets
    pub fn bets(&self) -> &Vec<u32> {
        &self.bj_hand.bets
    }

    /// A wrapper method for self.bj_hand.display_hand(),  displays the players hand in a nice way
    pub fn display_hand(&self) {
        self.bj_hand.display_hand();
    }

    /// Displays the players balance to the console
    pub fn display_balance(&self) {
        println!("{:<10}${}", "Balance:", self.balance)
    }

    /// Resets all of the necessary fields so the player can play another hand of blackjack
    pub fn reset(&mut self) {
        self.hand_idx = 0;
        self.bj_hand.reset();
    }
}

impl Player for ConsolePlayer {}
