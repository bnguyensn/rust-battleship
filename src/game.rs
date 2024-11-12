use crate::board::Board;
use crate::words::pluralize;
use core::fmt;
use std::io;

const FAILED_TO_READ_INPUT_MSG: &str = "Failed to read input.";

const PLAYER_NAME_MAX_LENGTH: usize = 20;

fn validate_player_name(player_name: &String) -> bool {
    player_name.len() <= PLAYER_NAME_MAX_LENGTH
        && player_name.len() > 0
        && player_name.chars().all(|c| c.is_alphanumeric())
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

const BOARD_SIZE_MIN: usize = 5;
const BOARD_SIZE_MAX: usize = 20;
const BOARD_SIZE_DEFAULT: usize = 10;
const SHIP_SIZE: usize = 2; // Must not exceed the grid size.
const SHIPS_COUNT: usize = 2; // Must not mathematically exceed the grid size.

pub struct Game {
    player_1_name: String,
    player_2_name: String,
    player_1_ships_remaining: usize,
    player_2_ships_remaining: usize,
    player_1_board: Board,
    player_2_board: Board,
}

fn ask_for_player_name(player_id: PlayerId) -> String {
    println!("Enter the name of player {}:", player_id);
    loop {
        let mut player_name = String::new();
        if let Ok(_) = io::stdin().read_line(&mut player_name) {
            player_name = player_name.trim().to_string();
            if validate_player_name(&player_name) {
                return player_name;
            } else {
                println!(
                    "Invalid input. Please enter a name with at most 20 alphanumeric characters."
                );
            }
        } else {
            println!("{FAILED_TO_READ_INPUT_MSG}");
        }
    }
}

fn ask_for_board_size() -> usize {
    println!(
        "Enter the size of the board. The board is a square with its side ranging from {} to {}. Leave blank for the default size (10):",
        BOARD_SIZE_MIN, BOARD_SIZE_MAX
    );
    loop {
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let trimmed_input = input.trim();

            if trimmed_input.is_empty() {
                return BOARD_SIZE_DEFAULT;
            }

            match input.trim().parse::<usize>() {
                Ok(size) => {
                    if size >= BOARD_SIZE_MIN && size <= BOARD_SIZE_MAX {
                        return size;
                    } else {
                        println!(
                            "Invalid input. Please enter a number between {} and {}.",
                            BOARD_SIZE_MIN, BOARD_SIZE_MAX
                        );
                    }
                }
                Err(_) => {
                    println!(
                        "Invalid input. Please enter a number between {} and {}.",
                        BOARD_SIZE_MIN, BOARD_SIZE_MAX
                    );
                }
            }
        } else {
            println!("{FAILED_TO_READ_INPUT_MSG}");
        }
    }
}

fn setup_ships(player_name: &String, board: &mut Board) {
    println!("{player_name}, please place your ships.");
    let mut ships_to_be_placed = SHIPS_COUNT;
    while ships_to_be_placed > 0 {
        let (x, y, orientation) = board.ask_for_ship_placement();
        board.place_ship(x, y, orientation, (ships_to_be_placed as u8 + b'0') as char);
        ships_to_be_placed -= 1;
    }
}

fn take_turn(player_name: &String, opponent_name: &String, opponent_board: &mut Board) -> bool {
    println!("{player_name}, it's your turn.");
    let (x, y) = opponent_board.ask_for_shoot_target();
    match opponent_board.shoot(x, y) {
        Some(ship_id) => {
            println!("{opponent_name}'s ship {ship_id} was hit and sunk.");
            return true;
        }
        None => {
            println!("{player_name} did not hit anything.");
            return false;
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let player_1_name = ask_for_player_name(PlayerId::One);
        let player_2_name = ask_for_player_name(PlayerId::Two);

        let board_size = ask_for_board_size();

        Game {
            player_1_name,
            player_2_name,
            player_1_ships_remaining: SHIPS_COUNT,
            player_2_ships_remaining: SHIPS_COUNT,
            player_1_board: Board::new(board_size, SHIP_SIZE),
            player_2_board: Board::new(board_size, SHIP_SIZE),
        }
    }

    pub fn start(&mut self) {
        println!(
            "Welcome to the game, {} and {}! The board size is {}x{}.",
            self.player_1_name,
            self.player_2_name,
            self.player_1_board.grid_size, // Assuming both players have the same board size.
            self.player_1_board.grid_size
        );

        setup_ships(&self.player_1_name, &mut self.player_1_board);
        setup_ships(&self.player_2_name, &mut self.player_2_board);

        println!("All set, time to play!");

        loop {
            let player_1_hit_a_ship = take_turn(
                &self.player_1_name,
                &self.player_2_name,
                &mut self.player_2_board,
            );
            if player_1_hit_a_ship {
                self.player_2_ships_remaining -= 1;
                print!(
                    "{} - remaining {}: {} ",
                    &self.player_2_name,
                    pluralize("ship", self.player_2_ships_remaining),
                    self.player_2_ships_remaining
                );
            }
            if self.player_2_ships_remaining == 0 {
                break;
            }
            let player_2_hit_a_ship = take_turn(
                &self.player_2_name,
                &self.player_1_name,
                &mut self.player_1_board,
            );
            if player_2_hit_a_ship {
                self.player_1_ships_remaining -= 1;
                print!(
                    "{} - remaining {}: {} ",
                    &self.player_1_name,
                    pluralize("ship", self.player_1_ships_remaining),
                    self.player_1_ships_remaining
                );
            }
            if self.player_1_ships_remaining == 0 {
                break;
            }
        }

        println!(
            "Congratulations! {} won the game!",
            if self.player_1_ships_remaining == 0 {
                &self.player_2_name
            } else {
                &self.player_1_name
            }
        );
    }
}
