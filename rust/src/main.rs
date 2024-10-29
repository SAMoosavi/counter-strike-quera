pub mod cli;
pub mod game;
pub mod game_time;
pub mod gun;
pub mod player;
pub mod setting;
pub mod team;
pub mod tui;

use game::Game;
use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum, Debug)]
enum Mod {
    Tui,
    Cli,
}

#[derive(Parser, Debug)]
#[command(name = "")]
struct Cli {
    #[arg(long = "mod", short = 'm', default_value = "tui")]
    mode: Mod,
}

fn main() {
    let args = Cli::parse();
    let mut game = Game::new().unwrap();

    match args.mode {
        Mod::Tui => tui::run(&mut game),
        Mod::Cli => cli::run(&mut game),
    }
}
