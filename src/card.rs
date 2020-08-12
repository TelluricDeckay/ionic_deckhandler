use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

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
