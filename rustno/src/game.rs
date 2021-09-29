use crate::player::Player;
use crate::card::{
    Card, Symbol, Color
};

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    playfield: Vec<Card>,
    turn: i32,
}

impl Game {
    pub fn new(mut players: Vec<Player>) -> Game {
        // Create deck
        let mut deck = create_deck();
        // shuffle
        shuffle(&mut deck);
        // Distribute
        for _ in 0..2 {
            for i in players.iter_mut() {
                i.add_card(draw_from_deck(&mut deck));
            }
        }

        // init playfield
        let mut playfield = Vec::new();
        playfield.push(draw_from_deck(&mut deck));
        // turn = player1
        let mut turn: i32 = 0;


        Game { players, deck, playfield, turn }
    }


}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for _ in 1..5 {
        deck.push(Card::new(Color::RED, Symbol::One));
        deck.push(Card::new(Color::BLUE, Symbol::One));
        deck.push(Card::new(Color::GREEN, Symbol::One));
        deck.push(Card::new(Color::YELLOW, Symbol::One));
    }

    deck
}

fn shuffle(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}

fn draw_from_deck(deck: &mut Vec<Card>) -> Card {
    let tmp = deck.pop();

    match tmp {
        Some(x) => x,
        None => unimplemented!(),
    }
}


