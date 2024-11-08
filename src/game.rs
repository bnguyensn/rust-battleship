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

const BOARD_SIZE: usize = 10;
const SHIP_SIZE: usize = 2; // Must not exceed the grid size.
const SHIPS_COUNT: usize = 2; // Must not mathematically exceed the grid size.

pub struct Game {
    player_1_name: String,
    player_2_name: String,
    player_1_ships_remaining: usize,
    player_2_ships_remaining: usize,
    board: Board,
}

impl Game {
    fn ask_for_player_name(player_id: PlayerId) -> String {
        let mut player_name = String::new();
        loop {
            println!("Enter the name of player {}:", player_id);

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

        let board = Board::new(BOARD_SIZE, SHIP_SIZE);

        Game {
            player_1_name,
            player_2_name,
            player_1_ships_remaining: SHIPS_COUNT,
            player_2_ships_remaining: SHIPS_COUNT,
            board,
        }
    }

    fn setup_ships(&mut self, player_name: &String) {
        println!("Player {}, please place your ships.", player_name);

        let mut ships_to_be_placed = SHIPS_COUNT;
        while ships_to_be_placed > 0 {
            let (x, y, orientation) = self.board.ask_for_ship_placement();
            self.board.place_ship(x, y, orientation);
            ships_to_be_placed -= 1;
        }
    }

    pub fn start(&mut self) {
        println!(
            "Welcome to the game, {} and {}!",
            self.player_1_name, self.player_2_name
        );

        self.setup_ships(&self.player_1_name.clone());
        self.setup_ships(&self.player_2_name.clone());
    }
}
