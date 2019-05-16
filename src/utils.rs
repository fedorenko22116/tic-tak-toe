use crate::minimax::{Board, Player};
use std::fmt::{Display, Formatter, Error};

pub fn build_interface(data: &Board, result: &String) {
    println!("Tic-tak-toe game!\n\r");
    println!("Press 'Up', 'Down', 'Left', 'Right' to move over board.\r");
    println!("Press 'x' to make a choice.\r");
    println!("Press 'ESC' or 'Ctrl+C' to quit.\n\r");

    for chunk in data.chunks(3).into_iter() {
        println!("  {} | {} | {} \r", chunk[0], chunk[1], chunk[2]);
    }

    println!("\n\r{}\n\r", result);
}

pub fn get_offset() -> (u16, u16) {
    (4, 1)
}

pub fn start_offset() -> (u16, u16) {
    (2, 6)
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match self {
            Player::Computer => 'O',
            Player::Human => 'X',
            Player::Blank => ' ',
        })
    }
}
