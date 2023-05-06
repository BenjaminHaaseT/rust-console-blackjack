use rand::prelude::*;
use std::fmt::Display;
use std::iter::Iterator;

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
    shuffle_flag_pos: usize,
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
        let shuffle_flag_pos = f32::floor(((n_cards - 1) as f32) * 0.8) as usize;

        Deck {
            cards,
            n_decks,
            shuffle_flag_pos,
        }
    }

    pub fn iter(&self) -> DeckIterator {
        DeckIterator {
            deck: &self,
            card_ptr: 0usize,
            shuffle_flag: false,
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
    }
}

pub struct DeckIterator<'a> {
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

pub struct Player {
    name: String,
    balance: f32,
    hand: Vec<Vec<Card>>,
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

pub struct BlackjackTable {
    deck: Deck,
    balance: f32,
    dealers_hand: Vec<Card>,
    dealers_hand_value: Vec<Vec<u8>>,
    bets: Vec<u8>,
}

//TODO: implement methods for a working console based blackjack game
impl BlackjackTable {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deck_iterator() {
        let deck = Deck::new(1);
        let mut deck_iter = deck.iter();

        for i in 0..deck.cards.len() {
            println!("deck.cards[i] = {}", deck.cards[i]);

            let next_card = deck_iter.next().unwrap();
            println!("next_card = {}", next_card);

            assert_eq!(deck.cards[i].rank, next_card.rank);
            assert_eq!(deck.cards[i].suit, next_card.suit);
        }
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck = Deck::new(6);
        deck.shuffle(6);
        let mut deck_iter = deck.iter();

        for i in 0..deck.cards.len() {
            println!("deck.cards[i] = {}", deck.cards[i]);

            let next_iter_card = deck_iter.next().unwrap();
            println!("next_card = {}", next_iter_card);

            assert_eq!(deck.cards[i].rank, next_iter_card.rank);
            assert_eq!(deck.cards[i].suit, next_iter_card.suit);
        }
    }

    #[test]
    #[should_panic]
    fn test_deck_shuffle_panic() {
        let mut deck = Deck::new(6);
        deck.shuffle(0);
    }
}
