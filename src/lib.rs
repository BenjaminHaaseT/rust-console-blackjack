pub mod console;

use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

// extern mod console;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

// pub trait BlackjackTable {}

// pub trait PlayersBlackjackHand {}

// pub trait DealersBlackjackHand {}

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
    suit: &'static str,
    rank: &'static str,
}

impl Card {
    /// Creates a new card struct given `suit` and `rank`
    pub fn new(suit: &'static str, rank: &'static str) -> Card {
        Card { suit, rank }
    }

    /// Associated method that returns a string represnting what a card facedown looks like on the console
    pub fn display_facedown() -> String {
        String::from("|*|")
    }

    /// Returns the numeric value of the playing card. Note aces default to the value of 1, and consequently,
    /// any logic that treats aces as multiple values i.e. 1 or 11 needs to be handled outside of this struct
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

/// A simple struct that acts as a collection of playing cards of type Card.
pub struct Deck {
    cards: Vec<Rc<Card>>,
    n_decks: u32,
    deck_pos: usize,
    shuffle_flag_pos: usize,
    shuffle_flag: bool,
}

/// A struct to represent a deck of cards, is basically a collection of card structs that implements some specific logic related to a game of blackjack
impl Deck {
    /// An associated function that aids in the building of a deck of cards
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

    /// Creates and returns a new Deck struct
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

pub fn run() -> std::io::Result<()> {
    let mut player = console::PlayerCLI::new(String::from("Rick Sanchez"), 500.0);
    let mut table = console::BlackjackTableCLI::new(500000000.0, 6, 7);
    let mut game = console::BlackjackGameCLI::new(5, player, table);
    game.play()?;

    Ok(())
}
