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
    last_card: Card,
    turn: usize,
}

impl Game {
    pub fn new(mut players: Vec<Player>) -> Game {
        // Create deck
        let mut deck = create_deck();

        // shuffle deck
        shuffle(&mut deck);

        // Distribute cards to player
        for _ in 0..2 {
            for i in players.iter_mut() {
                i.add_card(draw_from_deck(&mut deck));
            }
        }

        // init playfield
        let playfield = Vec::new();
        let last_card = draw_from_deck(&mut deck);

        // turn = player1
        let turn: usize = 0;


        Game { players, deck, playfield, last_card, turn }
    }

    pub fn game_loop(&mut self) {
        loop {
            // can the current player play ?
            if (!self.players[self.turn].can_play(&self.last_card)) {
                self.players[self.turn].add_card(draw_from_deck(&mut self.deck));
            }

            if (self.players[self.turn].can_play(&self.last_card)) {
                println!("player {} can play", self.turn);
                // player play a card;
                // update playfield and last_card
                // Do the card effect if any
            }

            // check if the current player win
            if (self.players[self.turn].has_no_card()) {
                break();
            }

            self.turn = (self.turn + 1) % self.players.len();
        }

        println!("Congrats player {} you win the game motherfucker!", self.turn);
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
