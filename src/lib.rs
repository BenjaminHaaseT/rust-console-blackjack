pub mod game_components;

use crate::game_components::BlackjackHand;
use rand::prelude::*;
use std::fmt::Display;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

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

// TODO: Implement other playing options such as split and double down
// Also implement way to query current hand value to find available options for the player
impl PlayersBlackjackHand {
    fn place_bet(&mut self, bet: u32) -> u32 {
        self.bets.push(bet);
        self.bets_str[self.hand_idx] = bet.to_string();
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
            formatted_bet_str.push(format!("{:<width$}$", self.bets_str[i]));
        }

        let (formatted_hand_str, formatted_hand_values_str, formatted_bet_str) = (
            self.hand_values_str.join(" | "),
            formatted_hand_values_str.join(" | "),
            formatted_bet_str.join(" | "),
        );

        let bet_tag = if self.bets.len() > 1 {
            "Bets:".to_string()
        } else {
            "Bet:".to_string()
        };

        println!("{:<9}{}", "You:", formatted_hand_str);
        println!("{:<9}{}", "Value:", formatted_hand_values_str);
        println!("{:<9}{}", bet_tag, formatted_bet_str);
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

    fn is_blackjack(&self) -> bool {
        self.hand_idx == 0
            && self.hand[0].len() == 2
            && ((self.hand[0][0].rank == "A" && self.hand[0][1].get_card_value() == 10)
                || (self.hand[0][0].get_card_value() == 10 && self.hand[0][1].rank == "A"))
    }

    fn busted(&self) -> bool {
        if self.hand_values[self.hand_idx].len() == 2 {
            self.hand_values[self.hand_idx][0] > 21 && self.hand_values[self.hand_idx][1] > 21
        } else {
            self.hand_values[self.hand_idx][0] > 21
        }
    }
}

pub struct Player {
    name: String,
    balance: f32,
    hand: Vec<Vec<Rc<Card>>>,
    hand_value: Vec<Vec<u8>>,
}

impl Player {
    pub fn new(name: String, balance: f32) -> Player {
        Player {
            name,
            balance,
            hand: vec![vec![]],
            hand_value: vec![vec![]],
        }
    }

    pub fn place_bet(&mut self, bet: f32) -> f32 {
        assert!(bet <= self.balance);
        self.balance -= bet;
        bet
    }
}

pub struct BlackjackTableCLI {
    deck: Deck,
    balance: f32,
    dealers_hand: Vec<Rc<Card>>,
    dealers_hand_value: Vec<u8>,
    bets: Vec<u32>,
    n_shuffles: u32,
}

//TODO: implement methods for a working console based blackjack game
impl BlackjackTableCLI {
    pub fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> BlackjackTableCLI {
        let deck = Deck::new(n_decks);

        BlackjackTableCLI {
            deck,
            balance: starting_balance,
            dealers_hand: vec![],
            dealers_hand_value: vec![],
            bets: vec![],
            n_shuffles,
        }
    }

    pub fn compute_hand_value(hand: &Vec<Rc<Card>>, hand_value: &mut Vec<u8>) {
        let val = hand.iter().map(|c| c.as_ref().get_card_value()).sum::<u8>();
        hand_value.push(val);
        if (hand[0].rank == "A" && hand[1].get_card_value() != 10)
            || (hand[0].get_card_value() != 0 && hand[1].rank == "A")
        {
            hand_value.push(hand_value[0] + 10);
        }
    }

    pub fn check_blackjack(hand: &Vec<Rc<Card>>) -> bool {
        assert!(
            hand.len() == 2,
            "cannot check for a blackjack with more than 2 cards"
        );
        (hand[0].rank == "A" && hand[1].get_card_value() == 10)
            || (hand[0].get_card_value() == 10 && hand[1].rank == "A")
    }

