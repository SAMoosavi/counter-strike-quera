use crate::{
    game::{Game, TeamId},
    game_time::GameTime,
    gun::TypeOfGun,
};
use clap::Parser;
use clap_repl::{
    reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory},
    ClapEditor,
};
use std::process::exit;

#[derive(Debug, Parser)]
#[command(name = "")]
struct NumberOfRoundCommand {
    n: u8,
}

#[derive(Debug, Parser)]
#[command(name = "")]
enum RoundCommand {
    Round { n: u8 },
}

#[derive(Debug, Parser)]
#[command(name = "")]
enum GameCommand {
    AddUser {
        name: String,
        team_id: TeamId,
        time: GameTime,
    },
    GetMoney {
        name: String,
        time: GameTime,
    },
    GetHealth {
        name: String,
        time: GameTime,
    },
    Tap {
        attacker: String,
        attacked: String,
        gun_type: TypeOfGun,
        time: GameTime,
    },
    Buy {
        player: String,
        gun: String,
        time: GameTime,
    },
    ScoreBoard {
        time: GameTime,
    },
}

fn game_handler(game: &mut Game, command: GameCommand) -> Result<String, String> {
    match command {
        GameCommand::AddUser {
            name,
            team_id,
            time,
        } => game.add_player(team_id, &name, &time),
        GameCommand::GetMoney { name, time } => {
            Ok(game.get_money_of_player(&name, &time)?.to_string())
        }
        GameCommand::GetHealth { name, time } => {
            Ok(game.get_health_of_player(&name, &time)?.to_string())
        }
        GameCommand::Tap {
            attacker,
            attacked,
            gun_type,
            time,
        } => game.tap(&attacker, &attacked, &gun_type, &time),
        GameCommand::Buy { player, gun, time } => game.buy(&player, &gun, &time),
        GameCommand::ScoreBoard { time } => Ok(game.score_board(&time)),
    }
}

fn clap_editor_builder<T: Parser + Send + Sync + 'static>(prompt: DefaultPrompt) -> ClapEditor<T> {
    ClapEditor::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(1000, "/tmp/clap-repl-simple-example-history".into())
                    .unwrap(),
            ))
        })
        .build()
}

pub fn run(game: &mut Game) {
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("CS_game".to_owned()),
        DefaultPromptSegment::Basic("".to_owned()),
    );

    let mut rl_game: ClapEditor<GameCommand> = clap_editor_builder(prompt.clone());
    let mut rl_round: ClapEditor<RoundCommand> = clap_editor_builder(prompt.clone());
    let mut rl_number_of_round: ClapEditor<NumberOfRoundCommand> = clap_editor_builder(prompt);

    let number_of_round = match rl_number_of_round.read_command() {
        clap_repl::ReadCommandOutput::Command(command) => command.n,
        clap_repl::ReadCommandOutput::CtrlC => {
            println!("bay!");
            exit(0)
        }
        clap_repl::ReadCommandOutput::EmptyLine => panic!(),
        clap_repl::ReadCommandOutput::ClapError(error) => panic!("{}", error),
        clap_repl::ReadCommandOutput::ShlexError => panic!(),
        clap_repl::ReadCommandOutput::ReedlineError(error) => panic!("{}", error),
        clap_repl::ReadCommandOutput::CtrlD => panic!(),
    };

    for _ in 0..number_of_round {
        let number_of_act = match rl_round.read_command() {
            clap_repl::ReadCommandOutput::Command(command) => match command {
                RoundCommand::Round { n } => n,
            },
            clap_repl::ReadCommandOutput::CtrlC => {
                println!("bay!");
                exit(0)
            }
            clap_repl::ReadCommandOutput::EmptyLine => panic!(),
            clap_repl::ReadCommandOutput::ClapError(error) => panic!("{}", error),
            clap_repl::ReadCommandOutput::ShlexError => panic!(),
            clap_repl::ReadCommandOutput::ReedlineError(error) => panic!("{}", error),
            clap_repl::ReadCommandOutput::CtrlD => panic!(),
        };

        let mut act = 0;
        while act < number_of_act {
            match rl_game.read_command() {
                clap_repl::ReadCommandOutput::Command(command) => {
                    match game_handler(game, command) {
                        Ok(ans) => {
                            if !ans.is_empty() {
                                println!("{}", ans);
                            }
                        }
                        Err(err) => println!("{}", err),
                    };
                    act += 1;
                }
                clap_repl::ReadCommandOutput::CtrlC => {
                    println!("bay!");
                    exit(0)
                }
                clap_repl::ReadCommandOutput::EmptyLine => println!(),
                clap_repl::ReadCommandOutput::ClapError(error) => println!("{}", error),
                clap_repl::ReadCommandOutput::ShlexError => println!(),
                clap_repl::ReadCommandOutput::ReedlineError(error) => println!("{}", error),
                clap_repl::ReadCommandOutput::CtrlD => println!(),
            };
        }
        println!("{}", game.end_of_round());
    }
}
