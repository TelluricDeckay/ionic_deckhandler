use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

mod suit {
    pub struct Suit(&'static str);

    impl super::fmt::Display for Suit {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            self.0.fmt(f)
        }
    }

    const CLUB: Suit = Suit("♣");
    const DIAMOND: Suit = Suit("♦");
    const HEART: Suit = Suit("♥");
    const SPADE: Suit = Suit("♠");

    pub const SUITS: [&Suit; 4] = [&CLUB, &DIAMOND, &HEART, &SPADE];
}

mod card_type {
    pub enum CardType {
        NumCardType(&'static str, i32),
        FaceCardType(&'static str, i32),
    }

    impl super::fmt::Display for CardType {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            match *self {
                CardType::NumCardType(str_val, i32) => str_val.fmt(f),
                CardType::FaceCardType(str_val, i32) => str_val.fmt(f),
            }
        }
    }

    use CardType::*;

    const ACE: CardType = NumCardType("Ace", 1);
    const TWO: CardType = NumCardType("Two", 2);
    const THREE: CardType = NumCardType("Three", 3);
    const FOUR: CardType = NumCardType("Four", 4);
    const FIVE: CardType = NumCardType("Five", 5);
    const SIX: CardType = NumCardType("Six", 6);
    const SEVEN: CardType = NumCardType("Seven", 7);
    const EIGHT: CardType = NumCardType("Eight", 8);
    const NINE: CardType = NumCardType("Nine", 9);
    const TEN: CardType = NumCardType("Ten", 10);
    const JACK: CardType = FaceCardType("Jack", 11);
    const QUEEN: CardType = FaceCardType("Queen", 12);
    const KING: CardType = FaceCardType("king", 13);

    pub const CARD_TYPES: [&CardType; 13] = [
        &ACE, &TWO, &THREE, &FOUR, &FIVE, &SIX, &SEVEN, &EIGHT, &NINE, &TEN, &JACK, &QUEEN, &KING,
    ];
}

pub struct Card {
    suit: &'static suit::Suit,
    card_type: &'static card_type::CardType,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.card_type, self.suit)
    }
}

impl Card {
    pub fn get_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in suit::SUITS.iter() {
            for card_type in card_type::CARD_TYPES.iter() {
                deck.push(Card {
                    suit: suit,
                    card_type: card_type,
                });
            }
        }
        deck
    }

    pub fn get_suit(&self) -> &suit::Suit {
        self.suit
    }

    pub fn get_type(&self) -> &card_type::CardType {
        self.card_type
    }

    pub fn is_facetype(&self) -> bool {
        match *self.card_type {
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
