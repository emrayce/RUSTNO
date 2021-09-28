mod player;
use player::Player;
mod card;
use card::{
    Card, Symbol, Color
};

fn main() {
    let mut player1 = Player::new(String::from("Sander"));
    let mut player2 = Player::new(String::from("Cedric"));
    let mut player3 = Player::new(String::from("Antoine"));
    let mut player4 = Player::new(String::from("Simon"));

    // Create players
    let players = vec![player1, player2, player3, player4];

    for i in players.iter() {
        println!("{:?}", i);
    }

    //Create deck and playfield
    let mut deck = Vec::new();
    for i in 1..5 {
        deck.push(Card::new(Color::RED, Symbol::ONE));
        deck.push(Card::new(Color::BLUE, Symbol::ONE));
        deck.push(Card::new(Color::GREEN, Symbol::ONE));
        deck.push(Card::new(Color::YELLOW, Symbol::ONE));
    }

    /*for i in deck.iter() {
        println!("{:?}", i);
    }*/
    // Distribute cards to player
    
    // Create order and first to play
    // Gameplay loop;
}
