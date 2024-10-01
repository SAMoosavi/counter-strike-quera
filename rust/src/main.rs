pub mod gun;
pub mod player;

pub mod game;
pub mod game_time;
pub mod setting;
pub mod team;

use crate::game::{Game, TeamId};
use crate::game_time::GameTime;

use std::io;

fn handel(game: &mut Game, query: &Vec<&str>) -> Result<(), String> {
    let command = *query.get(0).unwrap();
    match command {
        "ADD-USER" => {
            let name = *query.get(1).unwrap();
            let team_id = TeamId::to_enum(query.get(2).unwrap())?;
            let time = GameTime::new_from_str(query.get(3).unwrap());
            game.add_player(team_id, name, &time)
        }
        _ => Err(format!("the command {} is not found!", command)),
    }
}

fn main() {
    let mut game = Game::new();

    let mut round = String::new();
    io::stdin().read_line(&mut round).unwrap();
    let round: u8 = round.trim().parse().unwrap();
    for _ in 0..round {
        game.reset();

        let mut act = String::new();
        io::stdin().read_line(&mut act).unwrap();
        let act: Vec<&str> = act.split_whitespace().collect();
        let act: u8 = act[1].trim().parse().unwrap();
        for _ in 0..act {
            let mut query = String::new();
            io::stdin().read_line(&mut query).unwrap();
            let query: Vec<&str> = query.split_whitespace().collect();
            match handel(&mut game, &query) {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            }
        }
    }
}
