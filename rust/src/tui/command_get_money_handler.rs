use crate::{
    game::{Game, TeamId},
    game_time::GameTime,
    tui::{GameCommandHandler, GameEvent, Log},
};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};
use std::{cmp::PartialEq, str::FromStr};

#[derive(PartialEq, Eq)]
enum State {
    EnterName,
    EnterTime,
    Send,
}

impl State {
    pub fn next(&self) -> Self {
        match self {
            State::EnterName => State::EnterTime,
            State::EnterTime => State::Send,
            State::Send => State::EnterName,
        }
    }
}

pub struct CommandGetMoneyHandler {
    name: String,
    status: State,
    time: String,
}

impl CommandGetMoneyHandler {
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            status: State::EnterName,
            time: "".to_string(),
        }
    }

    fn pop(&mut self) {
        match self.status {
            State::EnterName if !self.name.is_empty() => {
                self.name.pop();
            }
            State::EnterTime if !self.time.is_empty() => {
                self.time.pop();
            }
            _ => {}
        };
    }
    fn push(&mut self, c: char) {
        match self.status {
            State::EnterName => {
                self.name.push(c);
            }
            State::EnterTime if (c.is_ascii_digit() || c == ':') => {
                self.time.push(c);
            }
            _ => {}
        };
    }
}

impl GameCommandHandler for CommandGetMoneyHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
        let title_bottom = match self.status {
            State::EnterName => " type name ",
            State::EnterTime => " type time ",
            State::Send => " send ",
        }
        .to_uppercase();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Get Money Of Player ").right_aligned())
            .title_bottom(Line::from(title_bottom).centered().bold().red())
            .border_type(BorderType::Rounded);

        let name = ListItem::new(match &self.status {
            State::EnterName => Line::from_iter([
                Span::from("please enter name: ").bold().yellow(),
                Span::from(&self.name).white(),
            ]),
            _ => Line::from_iter([
                Span::from("name: ").bold().yellow(),
                Span::from(&self.name).yellow(),
            ]),
        });

        let time = ListItem::new(match &self.status {
            State::EnterTime => Line::from_iter([
                Span::from("please enter time: ").bold().yellow(),
                Span::from(&self.time).white(),
            ]),
            _ => Line::from_iter([
                Span::from("time: ").bold().yellow(),
                Span::from(&self.time).yellow(),
            ]),
        });

        let list = List::new([name, time]).block(block);

        frame.render_widget(list, rect);

        let mut log = None;
        if self.status == State::Send {
            log = match GameTime::from_str(&self.time[..]) {
                Ok(time) => match game.get_money_of_player(&self.name[..], &time) {
                    Ok(num) => Some(Log::Result(format!("{}'s money: {num}", self.name))),
                    Err(str) => Some(Log::Error(format!("Get Money Of Player: {str}"))),
                },
                Err(str) => Some(Log::Error(format!("Get Money Of Player: {str}"))),
            };
            self.status = State::EnterName;
        }
        log
    }

    fn event_handler(&mut self, event: Event) -> GameEvent {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Backspace => self.pop(),
                    KeyCode::Enter => self.status = self.status.next(),
                    KeyCode::Delete => self.pop(),
                    KeyCode::Char(x) => self.push(x),
                    KeyCode::Null | KeyCode::Esc => return GameEvent::Back,
                    _ => {}
                };
            }
            _ => {}
        };
        GameEvent::None
    }
}
