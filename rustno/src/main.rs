mod card;
use card::{
    Card, Symbol, Color
};

fn main() {
    // Create players
    let players = vec![0, 0, 0, 0];

    //Create deck and playfield
    let mut deck = Vec::new();
    for i in 1..5 {
        deck.push(Card::new(Color::RED, Symbol::ONE));
    }

    for i in deck.iter() {
        println!("{:?}", i);
    }
    // Distribute cards to player
    // Create order and first to play
    // Gameplay loop;
}
