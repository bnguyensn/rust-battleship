use crate::board::Board;
use core::fmt;
use std::io;

enum PlayerId {
    One,
    Two,
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerId::One => write!(f, "1"),
            PlayerId::Two => write!(f, "2"),
        }
    }
}

pub struct Game {
    player_1_name: String,
    player_2_name: String,
    board: Board,
}

impl Game {
    fn ask_for_player_name(player_number: PlayerId) -> String {
        let mut player_name = String::new();
        loop {
            println!("Enter the name of player {}:", player_number);
            if let Ok(_) = io::stdin().read_line(&mut player_name) {
                player_name = player_name.trim().to_string();
                if player_name.len() <= 20 && player_name.chars().all(|c| c.is_alphanumeric()) {
                    return player_name;
                } else {
                    println!("Invalid input. Please enter a name with at most 20 alphanumeric characters.");
                }
            } else {
                println!("Failed to read input.");
            }
        }
    }

    pub fn new() -> Self {
        let player_1_name = Self::ask_for_player_name(PlayerId::One);
        let player_2_name = Self::ask_for_player_name(PlayerId::Two);

        Game {
            player_1_name,
            player_2_name,
            board: Board::new(10),
        }
    }
}
