use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod suit {
    pub struct Suit(&'static str);

    impl super::fmt::Display for Suit {
        fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
            self.0.fmt(f)
        }
    }

    const club: Suit = Suit("♣");
    const diamond: Suit = Suit("♦");
    const heart: Suit = Suit("♥");
    const spade: Suit = Suit("♠");

    pub const suits: [&Suit; 4] = [&club, &diamond, &heart, &spade];
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

    const ace: CardType = NumCardType("Ace");
    const two: CardType = NumCardType("Two");
    const three: CardType = NumCardType("Three");
    const four: CardType = NumCardType("Four");
    const five: CardType = NumCardType("Five");
    const six: CardType = NumCardType("Six");
    const seven: CardType = NumCardType("Seven");
    const eight: CardType = NumCardType("Eight");
    const nine: CardType = NumCardType("Nine");
    const ten: CardType = NumCardType("Ten");
    const jack: CardType = FaceCardType("Jack");
    const queen: CardType = FaceCardType("Queen");
    const king: CardType = FaceCardType("king");

    pub const card_types: [&CardType; 13] = [&ace, &two, &three, &four, &five, &six, &seven, &eight, &nine, &ten, &jack, &queen, &king];
}

pub struct Card {
    suit: &'static suit::Suit,
    cardType: &'static card_type::CardType,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.cardType, self.suit)
    }
}

impl Card {
    pub fn get_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in suit::suits.iter() {
            for card_type in card_type::card_types.iter() {
                deck.push(Card{ suit: suit, cardType: card_type });
            }
        }
        deck
    }

    pub fn get_suit(&self) -> &suit::Suit {
        self.suit
    }

    pub fn get_type(&self) -> &card_type::CardType {
        self.cardType
    }

    pub fn is_facetype(&self) -> bool {
        match *self.cardType {
            card_type::CardType::FaceCardType(..) => true,
            _ => false
        }
    }
}

pub trait DeckShuffler {
    fn shuffle_deck(&mut self);
}

impl DeckShuffler for Vec<Card> {
    fn shuffle_deck(&mut self) {
        self.shuffle(&mut thread_rng())
    }
}