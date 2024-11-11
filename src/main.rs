mod board;
mod game;
mod words;

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
