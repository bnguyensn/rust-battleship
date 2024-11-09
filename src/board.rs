use core::fmt;
use std::io;

const HORIZONTAL: char = 'H';
const VERTICAL: char = 'V';
const WATER: char = '~';

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "HORIZONTAL"),
            Orientation::Vertical => write!(f, "VERTICAL"),
        }
    }
}

const SHIP_PLACEMENT_INVALID_INPUT_MSG: &str = "Invalid input. Please enter coordinates in the form 'x y orientation' where x and y are integers, and are separated by a single space, and orientation is either 'H' or 'V'.";
const SHIP_PLACEMENT_INVALID_ORIENTATION_MSG: &str =
    "Invalid orientation. Please enter either 'H' for horizontal or 'V' for vertical.";
const SHIP_PLACEMENT_OVERLAP_MSG: &str =
    "Ship placement overlaps with another ship. Please try again.";
const FAILED_TO_READ_INPUT_MSG: &str = "Failed to read input.";

#[cfg(debug_assertions)]
fn debug_parts(parts: &Vec<&str>) {
    println!("parts: {:?}", parts);
}

#[derive(Clone)]
pub struct Board {
    pub grid: Vec<Vec<char>>, // For fixed-size grids: [[char; 10]; 10]
    grid_size: usize,
    ship_size: usize,
    ship_x_bound: usize,
    ship_y_bound: usize,
}

impl Board {
    pub fn new(grid_size: usize, ship_size: usize) -> Self {
        Board {
            grid: vec![vec![WATER; grid_size]; grid_size], // For fixed size grids: [['~'; 10]; 10]
            grid_size,
            ship_size,
            ship_x_bound: grid_size - ship_size,
            ship_y_bound: grid_size - ship_size,
        }
    }

    // ********** SETUP PHASE ********** //

    pub fn place_ship(&mut self, x: usize, y: usize, orientation: Orientation, ship_id: char) {
        match orientation {
            Orientation::Horizontal => {
                for i in 0..self.ship_size {
                    self.grid[x + i][y] = ship_id;
                }
            }
            Orientation::Vertical => {
                for i in 0..self.ship_size {
                    self.grid[x][y + i] = ship_id;
                }
            }
        }
        println!("Ship {ship_id} placed at ({x}, {y}) with orientation {orientation}");
    }

    // Return the coordinate and the ship orientation from player input.
    // Loop until the player enters valid coordinate and orientation.
    pub fn ask_for_ship_placement(&mut self) -> (usize, usize, Orientation) {
        println!("Enter the coordinate and orientation for your ship e.g. 3 4 H");
        'ship_placement: loop {
            let mut placement_input = String::new();
            match io::stdin().read_line(&mut placement_input) {
                Ok(_) => {
                    let parts: Vec<&str> = placement_input.trim().split_whitespace().collect();

                    #[cfg(debug_assertions)]
                    debug_parts(&parts);

                    if parts.len() != 3 {
                        println!("{}", SHIP_PLACEMENT_INVALID_INPUT_MSG);
                        continue;
                    }
                    let (x, y, orientation_char) = (
                        parts[0].parse::<usize>(),
                        parts[1].parse::<usize>(),
                        parts[2].chars().next(),
                    );

                    match (x, y, orientation_char) {
                        (Ok(x), Ok(y), Some(orientation)) => {
                            if orientation != HORIZONTAL && orientation != VERTICAL {
                                println!("{}", SHIP_PLACEMENT_INVALID_ORIENTATION_MSG);
                                continue;
                            }

                            if x >= self.ship_x_bound || y >= self.ship_y_bound {
                                println!("Coordinate out of bounds. Please enter values between 0 and {} for x and 0 and {} for y.", self.ship_x_bound, self.ship_y_bound);
                                continue;
                            }

                            match orientation {
                                HORIZONTAL => {
                                    for i in 0..self.ship_size {
                                        if self.grid[x + i][y] != WATER {
                                            println!("{}", SHIP_PLACEMENT_OVERLAP_MSG);
                                            continue 'ship_placement;
                                        }
                                    }
                                }
                                VERTICAL => {
                                    for i in 0..self.ship_size {
                                        if self.grid[x][y + i] != WATER {
                                            println!("{}", SHIP_PLACEMENT_OVERLAP_MSG);
                                            continue 'ship_placement;
                                        }
                                    }
                                }
                                _ => {
                                    println!("{}", SHIP_PLACEMENT_INVALID_ORIENTATION_MSG);
                                    continue;
                                }
                            }

                            let orientation = match orientation {
                                HORIZONTAL => Orientation::Horizontal,
                                VERTICAL => Orientation::Vertical,
                                _ => continue,
                            };
                            return (x, y, orientation);
                        }
                        _ => {
                            println!("{}", FAILED_TO_READ_INPUT_MSG);
                        }
                    }
                }
                Err(_) => {
                    println!("{}", FAILED_TO_READ_INPUT_MSG);
                }
            }
        }
    }

    // ********** GAME PHASE ********** //

    /// Prompts the player to enter coordinates to fire at.
    /// Returns the coordinates as a u32 tuple.
    /// Loops until the player enters valid coordinates.
    fn ask_for_shoot_target(&self) -> (usize, usize) {
        loop {
            println!("Enter a coordinate to shoot at e.g. 3 4");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    if x < self.grid_size && y < self.grid_size {
                        return (x, y);
                    } else {
                        println!("Coordinates out of bounds. Please enter values between 0 and {} for x and 0 and {} for y.", self.grid_size - 1, self.grid_size - 1);
                    }
                }
            }
            println!("Invalid input. Please enter coordinates in the form 'x y' where x and y are integers, and are separated by a single space.");
        }
    }

    fn take_turn(&mut self, player_name: &str) -> bool {
        let (x, y) = self.ask_for_shoot_target();
        false
    }
}
