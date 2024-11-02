use std::io;

#[derive(Clone)]
pub struct Board {
    pub grid: Vec<Vec<char>>,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: vec![vec!['~'; 10]; 10],
        }
    }

    fn print(&self, hide_ships: bool) {}

    fn place_ship(&mut self, player_name: &str) {}

    /// Prompts the player to enter coordinates to fire at.
    /// Returns the coordinates as a u32 tuple.
    /// Loops until the player enters valid coordinates.
    /// Coordinates are 0-indexed.
    fn prompt_for_coordinates(&self) -> (usize, usize) {
        let grid_width = self.grid.len();
        let grid_height = self.grid[0].len();

        loop {
            println!("Enter a coordinate to fire at e.g. 3 4");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    if x < grid_width && y < grid_height {
                        return (x, y);
                    } else {
                        println!("Coordinates out of bounds. Please enter values between 0 and {} for x and 0 and {} for y.", grid_width - 1, grid_height - 1);
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
