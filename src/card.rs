use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

pub mod suit {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Suit(pub &'static str);

    impl super::fmt::Display for Suit {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            self.0.fmt(f)
        }
    }

    pub const CLUB: Suit = Suit("♣");
    pub const DIAMOND: Suit = Suit("♦");
    pub const HEART: Suit = Suit("♥");
    pub const SPADE: Suit = Suit("♠");

    pub const SUITS: [&Suit; 4] = [&CLUB, &DIAMOND, &HEART, &SPADE];
}

pub mod card_type {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum CardType {
        NumCardType(&'static str, i32),
        FaceCardType(&'static str, i32),
    }

    impl super::fmt::Display for CardType {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            match *self {
                CardType::NumCardType(str_val, val) => write!(f, "{}, {}", str_val, val),
                CardType::FaceCardType(str_val, val) => write!(f, "{}, {}", str_val, val),
            }
        }
    }

    use CardType::*;

    pub const ACE: CardType = NumCardType("Ace", 1);
    pub const TWO: CardType = NumCardType("Two", 2);
    pub const THREE: CardType = NumCardType("Three", 3);
    pub const FOUR: CardType = NumCardType("Four", 4);
    pub const FIVE: CardType = NumCardType("Five", 5);
    pub const SIX: CardType = NumCardType("Six", 6);
    pub const SEVEN: CardType = NumCardType("Seven", 7);
    pub const EIGHT: CardType = NumCardType("Eight", 8);
    pub const NINE: CardType = NumCardType("Nine", 9);
    pub const TEN: CardType = NumCardType("Ten", 10);
    pub const JACK: CardType = FaceCardType("Jack", 11);
    pub const QUEEN: CardType = FaceCardType("Queen", 12);
    pub const KING: CardType = FaceCardType("king", 13);

    pub const CARD_TYPES: [&CardType; 13] = [
        &ACE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE, &TEN, &JACK, &QUEEN, &KING,
    ];
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    suit: &'static suit::Suit,
    pub value: &'static card_type::CardType,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

impl Card {
    pub fn new(suit: &'static suit::Suit, value: &'static card_type::CardType) -> Self {
        Self {
            suit: suit,
            value: value,
        }
    }

    pub fn get_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in suit::SUITS.iter() {
            for value in card_type::CARD_TYPES.iter() {
                deck.push(Card {
                    suit: suit,
                    value: value,
                });
            }
        }
        deck
    }

    pub fn get_suit(&self) -> &suit::Suit {
        self.suit
    }

    pub fn get_type(&self) -> &card_type::CardType {
        self.value
    }

    pub fn is_facetype(&self) -> bool {
        match *self.value {
            card_type::CardType::FaceCardType(..) => true,
            _ => false,
        }
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
