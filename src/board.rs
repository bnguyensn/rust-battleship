use std::io;

const SHIP_SIZE: usize = 2;

#[derive(Clone)]
pub struct Board {
    pub grid: Vec<Vec<char>>,
    grid_size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            grid: vec![vec!['~'; size]; size],
            grid_size: size,
        }
    }

    fn print(&self, hide_ships: bool) {}

    // ********** SETUP PHASE ********** //

    // Returns true if the ship was placed successfully, false otherwise.
    // The ship is placed horizontally if orientation is 'H', and vertically if orientation is 'V'.
    // The ship is placed at the given row and column.
    // The ship is represented by the character 'S'.
    // The ship cannot be placed if it would go out of bounds or overlap with another ship.
    // If the ship is placed successfully, the function returns true and updates the grid.
    // If the ship cannot be placed, the function returns false and the grid is unchanged.
    fn try_to_place_ship(&mut self, row: usize, col: usize, orientation: char) -> bool {
        match orientation {
            'H' => {
                if col + SHIP_SIZE <= self.grid_size {
                    // Check if there are any obstacles (ships etc.) in the way.
                    for i in 0..SHIP_SIZE {
                        if self.grid[row][col + i] != '~' {
                            return false;
                        }
                    }
                    // Place the ship.
                    for i in 0..SHIP_SIZE {
                        self.grid[row][col + i] = 'S';
                    }
                    true
                } else {
                    false
                }
            }
            'V' => {
                if row + SHIP_SIZE <= self.grid_size {
                    // Check if there are any obstacles (ships etc.) in the way.
                    for i in 0..SHIP_SIZE {
                        if self.grid[row + i][col] != '~' {
                            return false;
                        }
                    }
                    // Place the ship.
                    for i in 0..SHIP_SIZE {
                        self.grid[row + i][col] = 'S';
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn place_ship(&mut self, player_name: &str) {
        let mut ships = 2;
        while ships > 0 {}
    }

    /// Prompts the player to enter coordinates to fire at.
    /// Returns the coordinates as a u32 tuple.
    /// Loops until the player enters valid coordinates.
    fn prompt_for_coordinates(&self) -> (usize, usize) {
        loop {
            println!("Enter a coordinate to fire at e.g. 3 4");

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
        let (x, y) = self.prompt_for_coordinates();
        false
    }
}