    pub fn display_table_state(&self, player: &Player, display_dealers_hole_card: bool) {
        // Convert dealers hand to vector of strings depending on whether or not we are showing the dealers hole card
        let dealers_formatted_hand =
            vec![Card::display_facedown(), self.dealers_hand[1].to_string()].join(" ");

        // Compute players hand value, for each hand the player has a bet on
        let mut players_hand_vec = player
            .hand
            .iter()
            .map(|v| {
                v.iter()
                    .map(|c| c.as_ref().to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();

        // Do the same for the values of each hand
        let mut players_hand_value_vec = player
            .hand_value
            .iter()
            .map(|v| {
                if v.len() == 1 {
                    v[0].to_string()
                } else {
                    format!("{}/{}", v[0], v[1])
                }
            })
            .collect::<Vec<String>>();

        // Now get formatted hand string, values, and bets for each of the players hand
        let (players_formatted_hand, players_formatted_hand_values, players_formatted_bets) =
            if players_hand_vec.len() > 1 {
                let mut hand_str = String::new();
                let mut hand_value_str = String::new();
                let mut bet_str = String::new();

                hand_str.push_str(players_hand_vec[0].as_str());
                let width = hand_str.len();
                hand_value_str.push_str(format!("{:<width$}", players_hand_value_vec[0]).as_str());
                bet_str.push_str(format!("{:<width$}$", self.bets[0]).as_str());

                for i in 1..players_hand_vec.len() {
                    let width = hand_str.len();
                    hand_str.push_str(format!(" | {}", players_hand_vec[i]).as_str());
                    hand_value_str.push_str(
                        format!("{:<width$} | {}", "", players_hand_value_vec[i]).as_str(),
                    );
                    bet_str.push_str(format!("{:<width$} | {}$", "", self.bets[i]).as_str());
                }

                (hand_str, hand_value_str, bet_str)
            } else {
                (
                    players_hand_vec.remove(0),
                    players_hand_value_vec.remove(0),
                    self.bets[0].to_string(),
                )
            };

        println!("{}", format!("-").repeat(80));
        println!("{}", format!("{:<9}{}", "Dealer:", dealers_formatted_hand));
        println!("\n\n");
        println!("{}", format!("{:<9}{}", "You:", players_formatted_hand));
        println!(
            "{}",
            format!("{:<9}{}", "Value:", players_formatted_hand_values)
        );
        let bet_tag = if self.bets.len() == 1 {
            "Bet:".to_string()
        } else {
            "Bets:".to_string()
        };
        println!("{}", format!("{:<9}{}", bet_tag, players_formatted_bets));
        println!("{}", format!("{:<9}{}", "Balance:", player.balance));
    }

    pub fn place_bet(&mut self, player: &mut Player, bet: u32) {
        assert!(bet > 0, "cannot place a bet of 0$");
        self.bets.push(player.place_bet(bet as f32) as u32);
    }

    pub fn deal_hand(&mut self, player: &mut Player) {
        assert!(
            !self.bets.is_empty(),
            "unable to deal hand unless the player has placed a bet"
        );

        if self.deck.shuffle_flag {
            println!("Shuffling deck...");
            self.deck.shuffle(self.n_shuffles);
        }

        // Deal cards to player and dealer in the traditional order
        player.hand[0].push(Rc::clone(
            &self
                .deck
                .get_next_card()
                .expect("deck should be shuffled and posses enough cards"),
        ));

        self.dealers_hand.push(Rc::clone(
            &self
                .deck
                .get_next_card()
                .expect("deck should be shuffled and posses enough cards"),
        ));

        player.hand[0].push(Rc::clone(
            &self
                .deck
                .get_next_card()
                .expect("deck should be shuffled and posses enough cards"),
        ));

        self.dealers_hand.push(Rc::clone(
            &self
                .deck
                .get_next_card()
                .expect("deck should be shuffled and posses enough cards"),
        ));

        // Compute value of dealer/players hand, and update accordlingly
        BlackjackTableCLI::compute_hand_value(&player.hand[0], &mut player.hand_value[0]);
        BlackjackTableCLI::compute_hand_value(&self.dealers_hand, &mut self.dealers_hand_value);

        //TODO: Implement checking for dealers blackjack
        // // Check for dealers blackjack
        // if BlackjackTable::check_blackjack(&self.dealers_hand) {}

        // // Display output of deal
        // self.display_table_state(&player, false);
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

        table.place_bet(&mut player, 25);

        println!("{:?}", table.bets);
        assert_eq!(table.bets, vec![25u32]);

        println!("{}", player.balance);
        assert_eq!(player.balance, 475.0);
    }

    #[test]
    fn test_blackjack_table_dealing_initial_hand() {
        let mut table = BlackjackTableCLI::new(500000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);

        table.place_bet(&mut player, 25);
        table.deal_hand(&mut player);

        // Show the hand value of dealer and player
        // println!("{:?}", player.hand_value);
        // println!("{:?}", table.dealers_hand_value);

        // // Display the hands as well to check cards are getting added;
        // println!()

        assert_eq!(player.hand[0].len(), 2usize);
        assert_eq!(table.dealers_hand.len(), 2usize);
    }
}
