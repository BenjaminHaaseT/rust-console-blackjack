/// This module contains all the necessary structs for a blackjack game played on the console.
use crate::{
    BlackjackGameError, BlackjackTable, Card, DealersBlackjackHand, Deck, Player,
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

/// / A struct to implement the logic for a game of blackjack played over the console. Contains all the appropraite methods that imlement the
/// / typical valid rules of a blackjack game. Intended to interact with a player struct.
pub struct BlackjackTableCLI {
    deck: Deck,
    balance: f32,
    dealers_hand: DealersBlackjackHand,
    n_shuffles: u32,
}

impl BlackjackTableCLI {
    // /// Creates a new instance of a BlackjackTableCLI struct
    // pub fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> Self {
    //     let deck = Deck::new(n_decks);

    //     BlackjackTableCLI {
    //         deck,
    //         balance: starting_balance,
    //         dealers_hand: DealersBlackjackHand::new(),
    //         n_shuffles,
    //     }
    // }

    // /// Takes a Player struct, `player` and places a bet
    // fn place_bet(&self, player: &mut Player, bet: f32) -> Result<(), BlackjackGameError> {
    //     if bet <= 0.0 {
    //         return Err(BlackjackGameError {
    //             message: "Bet must be a positive amount".to_string(),
    //         });
    //         // return Err("Bet must be a positive amount".to_string());
    //     } else if self.balance < 1.5 * bet {
    //         return Err(BlackjackGameError {
    //             message: "Insufficient table balance to payout bet".to_string(),
    //         });
    //     }
    //     player.place_bet(bet)
    // }

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

    // /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
    // /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
    // fn play_option(
    //     &mut self,
    //     player: &mut Player,
    //     options: &HashMap<i32, String>,
    //     option: i32,
    // ) -> Result<(), BlackjackGameError> {
    //     match options[&option].as_str() {
    //         "stand" => Ok(self.stand(player)),
    //         "hit" => Ok(self.hit(player)),
    //         "split" => Ok(self.split(player)),
    //         "double down" => Ok(self.double_down(player)),
    //         _ => Err(BlackjackGameError {
    //             message: format!("{} is not a valid option", option),
    //         }),
    //     }
    // }

    // /// Takes a Player struct `player` and changes its state via its stand method
    // fn stand(&self, player: &mut Player) {
    //     player.stand();
    //     if !player.turn_is_over() {
    //         println!("{}", "-".to_string().repeat(80));
    //         self.dealers_hand.display_hand_without_hole();
    //         println!("\n\n");
    //         player.bj_hand.display_hand();
    //         player.display_balance();
    //     }
    // }

    // /// Takes a Player `player` and changes the state `players`'s hand by dealing another card.
    // /// The function then computes if the player has busted or not and adjusts the bets of the player accordingly
    // fn hit(&mut self, player: &mut Player) {
    //     player.receive_card(self.deck.get_next_card().unwrap());
    //     player.compute_hand_value();

    //     println!("{}", "-".to_string().repeat(80));
    //     self.dealers_hand.display_hand_without_hole();
    //     println!("\n\n");
    //     player.bj_hand.display_hand();
    //     player.display_balance();

    //     if player.busted() {
    //         println!("Busted, you lost the bet");
    //         self.balance += player.lose_bet() as f32;
    //     }
    // }

    // /// Method to implement the logic for doubling down on a bet
    // fn double_down(&mut self, player: &mut Player) {
    //     // Call the double_down() method of the player, and deal them another card
    //     player.double_down();
    //     player.receive_card(self.deck.get_next_card().unwrap());
    //     player.compute_hand_value();

    //     if !player.busted() {
    //         player.stand();
    //     } else {
    //         println!("{}", "-".to_string().repeat(80));
    //         self.dealers_hand.display_hand_without_hole();
    //         println!("\n\n");
    //         player.bj_hand.display_hand();
    //         player.display_balance();
    //         println!("Busted, you lost the bet");
    //         self.balance += player.lose_bet() as f32;
    //     }
    // }

    // /// Method to execute the logic for a player to split
    // fn split(&mut self, player: &mut Player) {
    //     player.split(
    //         self.deck.get_next_card().unwrap(),
    //         self.deck.get_next_card().unwrap(),
    //     );
    //     println!("{}", "-".to_string().repeat(80));
    //     self.dealers_hand.display_hand_without_hole();
    //     println!("\n\n");
    //     player.bj_hand.display_hand();
    //     player.display_balance();
    // }

    // /// Implments the logic that deals the initial cards at the start of a hand, checks if
    // /// dealer has a blackjack and whether or not `player` has a blackjack, and then executes the appropriate logic
    // fn deal_hand(&mut self, player: &mut Player) {
    //     assert!(
    //         !player.bj_hand.bets.is_empty(),
    //         "bet must be placed by the player before proceeding"
    //     );

    //     // Check if deck needs to be shuffled
    //     if self.deck.shuffle_flag {
    //         println!("Shuffling...");
    //         self.deck.shuffle(self.n_shuffles);
    //     }

    //     // Deal cards to player and dealer
    //     player.receive_card(self.deck.get_next_card().unwrap());

    //     self.dealers_hand
    //         .receive_card(self.deck.get_next_card().unwrap());

    //     player.receive_card(self.deck.get_next_card().unwrap());

    //     self.dealers_hand
    //         .receive_card(self.deck.get_next_card().unwrap());

    //     player.compute_hand_value();
    //     self.dealers_hand.compute_hand_value();

    //     // Check if dealer has blackjack or not, then perform the appropriate logic
    //     println!("{:-<80}", "");
    //     if self.dealers_hand.is_blackjack() {
    //         // Display state of table, no need to keep dealers hole card hidden
    //         self.dealers_hand.display_hand();
    //         self.dealers_hand.display_hand_value();
    //         println!("\n\n");
    //         player.bj_hand.display_hand();
    //         player.display_balance();
    //         println!();

    //         // Check if player has blackjack
    //         let mut result_str = String::from("Dealer has blackjack: ");
    //         if player.has_blackjack() {
    //             result_str.push_str("you pushed");
    //             player.balance += player.bj_hand.bets.pop().unwrap() as f32;
    //         } else {
    //             result_str.push_str("you lost the bet");
    //             self.balance += player.bj_hand.bets.pop().unwrap() as f32;
    //         }
    //         println!("{result_str}");
    //     } else {
    //         self.dealers_hand.display_hand_without_hole();
    //         println!("\n\n");
    //         player.bj_hand.display_hand();
    //         player.display_balance();

    //         // Check if player has a blackjack
    //         if player.has_blackjack() {
    //             let winnings = 1.5 * (player.bets()[0] as f32);
    //             self.balance -= winnings;
    //             player.balance += winnings + (player.bj_hand.bets.pop().unwrap() as f32);
    //             println!("You got blackjack, winnings: {:2.2}", winnings);
    //         }
    //     }
    // }

    // /// A method for computing and returning the optimal hand for the dealer at the end of a hand of blackjack.
    // /// The dealers draws cards according to the rules of blackjack, then the optimal hand once a hand with a value of no less than 17 is achieved
    // fn get_dealers_optimal_final_hand(&mut self) -> u8 {
    //     self.dealers_hand.compute_optimal_final_hand(&mut self.deck)
    // }

    // /// This method will complete a hand of blackjack, it will check `player` optimal hand(s) against the dealer and payout bets accordingly
    // /// A call to this method will also reset the state of `player` and the dealer to have empty hands i.e. `player` and dealer will be in a state to play another round
    // fn finish_hand(&mut self, player: &mut Player) {
    //     // Compute players optimal hands, if they have any bets remaining at the table
    //     // if the player has no remaining bets then, just skip to reseting dealer/player
    //     if let Some(players_optimal_hands) = player.get_optimal_hands() {
    //         let dealers_optimal_hand = self.get_dealers_optimal_final_hand();
    //         let mut winnings: f32 = 0.0;
    //         let mut refunded_bets: f32 = 0.0;
    //         let result_messages = if player.bets().len() > 1 {
    //             let mut res = vec![];

    //             for (i, bet) in player.bets().iter().enumerate() {
    //                 if *bet > 0 {
    //                     if dealers_optimal_hand > 21
    //                         || players_optimal_hands[&i] > dealers_optimal_hand
    //                     {
    //                         res.push(format!("You won bet #{}: ${}", i + 1, *bet));
    //                         self.balance -= *bet as f32;
    //                         winnings += *bet as f32;
    //                     } else if players_optimal_hands[&i] == dealers_optimal_hand {
    //                         res.push(format!("You pushed bet #{}: ${}", i + 1, *bet));
    //                         refunded_bets += *bet as f32;
    //                     } else {
    //                         self.balance += *bet as f32;
    //                         res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
    //                     }
    //                 } else {
    //                     res.push(format!("You lost bet #{}: ${}", i + 1, *bet))
    //                 }
    //             }
    //             res
    //         } else {
    //             if dealers_optimal_hand > 21 || players_optimal_hands[&0] > dealers_optimal_hand {
    //                 self.balance -= player.bets()[0] as f32;
    //                 winnings += player.bets()[0] as f32;
    //                 vec![String::from("You won the bet")]
    //             } else if players_optimal_hands[&0] == dealers_optimal_hand {
    //                 refunded_bets += player.bets()[0] as f32;
    //                 vec![String::from("You pushed")]
    //             } else {
    //                 self.balance += player.bets()[0] as f32;
    //                 vec![String::from("You lost the bet")]
    //             }
    //         };

    //         self.display_end_of_hand_state(&player, result_messages, winnings);
    //         player.balance += 2.0 * winnings;
    //         player.balance += refunded_bets;
    //     };

    //     self.dealers_hand.reset();
    //     player.reset();
    // }

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

impl BlackjackTable for BlackjackTableCLI {
    /// Creates a new instance of a BlackjackTableCLI struct
    fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> Self {
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
}
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
