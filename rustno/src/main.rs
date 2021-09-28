mod player;
use player::Player;
mod card;
use card::{
    Card, Symbol, Color
};

use rand::thread_rng;
use rand::seq::SliceRandom;

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

    //Create deck
    let mut deck = Vec::new();
    for i in 1..5 {
        deck.push(Card::new(Color::RED, Symbol::ONE));
        deck.push(Card::new(Color::BLUE, Symbol::ONE));
        deck.push(Card::new(Color::GREEN, Symbol::ONE));
        deck.push(Card::new(Color::YELLOW, Symbol::ONE));
    }

    // Randomize deck
    deck.shuffle(&mut thread_rng());
    for i in deck.iter() {
        println!("{:?}", i);
    }
    
    // Distribute cards to player
    for _ in 1..3 {
        for i in players.iter() {
        }
    }

    // Create order and first to play
    // Gameplay loop;
}

pub fn draw(mut player: Player, n: usize, mut deck: Vec<Card>) {
    if n > deck.len() {

    }
    else {
        player.draw(deck, n);
    }
}
