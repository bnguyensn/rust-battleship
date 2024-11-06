use crate::board::Board;
use core::fmt;
use std::io;

const PLAYER_NAME_MAX_LENGTH: usize = 20;

fn validate_player_name(player_name: &String) -> bool {
    player_name.len() <= PLAYER_NAME_MAX_LENGTH && player_name.chars().all(|c| c.is_alphanumeric())
}

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
                if validate_player_name(&player_name) {
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

        let board = Board::new(10);

        Game {
            player_1_name,
            player_2_name,
            board,
        }
    }
}
