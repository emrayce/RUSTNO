mod player;
use player::Player;
mod card;
use card::{
    Card, Symbol, Color
};

mod game;
use game::Game;


fn main() {

    let player1 = Player::new(String::from("Sander"));
    let player2 = Player::new(String::from("Cedric"));
    let player3 = Player::new(String::from("Antoine"));
    let player4 = Player::new(String::from("Simon"));

    // Create players
    let players = vec![player1, player2, player3, player4];

    let game = Game::new(players);

    println!("{:?}", game);

}



