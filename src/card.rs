use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

/// Provides a rank (value)
///
/// Examples:
///
/// ```
/// use ionic_deckhandler::{Card, Suit, Rank};
///
/// let card = Card::new(Rank::Five, Suit::Hearts);
/// let ucard_rank = Card::get_rank(&card) as usize;
/// ```
///
/// ```
/// use ionic_deckhandler::{Card, Suit, Rank};
///
/// let mut hand_arr = [
///     Card::new(Rank::Five, Suit::Hearts),
///     Card::new(Rank::Three, Suit::Hearts),
///     Card::new(Rank::Two, Suit::Hearts),
///     Card::new(Rank::Ace, Suit::Hearts),
///     Card::new(Rank::Four, Suit::Hearts),
///     ];
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// The total number of ranks within the deck of cards
pub const CARD_RANK_COUNT: usize = 13;

pub const ALL_RANKS: [Rank; CARD_RANK_COUNT] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

/// Provides a card suit
///
/// Examples:
///
/// ```
/// use ionic_deckhandler::{Card, Suit, Rank};
///
/// let card = Card::new(Rank::Five, Suit::Hearts);
/// let ucard_suit = Card::get_suit(&card) as usize;
/// ```
///
/// ```
/// use ionic_deckhandler::{Card, Suit, Rank};
///
/// let mut hand_arr = [
///     Card::new(Rank::Five, Suit::Hearts),
///     Card::new(Rank::Three, Suit::Hearts),
///     Card::new(Rank::Two, Suit::Hearts),
///     Card::new(Rank::Ace, Suit::Hearts),
///     Card::new(Rank::Four, Suit::Hearts),
///     ];
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

/// A card containing a rank (card value) and a suit. A card may be compared
/// by value against other cards.
///
/// Examples:
///
/// ```
/// use ionic_deckhandler::{Card, Suit, Rank};
///
/// let mut hand_arr = [
///     Card::new(Rank::Five, Suit::Hearts),
///     Card::new(Rank::Three, Suit::Hearts),
///     Card::new(Rank::Two, Suit::Hearts),
///     Card::new(Rank::Ace, Suit::Hearts),
///     Card::new(Rank::Four, Suit::Hearts),
///     ];
///
/// hand_arr.sort();
/// ```
///
#[derive(Debug, Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn get_deck() -> Vec<Self> {
        let mut deck = Vec::new();
        for suit in ALL_SUITS.iter() {
            for rank in ALL_RANKS.iter() {
                deck.push(Card::new(rank.clone(), suit.clone()));
            }
        }
        deck
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

/// A deck may be shuffled and sorted
///
/// Example:
///
/// ```
/// use ionic_deckhandler::{Card, Deck};
///
/// let mut deck = Card::get_deck();
/// {
///     let first_card = &deck[0];
///     assert_eq!(
///         format!("{:?}", first_card.get_suit()),
///         String::from("Clubs")
///     );
/// }
/// deck.shuffle_deck();
///
/// for card in deck.iter() {
///     println!("{:?}", card);
/// }
/// deck.sort();
/// for card in deck.iter() {
///     println!("{:?}", card);
/// }
/// ```
pub trait Deck {
    fn shuffle_deck(&mut self);

    fn take_from_top(&mut self) -> Option<Card>;

    fn push_to_bottom(&mut self, card: Card);
}

impl Deck for Vec<Card> {
    fn shuffle_deck(&mut self) {
        self.shuffle(&mut thread_rng())
    }

    fn take_from_top(&mut self) -> Option<Card> {
        self.pop()
    }

    fn push_to_bottom(&mut self, card: Card) {
        let len = self.len();
        self.insert(len, card);
    }
}
