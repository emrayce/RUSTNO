use relm::{Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};
use relm_derive::{Msg, widget};

use gtk::Orientation::{Vertical, Horizontal};


use crate::game::Game;
use crate::player::Player;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

// variable that will be updated
// what need to be displayed and change ?
// player hand, deck, playfield
// put the game struct ?
pub struct Model {
    game: Game
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        let player1 = Player::new(String::from("Sander"));
        let player2 = Player::new(String::from("Cedric"));
        let player3 = Player::new(String::from("Antoine"));
        let player4 = Player::new(String::from("Simon"));

        // Create players
        let players = vec![player1, player2, player3, player4];

        Model {
            game: Game::new(players),
        }
    } 

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }

    }

    view! {
        gtk::Window {
            #[name="drawing_area"]
            gtk::DrawingArea {
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

pub fn launch() {
    Win::run(()).expect("Win::run failed");
}
