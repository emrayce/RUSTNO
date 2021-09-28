use crate::card;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<card::Card>,
}

impl Player {

    pub fn new(name: String) -> Player {
        Player { name, hand: Vec::new() }
    }
}
