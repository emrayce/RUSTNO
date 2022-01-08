use crate::player::Player;
use crate::card::{
    Card, Symbol, Color
};

use rand::thread_rng;
use rand::seq::SliceRandom;
use regex::Regex;

use std::io::stdin;

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
            // Display the player info and the last card played 
            println!("{}", self.players[self.turn]);
            println!("Last card played: {}", self.last_card);

            // can the current player play ?
            if !self.players[self.turn].can_play(&self.last_card) {
                self.players[self.turn].add_card(draw_from_deck(&mut self.deck));
            }

            if self.players[self.turn].can_play(&self.last_card) {
                let chosen_card: usize = self.input_loop();

                // update playfield and last_card
                self.playfield.push(self.last_card);

                self.last_card = self.players[self.turn].play_card(chosen_card);
                // Do the card effect if any
            }

            // check if the current player wins
            if self.players[self.turn].has_no_card() {
                break();
            }

            self.turn = (self.turn + 1) % self.players.len();
        }

        println!("Congrats player {} you win the game motherfucker!", self.turn);
    }

    fn input_loop(&mut self) -> usize {
        loop {
            let mut chosen_card = String::new();
            println!("Chose a card to play: ");

            // player play a card;
            stdin().read_line(&mut chosen_card).expect("Enter a valid string");

            if is_user_input_valid(&chosen_card) {
                let index : usize = chosen_card[..1].parse::<usize>().unwrap();
                if index < self.players[self.turn].hand_length() {
                    return index;
                }
            }


            println!("Enter a valid card!");
        };
    }

    fn cur_player(&mut self) -> &Player {
        &self.players[self.turn]
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

// Check if user input is valid
// A valid input is [0-9]+
fn is_user_input_valid(string: &String) -> bool {
    let regex = Regex::new(r"[0-9]+").unwrap();
    regex.is_match(&string)
}
