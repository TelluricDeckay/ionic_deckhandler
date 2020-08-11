use ionic_deckhandler::{Card, Deck, Rank, Suit};

#[test]
fn test_create_deck() {
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
}
