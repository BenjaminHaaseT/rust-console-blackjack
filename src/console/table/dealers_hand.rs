use crate::{compute_optimal_hand, Card, Deck};
use std::rc::Rc;

/// Trait that performs all the bookeeping for tracking the dealers hand.
/// A helper trait for  `ConsoleBlackjackTable`.
pub struct ConsoleDealersBlackjackHand {
    hand: Vec<Rc<Card>>,
    hand_value: Vec<u8>,
    hand_str: String,
    hand_value_str: String,
}

impl ConsoleDealersBlackjackHand {
    /// Creates a new `DealersBlackjackHand` struct
    pub fn new() -> Self {
        let hand = vec![];
        let hand_value = vec![];
        let hand_str = String::new();
        let hand_value_str = String::new();

        ConsoleDealersBlackjackHand {
            hand,
            hand_value,
            hand_str,
            hand_value_str,
        }
    }

    /// Prints a string representing the value of the dealers hand to the console
    pub fn display_hand_value(&self) {
        println!("{:<10}{}", "Value", self.hand_value_str);
    }

    /// Display dealers hand without revealing the hole card i.e at the begginning of a hand
    pub fn display_hand_without_hole(&self) {
        println!(
            "{:<10}{} {}",
            "Dealer:",
            Card::display_facedown(),
            self.hand[1]
        );
    }

    /// Print the value of dealers hand to the console, formatted in a nice way
    pub fn display_hand(&self) {
        println!("{:<10}{}", "Dealer:", self.hand_str);
    }

    /// Checks whether the dealers hand has busted or not
    pub fn busted(&self) -> bool {
        if self.hand_value.len() == 2 {
            self.hand_value[0] > 21 && self.hand_value[1] > 21
        } else {
            self.hand_value[0] > 21
        }
    }

    /// Checks whether or not the dealers hand has busted
    pub fn is_blackjack(&self) -> bool {
        self.hand.len() == 2
            && ((self.hand[0].rank == "A" && self.hand[1].val == 10)
                || (self.hand[0].val == 10 && self.hand[1].rank == "A"))
    }

    /// Computes the dealers hand value, and updates the string that represents that value for display
    /// via standard output ie console
    pub fn compute_hand_value(&mut self) {
        if self.hand.len() == 2 {
            self.hand_value.push(self.hand.iter().map(|c| c.val).sum());

            // We need to check if there is an alternative hand value possible
            if self.hand[0].rank == "A" || self.hand[1].rank == "A" {
                let alternative_hand_val = self.hand_value[0] + 10;
                self.hand_value.push(alternative_hand_val);
            }
        } else {
            let new_card_val = self.hand.last().expect("hand should not be empty").val;
            self.hand_value[0] += new_card_val;
            if self.hand_value.len() == 2 {
                self.hand_value[1] += new_card_val;
            } else if self.hand_value[0] <= 11 && new_card_val == 1 {
                let alternative_hand = self.hand_value[0] + 10;
                self.hand_value.push(alternative_hand);
            }
        }

        // Update the string that will be displayed
        self.hand_value_str = if self.hand_value.len() == 2 {
            if self.hand_value[0] > 21 || self.hand_value[1] > 21 {
                format!("{}", u8::min(self.hand_value[0], self.hand_value[1]))
            } else if self.hand_value[0] == 21 || self.hand_value[1] == 21 {
                format!("21")
            } else {
                format!("{}/{}", self.hand_value[0], self.hand_value[1])
            }
        } else {
            self.hand_value[0].to_string()
        }
    }

    /// Receive a new card, `card` which will be pushed to dealers hand
    /// function will also update the value of `self.hand_str`, the string representing the cards to be printed to the console
    pub fn receive_card(&mut self, card: Rc<Card>) {
        self.hand.push(Rc::clone(&card));
        if self.hand_str.is_empty() {
            self.hand_str.push_str(card.to_string().as_str());
        } else {
            self.hand_str
                .push_str(format!(" {}", card.to_string()).as_str());
        }
    }

    /// Method for computing the optimal, valid final hand according to the rules of blackjack
    pub fn compute_optimal_final_hand(&mut self, deck: &mut Deck) -> u8 {
        if self.hand_value.len() == 1 {
            while self.hand_value[0] < 17 && self.hand_value.len() < 2 {
                self.receive_card(deck.get_next_card().unwrap());
                self.compute_hand_value();
            }
        }

        // Check to see if we have a hand multiple hand values
        while self.hand_value.len() == 2 && self.hand_value[0] < 17 && self.hand_value[1] < 17 {
            self.receive_card(deck.get_next_card().unwrap());
            self.compute_hand_value();
        }

        // Ensure the dealer has a valid hand no less than 17
        while self.hand_value.len() == 2
            && ((self.hand_value[0] < 17 && self.hand_value[1] > 21)
                || (self.hand_value[0] > 21 && self.hand_value[1] < 17))
        {
            self.receive_card(deck.get_next_card().unwrap());
            self.compute_hand_value();
        }

        // Now we are sure the dealer has drawn enough cards to either bust or have a valid hand according to rules of blackjack
        compute_optimal_hand(&self.hand_value)
    }

    /// Resets the dealers hand to play another round
    pub fn reset(&mut self) {
        self.hand.clear();
        self.hand_value.clear();
        self.hand_str.clear();
        self.hand_value_str.clear();
    }
}
