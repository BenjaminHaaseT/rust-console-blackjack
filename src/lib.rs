use std::fmt::Display;
use std::iter::Iterator;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

pub struct Card {
    suit: &'static str,
    rank: &'static str,
}

impl Card {
    pub fn new(suit: &'static str, rank: &'static str) -> Card {
        Card { suit, rank }
    }

    pub fn print_facedown() -> String {
        String::from("|*|")
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

pub struct Deck {
    cards: Vec<Card>,
    n_decks: u32,
    n_cards: usize,
    card_ptr: usize,
    shuffle_flag_pos: usize,
    shuffle_flag: bool,
}

impl Deck {
    fn build_card_deck(n_decks: u32) -> Vec<Card> {
        let mut cards = vec![];
        for _i in 0..n_decks {
            for suit in SUITS {
                for rank in RANKS {
                    cards.push(Card::new(suit, rank));
                }
            }
        }
        cards
    }

    pub fn new(n_decks: u32) -> Deck {
        assert!(n_decks > 0, "Cannot have a deck with zero cards");
        let cards = Self::build_card_deck(n_decks);
        let n_cards = cards.len();

        // TODO: refactor how Deck/DeckIterator interact i.e shift card index out of deck class
        let card_ptr = 0;
        let shuffle_flag_pos = f32::floor(((n_cards - 1) as f32) * 0.8) as usize;
        let shuffle_flag = true;

        Deck {
            cards,
            n_decks,
            n_cards,
            card_ptr,
            shuffle_flag_pos,
            shuffle_flag,
        }
    }
}

struct DeckIterator<'a> {
    deck: &'a Deck,
    card_ptr: usize,
    shuffle_flag: bool,
}

impl<'a> Iterator for DeckIterator<'a> {
    type Item = &'a Card;
    fn next(&mut self) -> Option<Self::Item> {
        if self.card_ptr == self.deck.shuffle_flag_pos {
            self.shuffle_flag = true;
        }
        if self.card_ptr < self.deck.cards.len() {
            let res = Some(&self.deck.cards[self.card_ptr]);
            self.card_ptr += 1;
            return res;
        }

        None
    }
}
