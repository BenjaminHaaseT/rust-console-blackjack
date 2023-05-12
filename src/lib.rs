use rand::prelude::*;
use std::fmt::Display;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

pub trait BlackjackHand {
    fn receive_card(&mut self, card: Rc<Card>);

    fn display_hand(&self);

    fn compute_hand_value(&mut self);

    fn is_blackjack(&self) -> bool;

    fn busted(&self) -> bool;
}

pub trait BlackjackTable {}

#[derive(PartialEq, Eq, Debug)]
pub struct Card {
    suit: &'static str,
    rank: &'static str,
}

impl Card {
    pub fn new(suit: &'static str, rank: &'static str) -> Card {
        Card { suit, rank }
    }

    pub fn display_facedown() -> String {
        String::from("|*|")
    }

    pub fn get_card_value(&self) -> u8 {
        match u8::from_str(self.rank) {
            Err(_) if self.rank == "A" => 1,
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

pub struct Deck {
    cards: Vec<Rc<Card>>,
    n_decks: u32,
    deck_pos: usize,
    shuffle_flag_pos: usize,
    shuffle_flag: bool,
}

impl Deck {
    fn build_card_deck(n_decks: u32) -> Vec<Rc<Card>> {
        let mut cards = vec![];
        for _i in 0..n_decks {
            for suit in SUITS {
                for rank in RANKS {
                    cards.push(Rc::new(Card::new(suit, rank)));
                }
            }
        }
        cards
    }

    pub fn new(n_decks: u32) -> Deck {
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

//TODO: implement Hand trait and player/dealers hand structs to encapsulate some functionality out of blackjack table struct
pub struct PlayersBlackjackHand {
    hand: Vec<Vec<Rc<Card>>>,
    hand_values: Vec<Vec<u8>>,
    bets: Vec<u32>,
    hand_str: Vec<String>,
    hand_values_str: Vec<String>,
    bets_str: Vec<String>,
    hand_idx: usize,
}

// TODO: Implement other playing options such as split and double down and constructor
impl PlayersBlackjackHand {
    fn new() -> Self {
        let hand = vec![vec![]];
        let hand_values = vec![vec![]];
        let bets = vec![];
        let hand_str = vec![String::new()];
        let hand_values_str = vec![String::new()];
        let bets_str = vec![];
        let hand_idx = 0usize;

        PlayersBlackjackHand {
            hand,
            hand_values,
            bets,
            hand_str,
            hand_values_str,
            bets_str,
            hand_idx,
        }
    }

    fn place_bet(&mut self, bet: u32) -> u32 {
        self.bets.push(bet);
        self.bets_str.push(bet.to_string());
        bet
    }
}

impl BlackjackHand for PlayersBlackjackHand {
    fn display_hand(&self) {
        let mut formatted_hand_values_str = vec![];
        let mut formatted_bet_str = vec![];
        for (i, h) in self.hand_values_str.iter().enumerate() {
            let width = h.len();
            formatted_hand_values_str.push(format!("{:<width$}", self.hand_values_str[i]));
            formatted_bet_str.push(format!("${:<width$}", self.bets_str[i]));
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

    /// Receive a new card, add it to the players hand and update the string representing
    /// the current hand as well
    fn receive_card(&mut self, card: Rc<Card>) {
        self.hand[self.hand_idx].push(Rc::clone(&card));
        // Now update string representing hand
        if self.hand_str[self.hand_idx].is_empty() {
            self.hand_str[self.hand_idx].push_str(card.to_string().as_str());
        } else {
            self.hand_str[self.hand_idx].push_str(format!(" {}", card.to_string()).as_str());
        }
    }

    /// Compute the value of the players current hand and update the formatted string
    /// representing the value of the current hand as well
    fn compute_hand_value(&mut self) {
        if self.hand[self.hand_idx].len() == 2 {
            self.hand_values[self.hand_idx].push(
                self.hand[self.hand_idx]
                    .iter()
                    .map(|c| c.get_card_value())
                    .sum::<u8>(),
            );

            // Need to check if we have more than one possible value for the given hand
            if self.hand[self.hand_idx][0].rank == "A" || self.hand[self.hand_idx][0].rank == "A" {
                let alternative_hand_val = self.hand_values[self.hand_idx]
                    .last()
                    .expect("hand should not be empty")
                    + 10;

                self.hand_values[self.hand_idx].push(alternative_hand_val);
            }
        } else {
            let new_card_val = self.hand[self.hand_idx]
                .last()
                .expect("hand should not be empty")
                .get_card_value();
            self.hand_values[self.hand_idx][0] += new_card_val;
            if self.hand_values[self.hand_idx].len() == 2 {
                self.hand_values[self.hand_idx][1] += new_card_val;
            }
        }

        // Now update the string that represents the value
        self.hand_values_str[self.hand_idx] = if self.hand_values[self.hand_idx].len() == 2 {
            format!(
                "{}/{}",
                self.hand_values[self.hand_idx][0], self.hand_values[self.hand_idx][1]
            )
        } else {
            self.hand_values[self.hand_idx][0].to_string()
        };
    }

    /// Checks if the players current hand is a blackjack, ensures it must be a natural blackjack by
    /// checking that `self.hand_idx` is equal to zero i.e. it is the first hand dealt to the player, not
    /// a hand dealt after splitting
    fn is_blackjack(&self) -> bool {
        self.hand_idx == 0
            && self.hand[0].len() == 2
            && ((self.hand[0][0].rank == "A" && self.hand[0][1].get_card_value() == 10)
                || (self.hand[0][0].get_card_value() == 10 && self.hand[0][1].rank == "A"))
    }

    /// Checks whether the current hand has busted or not
    fn busted(&self) -> bool {
        if self.hand_values[self.hand_idx].len() == 2 {
            self.hand_values[self.hand_idx][0] > 21 && self.hand_values[self.hand_idx][1] > 21
        } else {
            self.hand_values[self.hand_idx][0] > 21
        }
    }
}

pub struct DealersBlackjackHand {
    hand: Vec<Rc<Card>>,
    hand_value: Vec<u8>,
    hand_str: String,
    hand_value_str: String,
}

impl DealersBlackjackHand {
    /// Creates a new `DealersBlackjackHand` struct
    fn new() -> Self {
        let hand = vec![];
        let hand_value = vec![];
        let hand_str = String::new();
        let hand_value_str = String::new();

        DealersBlackjackHand {
            hand,
            hand_value,
            hand_str,
            hand_value_str,
        }
    }

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
}

impl BlackjackHand for DealersBlackjackHand {
    /// Checks whether the dealers hand has busted or not
    fn busted(&self) -> bool {
        if self.hand_value.len() == 2 {
            self.hand_value[0] > 21 && self.hand_value[1] > 21
        } else {
            self.hand_value[0] > 21
        }
    }

    /// Checks whether or not the dealers hand has busted
    fn is_blackjack(&self) -> bool {
        self.hand.len() == 2
            && ((self.hand[0].rank == "A" && self.hand[1].get_card_value() == 10)
                || (self.hand[0].get_card_value() == 10 && self.hand[1].rank == "A"))
    }

    /// Computes the dealers hand value, and updates the string that represents that value for display
    /// via standard output ie console
    fn compute_hand_value(&mut self) {
        if self.hand.len() == 2 {
            self.hand_value
                .push(self.hand.iter().map(|c| c.get_card_value()).sum());

            // We need to check if there is an alternative hand value possible
            if self.hand[0].rank == "A" || self.hand[1].rank == "A" {
                let alternative_hand_val = self.hand_value[0] + 10;
                self.hand_value.push(alternative_hand_val);
            }
        } else {
            let new_card_val = self
                .hand
                .last()
                .expect("hand should not be empty")
                .get_card_value();
            self.hand_value[0] += new_card_val;
            if self.hand_value.len() == 2 {
                self.hand_value[1] += new_card_val;
            }
        }

        // Update the string that will be displayed
        self.hand_value_str = if self.hand_value.len() == 2 {
            format!("{}/{}", self.hand_value[0], self.hand_value[1])
        } else {
            self.hand_value[0].to_string()
        }
    }

    /// Print the value of dealers hand to the console, formatted in a nice way
    fn display_hand(&self) {
        println!("{:<10}{}", "Dealer:", self.hand_str);
    }

    /// Receive a new card, `card` which will be pushed to dealers hand
    /// function will also update the value of `self.hand_str`, the string representing the cards to be printed to the console
    fn receive_card(&mut self, card: Rc<Card>) {
        self.hand.push(Rc::clone(&card));
        if self.hand_str.is_empty() {
            self.hand_str.push_str(card.to_string().as_str());
        } else {
            self.hand_str
                .push_str(format!(" {}", card.to_string()).as_str());
        }
    }
}
pub struct Player {
    name: String,
    balance: f32,
    bj_hand: PlayersBlackjackHand,
}

impl Player {
    pub fn new(name: String, balance: f32) -> Player {
        Player {
            name,
            balance,
            bj_hand: PlayersBlackjackHand::new(),
        }
    }

    pub fn place_bet(&mut self, bet: f32) {
        assert!(bet <= self.balance);
        self.balance -= bet;
        self.bj_hand.place_bet(bet as u32);
    }

    pub fn display_balance(&self) {
        println!("{:<10}${}", "Balance:", self.balance)
    }
}

pub struct BlackjackTableCLI {
    deck: Deck,
    balance: f32,
    dealers_hand: DealersBlackjackHand,
    n_shuffles: u32,
}

//TODO: implement methods for a working console based blackjack game
impl BlackjackTableCLI {
    pub fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> BlackjackTableCLI {
        let deck = Deck::new(n_decks);

        BlackjackTableCLI {
            deck,
            balance: starting_balance,
            dealers_hand: DealersBlackjackHand::new(),
            n_shuffles,
        }
    }

    // pub fn compute_hand_value(hand: &Vec<Rc<Card>>, hand_value: &mut Vec<u8>) {
    //     let val = hand.iter().map(|c| c.as_ref().get_card_value()).sum::<u8>();
    //     hand_value.push(val);
    //     if (hand[0].rank == "A" && hand[1].get_card_value() != 10)
    //         || (hand[0].get_card_value() != 0 && hand[1].rank == "A")
    //     {
    //         hand_value.push(hand_value[0] + 10);
    //     }
    // }

    // pub fn check_blackjack(hand: &Vec<Rc<Card>>) -> bool {
    //     assert!(
    //         hand.len() == 2,
    //         "cannot check for a blackjack with more than 2 cards"
    //     );
    //     (hand[0].rank == "A" && hand[1].get_card_value() == 10)
    //         || (hand[0].get_card_value() == 10 && hand[1].rank == "A")
    // }

    // pub fn display_table_state(&self, player: &Player, display_dealers_hole_card: bool) {
    //     // Convert dealers hand to vector of strings depending on whether or not we are showing the dealers hole card
    //     let dealers_formatted_hand =
    //         vec![Card::display_facedown(), self.dealers_hand[1].to_string()].join(" ");

    //     // Compute players hand value, for each hand the player has a bet on
    //     let mut players_hand_vec = player
    //         .hand
    //         .iter()
    //         .map(|v| {
    //             v.iter()
    //                 .map(|c| c.as_ref().to_string())
    //                 .collect::<Vec<String>>()
    //                 .join(" ")
    //         })
    //         .collect::<Vec<String>>();

    //     // Do the same for the values of each hand
    //     let mut players_hand_value_vec = player
    //         .hand_value
    //         .iter()
    //         .map(|v| {
    //             if v.len() == 1 {
    //                 v[0].to_string()
    //             } else {
    //                 format!("{}/{}", v[0], v[1])
    //             }
    //         })
    //         .collect::<Vec<String>>();

    //     // Now get formatted hand string, values, and bets for each of the players hand
    //     let (players_formatted_hand, players_formatted_hand_values, players_formatted_bets) =
    //         if players_hand_vec.len() > 1 {
    //             let mut hand_str = String::new();
    //             let mut hand_value_str = String::new();
    //             let mut bet_str = String::new();

    //             hand_str.push_str(players_hand_vec[0].as_str());
    //             let width = hand_str.len();
    //             hand_value_str.push_str(format!("{:<width$}", players_hand_value_vec[0]).as_str());
    //             bet_str.push_str(format!("{:<width$}$", self.bets[0]).as_str());

    //             for i in 1..players_hand_vec.len() {
    //                 let width = hand_str.len();
    //                 hand_str.push_str(format!(" | {}", players_hand_vec[i]).as_str());
    //                 hand_value_str.push_str(
    //                     format!("{:<width$} | {}", "", players_hand_value_vec[i]).as_str(),
    //                 );
    //                 bet_str.push_str(format!("{:<width$} | {}$", "", self.bets[i]).as_str());
    //             }

    //             (hand_str, hand_value_str, bet_str)
    //         } else {
    //             (
    //                 players_hand_vec.remove(0),
    //                 players_hand_value_vec.remove(0),
    //                 self.bets[0].to_string(),
    //             )
    //         };

    //     println!("{}", format!("-").repeat(80));
    //     println!("{}", format!("{:<9}{}", "Dealer:", dealers_formatted_hand));
    //     println!("\n\n");
    //     println!("{}", format!("{:<9}{}", "You:", players_formatted_hand));
    //     println!(
    //         "{}",
    //         format!("{:<9}{}", "Value:", players_formatted_hand_values)
    //     );
    //     let bet_tag = if self.bets.len() == 1 {
    //         "Bet:".to_string()
    //     } else {
    //         "Bets:".to_string()
    //     };
    //     println!("{}", format!("{:<9}{}", bet_tag, players_formatted_bets));
    //     println!("{}", format!("{:<9}{}", "Balance:", player.balance));
    // }

    /// takes a Player struct `player` and places a bet
    pub fn place_bet(&self, player: &mut Player, bet: f32) {
        assert!(bet > 0.0, "cannot place a bet of 0$");
        assert!(self.balance >= 1.5 * bet, "insufficient funds at the table");
        player.place_bet(bet);
    }

    /// Implments the logic that deals the initial cards at the start of a hand, checks if
    /// dealer has a blackjack and whether or not `player` has a blackjack and executes the appropriate logic
    pub fn deal_hand(&mut self, player: &mut Player) {
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
        player
            .bj_hand
            .receive_card(self.deck.get_next_card().unwrap());

        self.dealers_hand
            .receive_card(self.deck.get_next_card().unwrap());
        player
            .bj_hand
            .receive_card(self.deck.get_next_card().unwrap());

        self.dealers_hand
            .receive_card(self.deck.get_next_card().unwrap());

        player.bj_hand.compute_hand_value();
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
            if player.bj_hand.is_blackjack() {
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
            if player.bj_hand.is_blackjack() {
                let winnings = 1.5 * (player.bj_hand.bets[0] as f32);
                self.balance -= winnings;
                player.balance += winnings + (player.bj_hand.bets.pop().unwrap() as f32);

                println!("You won the bet, winnings: {:2.2}", winnings);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_deck_shuffle_panic() {
        let mut deck = Deck::new(6);
        deck.shuffle(0);
    }

    #[test]
    fn test_placing_a_bet() {
        let mut table = BlackjackTableCLI::new(5000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);

        table.place_bet(&mut player, 25.0);

        println!("{:?}", player.bj_hand.bets);
        assert_eq!(player.bj_hand.bets, vec![25u32]);

        println!("{}", player.balance);
        assert_eq!(player.balance, 475.0);
    }

    #[test]
    fn test_blackjack_table_dealing_initial_hand() {
        let mut table = BlackjackTableCLI::new(500000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);

        table.place_bet(&mut player, 25.0);
        table.deal_hand(&mut player);

        // assert_eq!(player.hand.hand[0].len(), 2usize);
        // assert_eq!(table.dealers_hand.hand.len(), 2usize);
    }
}
