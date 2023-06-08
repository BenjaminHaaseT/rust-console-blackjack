use crate::Card;
use std::rc::Rc;

pub struct ConsolePlayersBlackjackHand {
    pub hand: Vec<Vec<Rc<Card>>>,
    pub hand_values: Vec<Vec<u8>>,
    pub bets: Vec<u32>,
    hand_str: Vec<String>,
    hand_values_str: Vec<String>,
    bets_str: Vec<String>,
}

impl ConsolePlayersBlackjackHand {
    /// Creates a new PlayersBlackjackHand struct
    pub fn new() -> Self {
        let hand = vec![vec![]];
        let hand_values = vec![vec![]];
        let bets = vec![];
        let hand_str = vec![String::new()];
        let hand_values_str = vec![String::new()];
        let bets_str = vec![];

        ConsolePlayersBlackjackHand {
            hand,
            hand_values,
            bets,
            hand_str,
            hand_values_str,
            bets_str,
        }
    }

    /// Takes in `bet` and updates the state of the bet associated with a particular hand.
    pub fn place_bet(&mut self, bet: u32) {
        self.bets.push(bet);
        self.bets_str.push(bet.to_string());
    }

    /// Sets the bet of the current hand to 0 and returns the value of the current bet.
    pub fn lose_bet(&mut self, hand_idx: usize) -> u32 {
        let res = self.bets[hand_idx];
        self.bets[hand_idx] = 0;
        res
    }

    /// Simple function to check whether or not the current hand i.e. the hand at index `hand_idx` can split.
    /// The function will panice if `hand_idx` is not a valid index or the hand vector is empty.
    pub fn can_split(&self, hand_idx: usize) -> bool {
        self.hand[hand_idx].len() == 2 && self.hand[hand_idx][0].rank == self.hand[hand_idx][1].rank
    }

    /// Simple fucntion to check whether the current hand i.e. the hand at index `hand_idx` can double down.
    /// The function will panic if `hand_idx` is not a valid index, or hand_values vector is empty.
    pub fn can_double_down(&self, hand_idx: usize) -> bool {
        if self.hand_values[hand_idx].len() == 2 {
            hand_idx == 0
                && self.hand[hand_idx].len() == 2
                && ((self.hand_values[hand_idx][0] == 9
                    || self.hand_values[hand_idx][0] == 10
                    || self.hand_values[hand_idx][0] == 11)
                    || (self.hand_values[hand_idx][1] == 9
                        || self.hand_values[hand_idx][1] == 10
                        || self.hand_values[hand_idx][1] == 11))
        } else {
            hand_idx == 0
                && self.hand[hand_idx].len() == 2
                && (self.hand_values[hand_idx][0] == 9
                    || self.hand_values[hand_idx][0] == 10
                    || self.hand_values[hand_idx][0] == 11)
        }
    }

    /// Receive a new card, add it to the players hand and update the string representing
    /// the current hand as well
    pub fn receive_card(&mut self, card: Rc<Card>, hand_idx: usize) {
        self.hand[hand_idx].push(Rc::clone(&card));
        // Now update string representing hand
        if self.hand_str[hand_idx].is_empty() {
            self.hand_str[hand_idx].push_str(card.to_string().as_str());
        } else {
            self.hand_str[hand_idx].push_str(format!(" {}", card.to_string()).as_str());
        }
    }

    /// Implement the logic for doubling down on a bet, updates the bets and the bets_str for display purposes.
    /// Returns the value of the current bet for updating the players balance.
    pub fn double_down(&mut self, hand_idx: usize) -> u32 {
        let cur_bet = self.bets[hand_idx];
        self.bets[hand_idx] += cur_bet;
        self.bets_str[hand_idx] = self.bets[hand_idx].to_string();
        cur_bet
    }

