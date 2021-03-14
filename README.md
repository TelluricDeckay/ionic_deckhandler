[![Rust](https://github.com/TelluricDeckay/ionic-deckhandler/workflows/Rust/badge.svg?branch=trunk)](https://github.com/TelluricDeckay/ionic-deckhandler/actions?query=workflow%3ARust)
[![crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/ionic-deckhandler.svg
[crates-url]: https://crates.io/crates/ionic-deckhandler

Ionic Deckhandler
==================

Rust implementation of a deck creation and card shuffling library

## Examples

```rust
use ionic_deckhandler::{Card, Deck};

let mut deck = Card::get_deck();
{
    let first_card = &deck[0];
    assert_eq!(
        format!("{:?}", first_card.get_suit()),
        String::from("Clubs")
    );
}
deck.shuffle_deck();

for card in deck.iter() {
    println!("{:?}", card);
}
deck.sort();
for card in deck.iter() {
    println!("{:?}", card);
}
```

```rust
use ionic_deckhandler::{Card, Suit, Rank};

let card = Card::new(Rank::Five, Suit::Hearts);
let ucard_rank = Card::get_suit(&card) as usize;
let ucard_suit = Card::get_suit(&card) as usize;
```

```rust
use ionic_deckhandler::{Card, Suit, Rank};

let mut hand_arr = [
    Card::new(Rank::Five, Suit::Hearts),
    Card::new(Rank::Three, Suit::Hearts),
    Card::new(Rank::Two, Suit::Hearts),
    Card::new(Rank::Ace, Suit::Hearts),
    Card::new(Rank::Four, Suit::Hearts),
    ];

hand_arr.sort();
```

## Contributing

See
[CONTRIBUTING.md](https://github.com/TelluricDeckay/ionic-deckhandler/blob/trunk/CONTRIBUTING.md)

## Help and Support

* [Issues](https://github.com/TelluricDeckay/ionic-deckhandler/issues)
* [Chat room](https://telluric-deckay.zulipchat.com/)
* Emails listed in [Cargo.toml](https://github.com/TelluricDeckay/ionic-deckhandler/blob/trunk/Cargo.toml)

