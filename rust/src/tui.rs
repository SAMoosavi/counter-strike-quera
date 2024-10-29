mod command_add_user_handler;
mod command_get_health_handler;
mod command_get_money_handler;
mod command_none_handler;
mod command_score_board_handler;

use std::io;

use crate::game::Game;
use command_add_user_handler::CommandAddUserHandler;
use command_get_health_handler::CommandGetHealthHandler;
use command_get_money_handler::CommandGetMoneyHandler;
use command_none_handler::CommandNoneHandler;
use command_score_board_handler::CommandScoreBoardHandler;

use ratatui::{
    crossterm::event::{self, Event},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem},
    DefaultTerminal, Frame,
};

enum GameEvent {
    Back,
    ChangeState(String),
    None,
}

trait GameCommandHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log>;
    fn event_handler(&mut self, event: Event) -> GameEvent;
}

enum GameCommand {
    AddUser(CommandAddUserHandler),
    GetMoney(CommandGetMoneyHandler),
    GetHealth(CommandGetHealthHandler),
    Tap,
    Buy,
    ScoreBoard(CommandScoreBoardHandler),
    None(CommandNoneHandler),
}

impl GameCommand {
    pub fn get_commands_name() -> Vec<&'static str> {
        vec![
            "add-user",
            "get-money",
            "get-health",
            "tap",
            "buy",
            "score-board",
        ]
    }
}

pub fn run(game: &mut Game) {
    let mut terminal = ratatui::init();
    App::new(game).run(&mut terminal).unwrap();
    ratatui::restore();
}

enum Log {
    Result(String),
    Error(String),
}

struct App<'a> {
    game: &'a mut Game,
    exit: bool,
    logs: Vec<Log>,
    state: GameCommand,
}

impl<'a> App<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self {
            game,
            exit: false,
            logs: vec![],
            state: GameCommand::None(CommandNoneHandler::default()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(frame.area());

                self.show_work(frame, layout[0]);
                self.show_log(frame, layout[1]);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn show_log(&mut self, frame: &mut Frame, rect: Rect) {
        let mut lines: Vec<ListItem> = vec![];
        for log in &self.logs {
            let message = match log {
                Log::Result(message) => Text::from(message.clone()).style(Style::new().green()),
                Log::Error(message) => Text::from(message.clone()).style(Style::new().red()),
            };
            lines.push(ListItem::new(message));
        }

        if lines.is_empty() {
            lines.push(ListItem::new(""));
        }

        let list = List::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Game Log")
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(list, rect);
    }

    fn show_work(&mut self, frame: &mut Frame, rect: Rect) {
        // TODO: use get_handler
        let handler: &mut dyn GameCommandHandler = match &mut self.state {
            GameCommand::None(none) => none,
            GameCommand::ScoreBoard(scoreboard) => scoreboard,
            GameCommand::AddUser(add_user) => add_user,
            GameCommand::GetMoney(get_money) => get_money,
            GameCommand::GetHealth(get_health) => get_health,
            _ => todo!(),
        };

        if let Some(log) = handler.run(frame, rect, self.game) {
            self.logs.push(log);
            self.game_event_handler(GameEvent::Back);
        }
    }

    fn get_handler(&mut self) -> &mut dyn GameCommandHandler {
        match &mut self.state {
            GameCommand::None(none) => none,
            GameCommand::ScoreBoard(scoreboard) => scoreboard,
            GameCommand::AddUser(add_user) => add_user,
            GameCommand::GetMoney(get_money) => get_money,
            GameCommand::GetHealth(get_health) => get_health,
            _ => todo!(),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let handler = self.get_handler();
        let event = handler.event_handler(event::read()?);
        self.game_event_handler(event);
        Ok(())
    }

    fn game_event_handler(&mut self, event: GameEvent) {
        match event {
            GameEvent::Back => self.state = GameCommand::None(CommandNoneHandler::default()),
            GameEvent::ChangeState(state) => {
                self.state = match &state[..] {
                    "add-user" => GameCommand::AddUser(CommandAddUserHandler::default()),
                    "get-money" => GameCommand::GetMoney(CommandGetMoneyHandler::default()),
                    "get-health" => GameCommand::GetHealth(CommandGetHealthHandler::default()),
                    "tap" => GameCommand::Tap,
                    "buy" => GameCommand::Buy,
                    "score-board" => GameCommand::ScoreBoard(CommandScoreBoardHandler::default()),
                    "none" => GameCommand::None(CommandNoneHandler::default()),
                    _ => panic!("Invalid state: {}", state),
                }
            }
            GameEvent::None => {}
        }
    }
}
