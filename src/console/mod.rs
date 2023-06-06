pub mod player;
pub mod table;

use crate::console::player::ConsolePlayer;
use crate::console::table::ConsoleBlackjackTable;
use crate::BlackjackTable;
use std::str::FromStr;

/// A struct for implementing the control flow logic/error checking of a blackjack game played via the console
pub struct ConsoleBlackjackGame {
    table: ConsoleBlackjackTable,
    player: ConsolePlayer,
    minimum_bet: u32,
}

impl ConsoleBlackjackGame {
    /// Returns a new BlackjackGameCLI
    pub fn new(
        minimum_bet: u32,
        player: ConsolePlayer,
        table: ConsoleBlackjackTable,
    ) -> ConsoleBlackjackGame {
        ConsoleBlackjackGame {
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
