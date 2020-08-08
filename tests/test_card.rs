use ionic_deckhandler::Deck;

#[test]
fn test_create_deck() {
    let mut deck = ionic_deckhandler::Card::get_deck();
    {
        let first_card = &deck[0];
        assert_eq!(format!("{}", first_card.get_suit()), String::from("♣"));
    }
    deck.shuffle_deck();

    for card in deck.iter() {
        println!("{}", card);
    }
}
