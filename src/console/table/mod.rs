mod dealers_hand;

use crate::console::player::ConsolePlayer;
use crate::{BlackjackGameError, BlackjackTable, Deck, Player};
use dealers_hand::ConsoleDealersBlackjackHand;
use std::collections::HashMap;

/// / A struct to implement the logic for a game of blackjack played over the console. Contains all the appropraite methods that imlement the
/// / typical valid rules of a blackjack game. Intended to interact with a player struct.
pub struct ConsoleBlackjackTable {
    deck: Deck,
    balance: f32,
    dealers_hand: ConsoleDealersBlackjackHand,
    n_shuffles: u32,
}

impl ConsoleBlackjackTable {
    /// Takes a HashMap<i32, String> of numbered options and prints the options formatted nicely.
    pub fn display_playing_options(&self, options: &HashMap<i32, String>, player: &ConsolePlayer) {
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

    /// A method that will display the state of the game on the console at the end of a hand
    pub fn display_end_of_hand_state(
        &self,
        player: &ConsolePlayer,
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

    /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
    /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
    fn get_playing_options(
        &mut self,
        player: &mut ConsolePlayer,
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
}

impl BlackjackTable<ConsolePlayer> for ConsoleBlackjackTable {
    /// Creates a new instance of a BlackjackTableCLI struct
    fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> Self {
        let deck = Deck::new(n_decks);

        ConsoleBlackjackTable {
            deck,
            balance: starting_balance,
            dealers_hand: ConsoleDealersBlackjackHand::new(),
            n_shuffles,
        }
    }

    /// Takes a Player struct, `player` and places a bet
    fn place_bet(&self, player: &mut ConsolePlayer, bet: f32) -> Result<(), BlackjackGameError> {
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

    /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
    /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
    fn play_option(
        &mut self,
        player: &mut ConsolePlayer,
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
    fn stand(&self, player: &mut ConsolePlayer) {
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
    fn hit(&mut self, player: &mut ConsolePlayer) {
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
    fn double_down(&mut self, player: &mut ConsolePlayer) {
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
    fn split(&mut self, player: &mut ConsolePlayer) {
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
    fn deal_hand(&mut self, player: &mut ConsolePlayer) {
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
    fn finish_hand(&mut self, player: &mut ConsolePlayer) {
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
}
