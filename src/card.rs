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
        NumCardType(&'static str),
        FaceCardType(&'static str),
    }

    impl super::fmt::Display for CardType {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            match *self {
                CardType::NumCardType(str_val) => str_val.fmt(f),
                CardType::FaceCardType(str_val) => str_val.fmt(f),
            }
        }
    }

    use CardType::*;

    const ACE: CardType = NumCardType("Ace");
    const TWO: CardType = NumCardType("Two");
    const THREE: CardType = NumCardType("Three");
    const FOUR: CardType = NumCardType("Four");
    const FIVE: CardType = NumCardType("Five");
    const SIX: CardType = NumCardType("Six");
    const SEVEN: CardType = NumCardType("Seven");
    const EIGHT: CardType = NumCardType("Eight");
    const NINE: CardType = NumCardType("Nine");
    const TEN: CardType = NumCardType("Ten");
    const JACK: CardType = FaceCardType("Jack");
    const QUEEN: CardType = FaceCardType("Queen");
    const KING: CardType = FaceCardType("king");

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
