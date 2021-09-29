mod player;
use player::Player;
mod card;
use card::{
    Card, Symbol, Color
};

use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {

    let player1 = Player::new(String::from("Sander"));
    let player2 = Player::new(String::from("Cedric"));
    let player3 = Player::new(String::from("Antoine"));
    let player4 = Player::new(String::from("Simon"));

    // Create players
    let mut players = vec![player1, player2, player3, player4];

    /*for i in players.iter() {
        println!("{:?}", i);
    }*/

    //Create deck
    let mut deck = Vec::new();
    let mut playfield = Vec::new();

    for i in 1..5 {
        deck.push(Card::new(Color::RED, Symbol::ONE));
        deck.push(Card::new(Color::BLUE, Symbol::ONE));
        deck.push(Card::new(Color::GREEN, Symbol::ONE));
        deck.push(Card::new(Color::YELLOW, Symbol::ONE));
    }

    // Randomize deck
    deck.shuffle(&mut thread_rng());
    /*for i in deck.iter() {
        println!("{:?}", i);
    }*/
    
    // Distribute cards to player
    for _ in 1..3 {
        for i in players.iter_mut() {
            i.add_card(draw_from_deck(&mut deck));
        }
    }
for i in players.iter() {
        println!("{:?}", i);
    }

    playfield.push(draw_from_deck(&mut deck));   

    // Create order and first to play
    // Gameplay loop;
}

pub fn draw_from_deck(deck: &mut Vec<Card>) -> Card {
    let tmp = deck.pop();

    match tmp {
        Some(x) => x,
        None => unimplemented!(),
    }
}



