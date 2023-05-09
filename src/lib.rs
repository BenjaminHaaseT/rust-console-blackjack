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

pub struct BlackjackTable {
    deck: Deck,
    balance: f32,
    dealers_hand: Vec<Rc<Card>>,
    dealers_hand_value: Vec<u8>,
    bets: Vec<u8>,
    n_shuffles: u32,
}

//TODO: implement methods for a working console based blackjack game
impl BlackjackTable {
    pub fn new(starting_balance: f32, n_decks: u32, n_shuffles: u32) -> BlackjackTable {
        let deck = Deck::new(n_decks);

        BlackjackTable {
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

    pub fn display_table_state(&self, player: &Player, display_dealers_hole_card: bool) {
        // Convert dealers hand to vector of strings depending on whether or not we are showing the dealers hole card
        let dealers_formatted_hand = if display_dealers_hole_card {
            self.dealers_hand
                .iter()
                .map(|c| c.as_ref().to_string())
                .collect::<Vec<String>>()
        } else {
            vec![
                Card::display_facedown(),
                self.dealers_hand[1].as_ref().to_string(),
            ]
        }
        .join(" ");

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

        let players_formatted_hand = if players_hand_vec.len() > 1 {
            players_hand_vec.join(" | ")
        } else {
            players_hand_vec.remove(0)
        };

        println!("{}", format!("-").repeat(80));
        println!("Dealer: {}", dealers_formatted_hand);
        println!("\n\n");
        println!("You: {}", players_formatted_hand);
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
        BlackjackTable::compute_hand_value(&player.hand[0], &mut player.hand_value[0]);
        BlackjackTable::compute_hand_value(&self.dealers_hand, &mut self.dealers_hand_value);

        // Display output of deal, then check for dealers blackjack
        self.display_table_state(&player, false);
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
    fn test_blackjack_table_dealing_initial_hand() {
        let mut table = BlackjackTable::new(500000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);
    }
}