    /// Implements the logic for splitting a valid hand. Returns the value of the current bet to update the players balance.
    pub fn split(&mut self, hand_idx: usize) -> u32 {
        // Get current bet and add another to the vector that keeps track of the number of bets, also push another bet to the vector of strings
        // representing the current total number of bets the player has at the table
        let cur_bet = self.bets[hand_idx];
        self.bets.insert(hand_idx + 1, cur_bet);
        self.bets_str.insert(hand_idx + 1, cur_bet.to_string());

        // Get the card that will be the geneis of the new hand, reset the hand value for the current hand and
        // push a new vector onto hand_values representing the new empty hand value
        let new_hand = self.hand[hand_idx].pop().unwrap();
        self.hand.insert(hand_idx + 1, vec![new_hand]);

        self.hand_values[hand_idx].clear();
        self.hand_values.insert(hand_idx + 1, vec![]);

        self.hand_values_str[hand_idx].clear();
        self.hand_values_str.insert(hand_idx + 1, String::new());

        // Now update the hand_str and hand_values_str
        self.hand_str[hand_idx] = self.hand[hand_idx][0].to_string();
        self.hand_str
            .insert(hand_idx + 1, self.hand[hand_idx + 1][0].to_string());

        cur_bet
    }

    /// Compute the value of the players current hand and update the formatted string
    /// representing the value of the current hand as well
    pub fn compute_hand_value(&mut self, hand_idx: usize) {
        if self.hand[hand_idx].len() == 2 {
            self.hand_values[hand_idx].push(self.hand[hand_idx].iter().map(|c| c.val).sum::<u8>());

            // Need to check if we have more than one possible value for the given hand
            if self.hand[hand_idx][0].rank == "A" || self.hand[hand_idx][1].rank == "A" {
                let alternative_hand_val = self.hand_values[hand_idx]
                    .last()
                    .expect("hand should not be empty")
                    + 10;

                self.hand_values[hand_idx].push(alternative_hand_val);
            }
        } else {
            let new_card_val = self.hand[hand_idx]
                .last()
                .expect("hand should not be empty")
                .val;
            self.hand_values[hand_idx][0] += new_card_val;
            if self.hand_values[hand_idx].len() == 2 {
                self.hand_values[hand_idx][1] += new_card_val;
            } else if self.hand_values[hand_idx][0] <= 11 && new_card_val == 1 {
                // Check if we need another hand value to represent all possible values of the hand
                let alternative_hand = self.hand_values[hand_idx][0] + 10;
                self.hand_values[hand_idx].push(alternative_hand);
            }
        }

        // Now update the string that represents the value
        self.hand_values_str[hand_idx] = if self.hand_values[hand_idx].len() == 2 {
            if self.hand_values[hand_idx][0] > 21 || self.hand_values[hand_idx][1] > 21 {
                format!(
                    "{}",
                    u8::min(self.hand_values[hand_idx][0], self.hand_values[hand_idx][1])
                )
            } else {
                format!(
                    "{}/{}",
                    self.hand_values[hand_idx][0], self.hand_values[hand_idx][1]
                )
            }
        } else {
            self.hand_values[hand_idx][0].to_string()
        };
    }

    /// Checks if the players current hand is a blackjack, ensures it must be a natural blackjack by
    /// checking that `self.hand_idx` is equal to zero i.e. it is the first hand dealt to the player, not
    /// a hand dealt after splitting
    pub fn is_blackjack(&self, hand_idx: usize) -> bool {
        hand_idx == 0
            && self.hand[0].len() == 2
            && ((self.hand[0][0].rank == "A" && self.hand[0][1].val == 10)
                || (self.hand[0][0].val == 10 && self.hand[0][1].rank == "A"))
    }

    /// Checks whether the current hand has busted or not
    pub fn busted(&self, hand_idx: usize) -> bool {
        if self.hand_values[hand_idx].len() == 2 {
            self.hand_values[hand_idx][0] > 21 && self.hand_values[hand_idx][1] > 21
        } else {
            self.hand_values[hand_idx][0] > 21
        }
    }

    /// Displays the players current blackjack hand in the console printed in a nice looking format
    pub fn display_hand(&self) {
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

    /// Resets the hand to a new empty hand
    pub fn reset(&mut self) {
        self.hand = vec![vec![]];
        self.hand_values = vec![vec![]];
        self.bets.clear();
        self.hand_str = vec![String::from("")];
        self.hand_values_str = vec![String::from("")];
        self.bets_str.clear();
    }
}
