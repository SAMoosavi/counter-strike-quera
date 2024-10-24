pub mod cli;
pub mod tui;
pub mod game;
pub mod game_time;
pub mod gun;
pub mod player;
pub mod setting;
pub mod team;

use game::Game;

fn main() {
    let mut game = Game::new().unwrap();

    // cli::run(&mut game);
    tui::run(&mut game);
}
