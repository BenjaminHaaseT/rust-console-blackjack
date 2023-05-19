use rand::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Iterator;
use std::rc::Rc;
use std::str::FromStr;

const SUITS: [&'static str; 4] = ["C", "D", "H", "S"];
const RANKS: [&'static str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

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

pub struct PlayersBlackjackHand {
    hand: Vec<Vec<Rc<Card>>>,
    hand_values: Vec<Vec<u8>>,
    bets: Vec<u32>,
    hand_str: Vec<String>,
    hand_values_str: Vec<String>,
    bets_str: Vec<String>,
    hand_idx: usize,
}

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

    /// Sets the bet of the current hand to 0 and returns the value of the current bet.
    fn lose_bet(&mut self, hand_idx: usize) -> u32 {
        let res = self.bets[hand_idx];
        self.bets[hand_idx] = 0;
        res
    }

    /// Simple function to check whether or not the current hand i.e. the hand at index `hand_idx` can split.
    /// The function will panice if `hand_idx` is not a valid index or the hand vector is empty.
    fn can_split(&self, hand_idx: usize) -> bool {
        self.hand[hand_idx].len() == 2 && self.hand[hand_idx][0].rank == self.hand[hand_idx][1].rank
    }

    /// Simple fucntion to check whether the current hand i.e. the hand at index `hand_idx` can double down.
    /// The function will panic if `hand_idx` is not a valid index, or hand_values vector is empty.
    fn can_double_down(&self, hand_idx: usize) -> bool {
        if self.hand_values[hand_idx].len() == 2 {
            hand_idx == 0
                && ((self.hand_values[hand_idx][0] == 9
                    || self.hand_values[hand_idx][0] == 10
                    || self.hand_values[hand_idx][0] == 11)
                    || (self.hand_values[hand_idx][1] == 9
                        || self.hand_values[hand_idx][1] == 10
                        || self.hand_values[hand_idx][1] == 11))
        } else {
            hand_idx == 0
                && (self.hand_values[hand_idx][0] == 9
                    || self.hand_values[hand_idx][0] == 10
                    || self.hand_values[hand_idx][0] == 11)
        }
    }

    /// Displays the players current blackjack hand in the console printed in a nice looking format
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
    fn receive_card(&mut self, card: Rc<Card>, hand_idx: usize) {
        self.hand[hand_idx].push(Rc::clone(&card));
        // Now update string representing hand
        if self.hand_str[hand_idx].is_empty() {
            self.hand_str[hand_idx].push_str(card.to_string().as_str());
        } else {
            self.hand_str[hand_idx].push_str(format!(" {}", card.to_string()).as_str());
        }
    }

    /// Compute the value of the players current hand and update the formatted string
    /// representing the value of the current hand as well
    fn compute_hand_value(&mut self, hand_idx: usize) {
        if self.hand[hand_idx].len() == 2 {
            self.hand_values[hand_idx].push(
                self.hand[hand_idx]
                    .iter()
                    .map(|c| c.get_card_value())
                    .sum::<u8>(),
            );

            // Need to check if we have more than one possible value for the given hand
            if self.hand[hand_idx][0].rank == "A" || self.hand[hand_idx][0].rank == "A" {
                let alternative_hand_val = self.hand_values[hand_idx]
                    .last()
                    .expect("hand should not be empty")
                    + 10;

                self.hand_values[hand_idx].push(alternative_hand_val);
            }
        } else {
            let new_card_val = self.hand[hand_idx]
                .last()
                .expect("hand should not be empty")
                .get_card_value();
            self.hand_values[hand_idx][0] += new_card_val;
            if self.hand_values[hand_idx].len() == 2 {
                self.hand_values[hand_idx][1] += new_card_val;
            }
        }

        // Now update the string that represents the value
        self.hand_values_str[hand_idx] = if self.hand_values[hand_idx].len() == 2 {
            format!(
                "{}/{}",
                self.hand_values[hand_idx][0], self.hand_values[hand_idx][1]
            )
        } else {
            self.hand_values[hand_idx][0].to_string()
        };
    }

    /// Checks if the players current hand is a blackjack, ensures it must be a natural blackjack by
    /// checking that `self.hand_idx` is equal to zero i.e. it is the first hand dealt to the player, not
    /// a hand dealt after splitting
    fn is_blackjack(&self, hand_idx: usize) -> bool {
        hand_idx == 0
            && self.hand[0].len() == 2
            && ((self.hand[0][0].rank == "A" && self.hand[0][1].get_card_value() == 10)
                || (self.hand[0][0].get_card_value() == 10 && self.hand[0][1].rank == "A"))
    }

    /// Checks whether the current hand has busted or not
    fn busted(&self, hand_idx: usize) -> bool {
        if self.hand_values[hand_idx].len() == 2 {
            self.hand_values[hand_idx][0] > 21 && self.hand_values[hand_idx][1] > 21
        } else {
            self.hand_values[hand_idx][0] > 21
        }
    }
}

pub struct Player {
    name: String,
    balance: f32,
    bj_hand: PlayersBlackjackHand,
    hand_idx: usize,
}

// TODO: Implement a method to deal with updating the players balance
impl Player {
    pub fn new(name: String, balance: f32) -> Player {
        Player {
            name,
            balance,
            bj_hand: PlayersBlackjackHand::new(),
            hand_idx: 0usize,
        }
    }

    pub fn stand(&mut self) {
        self.hand_idx += 1;
    }

    pub fn turn_is_over(&self) -> bool {
        self.hand_idx == self.bj_hand.hand.len()
    }

    pub fn place_bet(&mut self, bet: f32) {
        assert!(bet <= self.balance);
        self.balance -= bet;
        self.bj_hand.place_bet(bet as u32);
    }

    pub fn display_balance(&self) {
        println!("{:<10}${}", "Balance:", self.balance)
    }

    /// Returns the value of the current bet and resets its value to 0 for post processing
    pub fn lose_bet(&mut self) -> u32 {
        self.bj_hand.lose_bet(self.hand_idx)
    }

    /// Queries the players hand struct to see what the valid options are for the player to takes
    /// function will panic if the players current hand has busted or the player has not placed any bets
    pub fn get_playing_options(&self) -> HashMap<i32, String> {
        assert!(!self.bj_hand.busted(self.hand_idx), "hand should be over");
        assert!(
            !self.bj_hand.bets.is_empty(),
            "player should have placed a bet"
        );
        let mut playing_options = HashMap::new();
        playing_options.insert(1, "stand".to_string());
        playing_options.insert(2, "hit".to_string());
        let mut playing_option = 3;

        if self.bj_hand.can_split(self.hand_idx)
            && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
            && self.bj_hand.hand.len() < 4
        {
            playing_options.insert(playing_option, "split".to_string());
            playing_option += 1;
        }

        if self.bj_hand.can_double_down(self.hand_idx)
            && self.balance >= (self.bj_hand.bets[self.hand_idx] as f32)
            && self.hand_idx == 0
        {
            playing_options.insert(playing_option, "double down".to_string());
        }

        playing_options
    }

    pub fn receive_card(&mut self, card: Rc<Card>) {
        self.bj_hand.receive_card(card, self.hand_idx);
    }

    pub fn has_blackjack(&self) -> bool {
        self.bj_hand.is_blackjack(self.hand_idx)
    }

    pub fn busted(&self) -> bool {
        self.bj_hand.busted(self.hand_idx)
    }

    pub fn compute_hand_value(&mut self) {
        self.bj_hand.compute_hand_value(self.hand_idx);
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

struct BlackjackTableCLI {
    deck: Deck,
    balance: f32,
    dealers_hand: DealersBlackjackHand,
    n_shuffles: u32,
}

//TODO: implement method for taking in an option and playing that option, start with the first two simple ones i.e. stand and hit, then implment logic needed
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

    /// Takes a Player struct, `player` and places a bet
    pub fn place_bet(&self, player: &mut Player, bet: f32) {
        assert!(bet > 0.0, "cannot place a bet of 0$");
        assert!(self.balance >= 1.5 * bet, "insufficient funds at the table");
        player.place_bet(bet);
    }

    /// Takes `player` and queries the `players` hand to find valid playing options for the player, and then,
    /// display playing options on the console.
    pub fn display_playing_options(&self, player: &Player) {
        println!();
        println!("Your options: ");
        let options = player.get_playing_options();
        for i in 1..=(options.len() as i32) {
            println!("\t {}: {}", i, options.get(&i).unwrap());
        }

        // // Get the option from the console
        // let option = std::io::stdin();

        // // Then play the option, TODO: Fix this functionality
        // self.play_option(&mut player, options, option);
    }

    /// Takes a Player `player`, HashMap `options` of playing options and an i32 `option`, then selects and calls the method
    /// that implements the correct logic for the given option. The method pancis if `option` is not in the HashMap `options`
    pub fn play_option(
        &mut self,
        player: &mut Player,
        options: HashMap<i32, String>,
        option: i32,
    ) -> Result<(), String> {
        match options[&option].as_str() {
            "stand" => Ok(self.stand(player)),
            "hit" => Ok(self.hit(player)),
            // "split" => Ok(self.split()),
            // "double down" => Ok(self.double_down()),     TODO: Implement these methods
            _ => Err(format!(
                "sorry but {} is not a valid option for the given options",
                option
            )),
        }
    }

    /// Takes a Player struct `player` and changes its state via its stand method
    pub fn stand(&self, player: &mut Player) {
        player.stand();
    }

    /// Takes a Player `player` and changes the state `players`'s hand by dealing another card.
    /// The function then computes if the player has busted or not and adjusts the bets of the player accordingly
    pub fn hit(&mut self, player: &mut Player) {
        player.receive_card(self.deck.get_next_card().unwrap());
        println!("{}", "-".to_string().repeat(80));
        self.dealers_hand.display_hand_without_hole();
        println!("\n\n");
        player.bj_hand.display_hand();
        player.display_balance();

        if player.busted() {
            println!("Busted, you lose the bet");
            self.balance += player.lose_bet() as f32;
        }
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

    #[test]
    fn test_blackjack_table_dealing_and_displaying_options() {
        let mut table = BlackjackTableCLI::new(500000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);

        table.place_bet(&mut player, 25.0);
        table.deal_hand(&mut player);
        table.display_playing_options(&player);
    }

    #[test]
    fn test_blackjack_table_dealing_and_playing_option() {
        let mut table = BlackjackTableCLI::new(500000000., 6, 7);
        let mut player = Player::new("Rick Sanchez".to_string(), 500.);

        table.place_bet(&mut player, 25.0);
        table.deal_hand(&mut player);
        table.display_playing_options(&player);
        let options = player.get_playing_options();
        table.play_option(&mut player, options, 2);
    }
}
