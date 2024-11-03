mod command_add_user_handler;
mod command_buy_handler;
mod command_get_health_handler;
mod command_get_money_handler;
mod command_none_handler;
mod command_score_board_handler;
mod command_tap_handler;

use crate::game::Game;
use command_add_user_handler::CommandAddUserHandler;
use command_buy_handler::CommandBuyHandler;
use command_get_health_handler::CommandGetHealthHandler;
use command_get_money_handler::CommandGetMoneyHandler;
use command_none_handler::CommandNoneHandler;
use command_score_board_handler::CommandScoreBoardHandler;
use command_tap_handler::CommandTapHandler;
use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
    DefaultTerminal, Frame,
};

enum GameEvent {
    Back,
    ChangeState(String),
    Finish,
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
    Tap(CommandTapHandler),
    Buy(CommandBuyHandler),
    ScoreBoard(CommandScoreBoardHandler),
    None,
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
    none_handler: CommandNoneHandler,
}

impl<'a> App<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self {
            game,
            exit: false,
            logs: vec![],
            state: GameCommand::None,
            none_handler: CommandNoneHandler::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(frame.area());

                while self.show_work(frame, layout[0]) {
                    frame.render_widget(Clear, layout[0]);
                }
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

    fn show_work(&mut self, frame: &mut Frame, rect: Rect) -> bool {
        // TODO: use get_handler
        let handler: &mut dyn GameCommandHandler = match &mut self.state {
            GameCommand::None => &mut self.none_handler,
            GameCommand::ScoreBoard(scoreboard) => scoreboard,
            GameCommand::AddUser(add_user) => add_user,
            GameCommand::GetMoney(get_money) => get_money,
            GameCommand::GetHealth(get_health) => get_health,
            GameCommand::Buy(buy) => buy,
            GameCommand::Tap(tap) => tap,
        };

        if let Some(log) = handler.run(frame, rect, self.game) {
            self.logs.push(log);
            self.game_event_handler(GameEvent::Back);
            true
        } else {
            false
        }
    }

    fn get_handler(&mut self) -> &mut dyn GameCommandHandler {
        match &mut self.state {
            GameCommand::None => &mut self.none_handler,
            GameCommand::ScoreBoard(scoreboard) => scoreboard,
            GameCommand::AddUser(add_user) => add_user,
            GameCommand::GetMoney(get_money) => get_money,
            GameCommand::GetHealth(get_health) => get_health,
            GameCommand::Buy(buy) => buy,
            GameCommand::Tap(tap) => tap,
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let terminal_event = event::read()?;
        if let Event::Key(key) = terminal_event {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                self.exit = true;
            }
        }
        let handler = self.get_handler();
        let game_event = handler.event_handler(terminal_event);

        self.game_event_handler(game_event);
        Ok(())
    }

    fn game_event_handler(&mut self, event: GameEvent) {
        match event {
            GameEvent::Back => self.state = GameCommand::None,
            GameEvent::ChangeState(state) => {
                self.state = match &state[..] {
                    "add-user" => GameCommand::AddUser(CommandAddUserHandler::default()),
                    "get-money" => GameCommand::GetMoney(CommandGetMoneyHandler::default()),
                    "get-health" => GameCommand::GetHealth(CommandGetHealthHandler::default()),
                    "tap" => GameCommand::Tap(CommandTapHandler::default()),
                    "buy" => GameCommand::Buy(CommandBuyHandler::default()),
                    "score-board" => GameCommand::ScoreBoard(CommandScoreBoardHandler::default()),
                    "none" => GameCommand::None,
                    _ => panic!("Invalid state: {}", state),
                }
            }
            GameEvent::Finish => self.exit = true,
            GameEvent::None => {}
        }
    }
}
