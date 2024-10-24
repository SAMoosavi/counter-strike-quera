use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem},
    DefaultTerminal, Frame,
};

use crate::{
    game::{Game, TeamId},
    game_time::GameTime,
    gun::TypeOfGun,
};

struct GameCommandNone {
    selected: usize,
}

impl GameCommandNone {
    pub fn default() -> Self {
        Self { selected: 0 }
    }

    pub fn run(&mut self, frame: &mut Frame, rect: Rect) {
        let mut lines: Vec<ListItem> = vec![];

        for command in GameCommand::get_commands_name() {
            lines.push(ListItem::new(
                Line::from(command).style(Style::new().yellow()),
            ));
        }

        let list = List::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Game Log")
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(list, rect);
    }
}

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
    // No need to annotate the lifetime in the function signature
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
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            Event::Key(_) => todo!(),
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Char(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            KeyCode::CapsLock => todo!(),
            KeyCode::ScrollLock => todo!(),
            KeyCode::NumLock => todo!(),
            KeyCode::PrintScreen => todo!(),
            KeyCode::Pause => todo!(),
            KeyCode::Menu => todo!(),
            KeyCode::KeypadBegin => todo!(),
            KeyCode::Media(media_key_code) => todo!(),
            KeyCode::Modifier(modifier_key_code) => todo!(),
        };
    }
}
