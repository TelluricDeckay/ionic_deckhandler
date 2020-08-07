mod card;
use card::DeckShuffler;

fn main() {
    let mut deck = card::Card::get_deck();
    deck.shuffle_deck();
    
    for card in deck.iter() {
        println!("{}", card);
    }
    {
        let first_card = &deck[0];
        println!("The suit of the first card is {}. The type is '{}'.", first_card.get_suit(), first_card.get_type());
        println!("First card is face type: {}", first_card.is_facetype());
    }
}
