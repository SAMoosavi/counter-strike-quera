use crate::{
    game::Game,
    game_time::GameTime,
    gun::TypeOfGun,
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
    EnterAttacker,
    EnterAttacked,
    EnterTypeOfGun,
    EnterTime,
    Send,
}

impl State {
    pub fn next(&self) -> Self {
        match self {
            State::EnterAttacker => State::EnterAttacked,
            State::EnterAttacked => State::EnterTypeOfGun,
            State::EnterTypeOfGun => State::EnterTime,
            State::EnterTime => State::Send,
            State::Send => State::EnterAttacker,
        }
    }
}

pub struct CommandTapHandler {
    attacker: String,
    attacked: String,
    status: State,
    time: String,
    gun_type: TypeOfGun,
}

impl CommandTapHandler {
    pub fn default() -> Self {
        Self {
            attacker: "".to_string(),
            attacked: "".to_string(),
            status: State::EnterAttacker,
            time: "".to_string(),
            gun_type: TypeOfGun::Knife,
        }
    }

    fn next_type_of_gun(&mut self) {
        if self.status == State::EnterTypeOfGun {
            self.gun_type = match self.gun_type {
                TypeOfGun::Knife => TypeOfGun::Pistol,
                TypeOfGun::Pistol => TypeOfGun::Heavy,
                TypeOfGun::Heavy => TypeOfGun::Knife,
            };
        }
    }
    fn prev_type_of_gun(&mut self) {
        if self.status == State::EnterTypeOfGun {
            self.gun_type = match self.gun_type {
                TypeOfGun::Knife => TypeOfGun::Heavy,
                TypeOfGun::Pistol => TypeOfGun::Knife,
                TypeOfGun::Heavy => TypeOfGun::Pistol,
            };
        }
    }

    fn pop(&mut self) {
        match self.status {
            State::EnterAttacker => self.attacker.pop(),
            State::EnterAttacked => self.attacked.pop(),
            State::EnterTime => self.time.pop(),
            _ => None,
        };
    }
    fn push(&mut self, c: char) {
        match self.status {
            State::EnterAttacker => self.attacker.push(c),
            State::EnterAttacked => self.attacked.push(c),
            State::EnterTime if (c.is_ascii_digit() || c == ':') => self.time.push(c),

            _ => {}
        };
    }
}

impl GameCommandHandler for CommandTapHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
        let title_bottom = match self.status {
            State::EnterAttacker => " type attacker name ",
            State::EnterAttacked => " type attacked name ",
            State::EnterTypeOfGun => " use ↑↓ for change team ",
            State::EnterTime => " type time ",
            State::Send => " send ",
        }
        .to_uppercase();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Add User ").right_aligned())
            .title_bottom(Line::from(title_bottom).centered().bold().red())
            .border_type(BorderType::Rounded);

        let attacker_name = ListItem::new(match &self.status {
            State::EnterAttacker => Line::from_iter([
                Span::from("please enter attacker name: ").bold().yellow(),
                Span::from(&self.attacker).white(),
            ]),
            _ => Line::from_iter([
                Span::from("attacker name: ").bold().yellow(),
                Span::from(&self.attacker).yellow(),
            ]),
        });

        let attacked_name = ListItem::new(match &self.status {
            State::EnterAttacked => Line::from_iter([
                Span::from("please enter attacked name: ").bold().yellow(),
                Span::from(&self.attacked).white(),
            ]),
            _ => Line::from_iter([
                Span::from("attacked name: ").bold().yellow(),
                Span::from(&self.attacked).yellow(),
            ]),
        });

        let gun_type = ListItem::new(match &self.status {
            State::EnterTypeOfGun => Line::from_iter([
                Span::from("please select type of gun: ").bold().yellow(),
                Span::from(self.gun_type.to_string()).white(),
            ]),
            _ => Line::from_iter([
                Span::from("type of gun: ").bold().yellow(),
                Span::from(self.gun_type.to_string()).yellow(),
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

        let list = List::new([attacker_name, attacked_name, gun_type, time]).block(block);

        frame.render_widget(list, rect);

        let mut log = None;
        if self.status == State::Send {
            log = match GameTime::from_str(&self.time[..]) {
                Ok(time) => match game.tap(
                    &self.attacker[..],
                    &self.attacked[..],
                    &self.gun_type,
                    &time,
                ) {
                    Ok(str) => Some(Log::Result(format!("Tap: {str}"))),
                    Err(str) => Some(Log::Error(format!("Tap: {str}"))),
                },
                Err(str) => Some(Log::Error(format!("Tap: {str}"))),
            };
            self.status = State::EnterAttacker;
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
                    KeyCode::Up => self.next_type_of_gun(),
                    KeyCode::Down => self.prev_type_of_gun(),
                    _ => {}
                };
            }
            _ => {}
        };
        GameEvent::None
    }
}
