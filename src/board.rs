use core::fmt;
use std::collections::HashMap;
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

#[derive(Debug, Clone)]
pub struct Ship {
    pub id: char,
    pub coordinates: Vec<[usize; 2]>,
    pub is_sunk: bool,
}

const SHIP_PLACEMENT_INVALID_INPUT_MSG: &str = "Invalid input. Please enter coordinates in the form 'x y orientation' where x and y are integers, and are separated by a single space, and orientation is either 'H' or 'V'.";
const SHIP_PLACEMENT_INVALID_ORIENTATION_MSG: &str =
    "Invalid orientation. Please enter either 'H' for horizontal or 'V' for vertical.";
const SHIP_PLACEMENT_OVERLAP_MSG: &str =
    "Ship placement overlaps with another ship. Please try again.";
const SHOOTING_INVALID_INPUT_MSG: &str = "Invalid input. Please enter coordinates in the form 'x y' where x and y are integers, and are separated by a single space.";
const FAILED_TO_READ_INPUT_MSG: &str = "Failed to read input.";

#[cfg(debug_assertions)]
fn debug_parts(parts: &Vec<&str>) {
    println!("parts: {:?}", parts);
}

fn get_coordinate(grid: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    grid[y][x]
}

fn set_coordinate(grid: &mut Vec<Vec<char>>, x: usize, y: usize, value: char) {
    grid[y][x] = value;
}

#[derive(Clone)]
pub struct Board {
    pub grid: Vec<Vec<char>>, // For fixed-size grids: [[char; 10]; 10]
    pub grid_size: usize,
    ship_size: usize,
    ship_x_bound: usize,
    ship_y_bound: usize,
    ships: HashMap<char, Ship>,
}

impl Board {
    pub fn new(grid_size: usize, ship_size: usize) -> Self {
        Board {
            grid: vec![vec![WATER; grid_size]; grid_size], // For fixed size grids: [['~'; 10]; 10]
            grid_size,
            ship_size,
            ship_x_bound: grid_size - ship_size,
            ship_y_bound: grid_size - ship_size,
            ships: HashMap::new(),
        }
    }

    // ********** UTILITIES ********** //

    fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.grid_size && y < self.grid_size
    }

    fn check_bounds_ship(&self, x: usize, y: usize, ship_orientation: &Orientation) -> bool {
        match ship_orientation {
            Orientation::Horizontal => self.check_bounds(x + (&self.ship_size - 1), y),
            Orientation::Vertical => self.check_bounds(x, y + (&self.ship_size - 1)),
        }
    }

    fn get_ship_by_id(&self, ship_id: char) -> Option<&Ship> {
        self.ships.get(&ship_id)
    }

    pub fn get_remaining_ships_count(&self) -> usize {
        self.ships.len()
    }

    // ********** SETUP PHASE ********** //

    pub fn place_ship(&mut self, x: usize, y: usize, orientation: Orientation, ship_id: char) {
        let mut ship_coordinates: Vec<[usize; 2]> = Vec::new();
        match orientation {
            Orientation::Horizontal => {
                for i in 0..self.ship_size {
                    set_coordinate(&mut self.grid, x + i, y, ship_id);
                    ship_coordinates.push([y, x + i]);
                }
            }
            Orientation::Vertical => {
                for i in 0..self.ship_size {
                    set_coordinate(&mut self.grid, x, y + i, ship_id);
                    ship_coordinates.push([y + i, x]);
                }
            }
        }
        let ship = Ship {
            id: ship_id,
            coordinates: ship_coordinates,
            is_sunk: false,
        };
        self.ships.insert(ship_id, ship);
        println!("Ship {ship_id} placed at ({x}, {y}) with orientation {orientation}");
    }

    // Return the ship placement coordinate and orientation from player input.
    // Loop until the player enters a valid coordinate and orientation.
    pub fn ask_for_ship_placement(&mut self) -> (usize, usize, Orientation) {
        println!("Enter the coordinate and orientation for your ship e.g. 3 4 H");
        'ship_placement: loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let parts: Vec<&str> = input.trim().split_whitespace().collect();
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

                    let orientation = match orientation_char {
                        Some(HORIZONTAL) => Ok(Orientation::Horizontal),
                        Some(VERTICAL) => Ok(Orientation::Vertical),
                        _ => {
                            println!("{}", SHIP_PLACEMENT_INVALID_ORIENTATION_MSG);
                            Err(SHIP_PLACEMENT_INVALID_ORIENTATION_MSG)
                        }
                    };

                    match (x, y, orientation) {
                        (Ok(x), Ok(y), Ok(orientation)) => {
                            if !self.check_bounds_ship(x, y, &orientation) {
                                println!("Coordinate out of bounds. Please enter values between 0 and {} for x and 0 and {} for y.", self.ship_x_bound, self.ship_y_bound);
                                continue;
                            }

                            match orientation {
                                Orientation::Horizontal => {
                                    for i in 0..self.ship_size {
                                        if get_coordinate(&self.grid, x + i, y) != WATER {
                                            println!("{}", SHIP_PLACEMENT_OVERLAP_MSG);
                                            continue 'ship_placement;
                                        }
                                    }
                                }
                                Orientation::Vertical => {
                                    for i in 0..self.ship_size {
                                        if get_coordinate(&self.grid, x, y + i) != WATER {
                                            println!("{}", SHIP_PLACEMENT_OVERLAP_MSG);
                                            continue 'ship_placement;
                                        }
                                    }
                                }
                            }

                            return (x, y, orientation);
                        }
                        _ => {
                            println!("{}", FAILED_TO_READ_INPUT_MSG);
                        }
                    }
                }
                _ => {
                    println!("{}", FAILED_TO_READ_INPUT_MSG);
                }
            }
        }
    }

    // TODO: add a single public method that performs place_ship() and ask_for_ship_placement() in one go.
    //  Then, make place_ship() and ask_for_ship_placement() private.

    // ********** GAME PHASE ********** //

    fn sink(&mut self, ship_id: &char) {
        match self.ships.get(&ship_id) {
            Some(ship) => {
                for coordinate in ship.coordinates.iter() {
                    let (x, y) = (coordinate[0], coordinate[1]);
                    set_coordinate(&mut self.grid, x, y, WATER);
                }
                self.ships.get_mut(&ship_id).unwrap().is_sunk = true;
                self.ships.remove(&ship_id);
            }
            None => {}
        }
    }

    pub fn shoot(&mut self, x: usize, y: usize) -> Option<char> {
        let target = get_coordinate(&self.grid, x, y);
        match target {
            WATER => {
                return None;
            }
            _ => match self.get_ship_by_id(target) {
                None => {
                    return None;
                }
                Some(ship) => {
                    if ship.is_sunk {
                        return None;
                    }
                    self.sink(&target);
                    return Some(target);
                }
            },
        }
    }

    /// Return the shooting coordinate from player input.
    /// Loops until the player enters a valid coordinate.
    pub fn ask_for_shoot_target(&self) -> (usize, usize) {
        println!("Enter a coordinate to shoot at e.g. 3 4");
        loop {
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let parts: Vec<&str> = input.trim().split_whitespace().collect();
                    #[cfg(debug_assertions)]
                    debug_parts(&parts);
                    if parts.len() != 2 {
                        println!("{SHOOTING_INVALID_INPUT_MSG}");
                        continue;
                    }
                    let (x, y) = (parts[0].parse::<usize>(), parts[1].parse::<usize>());

                    match (x, y) {
                        (Ok(x), Ok(y)) => {
                            if x >= self.grid_size || y >= self.grid_size {
                                println!("Coordinate out of bounds. Please enter values between 0 and {} for x and 0 and {} for y.", self.grid_size - 1, self.grid_size - 1);
                                continue;
                            }
                            return (x, y);
                        }
                        _ => {
                            println!("{FAILED_TO_READ_INPUT_MSG}");
                        }
                    }
                }
                _ => {
                    println!("{FAILED_TO_READ_INPUT_MSG}");
                }
            }
        }
    }

    pub fn print_board(&self) {
        // Print the board
        println!("  0 1 2 3 4 5 6 7 8 9");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{i} ");
            for cell in row {
                print!("{cell} ");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_check_bounds(board: &Board, x: usize, y: usize, expected: bool) {
        assert!(
            board.check_bounds(x, y) == expected,
            "Expected check_bounds to return {} for ({}, {}) when grid_size is {}",
            expected,
            x,
            y,
            board.grid_size
        );
    }

    fn assert_check_bounds_ship(
        board: &Board,
        x: usize,
        y: usize,
        orientation: &Orientation,
        expected: bool,
    ) {
        assert!(
            board.check_bounds_ship(x, y, orientation) == expected,
            "Expected check_bounds_ship to return {} for ship ({}, {}) ({}) when grid_size is {}",
            expected,
            x,
            y,
            orientation,
            board.grid_size
        );
    }

    #[test]
    fn test_check_bounds() {
        let board = Board::new(10, 2);

        assert_check_bounds(&board, 0, 0, true);
        assert_check_bounds(&board, 5, 5, true);
        assert_check_bounds(&board, 9, 9, true);
        assert_check_bounds(&board, 10, 10, false);
        assert_check_bounds(&board, 10, 5, false);
        assert_check_bounds(&board, 5, 10, false);
    }

    #[test]
    fn test_check_bounds_ship() {
        let board = Board::new(10, 2);

        assert_check_bounds_ship(&board, 0, 0, &Orientation::Horizontal, true);
        assert_check_bounds_ship(&board, 0, 0, &Orientation::Vertical, true);
        assert_check_bounds_ship(&board, 8, 9, &Orientation::Horizontal, true);
        assert_check_bounds_ship(&board, 9, 8, &Orientation::Vertical, true);
        assert_check_bounds_ship(&board, 9, 9, &Orientation::Horizontal, false);
        assert_check_bounds_ship(&board, 9, 8, &Orientation::Horizontal, false);
        assert_check_bounds_ship(&board, 9, 9, &Orientation::Vertical, false);
        assert_check_bounds_ship(&board, 8, 9, &Orientation::Vertical, false);
    }

    #[test]
    fn test_place_ship() {
        let mut board = Board::new(10, 10);

        assert!(board.get_remaining_ships_count() == 0);

        board.place_ship(0, 0, Orientation::Horizontal, '1');

        assert!(board.get_remaining_ships_count() == 1);
        assert!(board.get_ship_by_id('1').is_some());
    }
}
