use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    DefaultTerminal, Frame,
};
use regex::Regex;

use crate::{
    game::{Game, TeamId},
    game_time::GameTime,
    gun::TypeOfGun,
};

enum GameEvent {
    Back,
    ChangeState(String),
    None,
}


trait GameCommandHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect);
    fn event_handler(&mut self, event: Event) -> GameEvent;
}

struct GameCommandNone {
    selected: usize,
    commands: Vec<&'static str>,
    commands_filtered: Vec<&'static str>,
    search: String,
}

impl GameCommandNone {
    pub fn default() -> Self {
        Self {
            selected: 0,
            commands: GameCommand::get_commands_name(),
            commands_filtered: GameCommand::get_commands_name(),
            search: "".to_string(),
        }
    }

    fn filter_commands(&mut self) {
        let search = Regex::new(&self.search).unwrap();

        self.commands_filtered = self
            .commands
            .iter()
            .filter(|x| search.is_match(x))
            .cloned()
            .collect();
    }

    fn reset_selected(&mut self) {
        self.selected = 0;
    }

    fn select_next(&mut self) {
        self.selected = if !self.commands_filtered.is_empty()
            && self.commands_filtered.len() - 1 != self.selected
        {
            self.selected + 1
        } else {
            self.selected
        };
    }

    fn select_prev(&mut self) {
        self.selected = if self.selected != 0 {
            self.selected - 1
        } else {
            0
        };
    }

    fn select_first(&mut self) {
        self.selected = 0;
    }

    fn select_last(&mut self) {
        self.selected = if self.commands_filtered.is_empty() {
            0
        } else {
            self.commands_filtered.len() - 1
        };
    }

    fn push_search(&mut self, c: char) {
        self.reset_selected();
        self.search.push(c);
        self.filter_commands();
    }

    fn pop_search(&mut self) {
        if !self.search.is_empty() {
            self.reset_selected();
            self.search.pop();
            self.filter_commands();
        }
    }
}
impl GameCommandHandler for GameCommandNone {
    fn run(&mut self, frame: &mut Frame, rect: Rect) {
        let mut lines: Vec<ListItem> = vec![];

        for (index, command) in self.commands_filtered.iter().enumerate() {
            lines.push(ListItem::new(Line::from(*command).style(
                if self.selected != index {
                    Style::new().yellow()
                } else {
                    Style::new().black().bg(Color::Yellow)
                },
            )));
        }

        let list = List::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" Game Command ").right_aligned())
                .title(
                    Line::from_iter([
                        Span::from(" Search Command: "),
                        Span::from(&self.search).fg(Color::Red),
                        Span::from(" "),
                    ])
                        .centered(),
                )
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(list, rect);
    }

    fn event_handler(&mut self, event: Event) -> GameEvent {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Backspace => self.pop_search(),
                    KeyCode::Enter => {
                        return GameEvent::ChangeState(
                            self.commands_filtered[self.selected].to_string(),
                        )
                    }
                    KeyCode::Up => self.select_prev(),
                    KeyCode::Down => self.select_next(),
                    KeyCode::Home => self.select_first(),
                    KeyCode::End => self.select_last(),
                    KeyCode::Delete => self.pop_search(),
                    KeyCode::Char(x) => self.push_search(x),
                    KeyCode::Null | KeyCode::Esc => return GameEvent::Back,
                    _ => {}
                };
            }
            _ => {}
        };
        GameEvent::None
    }
}

enum GameCommand {
    AddUser,
    GetMoney,
    GetHealth,
    Tap,
    Buy,
    ScoreBoard,
    None(GameCommandNone),
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
            state: GameCommand::None(GameCommandNone::default()),
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
                Log::Result(message) => Line::from(message.clone()).style(Style::new().green()),
                Log::Error(message) => Line::from(message.clone()).style(Style::new().red()),
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
        match &mut self.state {
            GameCommand::None(none) => none.run(frame, rect),
            _ => todo!(),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let event = match &mut self.state {
            GameCommand::None(none) => none.event_handler(event::read()?),
            _ => todo!(),
        };
        self.game_event_handler(event);
        Ok(())
    }

    fn game_event_handler(&mut self, event: GameEvent) {
        match event {
            GameEvent::Back => { self.state = GameCommand::None(GameCommandNone::default()); }
            GameEvent::ChangeState(state) => self.state = match &state[..] {
                "add-user" => GameCommand::AddUser,
                "get-money" => GameCommand::GetMoney,
                "get-health" => GameCommand::GetHealth,
                "tap" => GameCommand::Tap,
                "buy" => GameCommand::Buy,
                "score-board" => GameCommand::ScoreBoard,
                "none" => GameCommand::None(GameCommandNone::default()),
                _ => panic!("Invalid state: {}", state),
            },
            GameEvent::None => {}
        }
    }
}
