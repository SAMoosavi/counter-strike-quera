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
    EnterTeamId,
    EnterTime,
    Send,
}

impl State {
    pub fn next(&self) -> Self {
        match self {
            State::EnterName => State::EnterTeamId,
            State::EnterTeamId => State::EnterTime,
            State::EnterTime => State::Send,
            State::Send => State::EnterName,
        }
    }
}

pub struct CommandAddUserHandler {
    name: String,
    status: State,
    time: String,
    team_id: TeamId,
}

impl CommandAddUserHandler {
    pub fn default() -> Self {
        Self {
            name: "".to_string(),
            status: State::EnterName,
            time: "".to_string(),
            team_id: TeamId::Terrorist,
        }
    }

    fn next_team_id(&mut self) {
        if self.status == State::EnterTeamId {
            self.team_id = match self.team_id {
                TeamId::Terrorist => TeamId::CounterTerrorist,
                TeamId::CounterTerrorist => TeamId::Terrorist,
            };
        }
    }
    fn prev_team_id(&mut self) {
        if self.status == State::EnterTeamId {
            self.team_id = match self.team_id {
                TeamId::Terrorist => TeamId::CounterTerrorist,
                TeamId::CounterTerrorist => TeamId::Terrorist,
            };
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

impl GameCommandHandler for CommandAddUserHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
        let title_bottom = match self.status {
            State::EnterName => " type name ",
            State::EnterTeamId => " use ↑↓ for change team ",
            State::EnterTime => " type time ",
            State::Send => " send ",
        }
        .to_uppercase();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Add User ").right_aligned())
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

        let team_id = ListItem::new(match &self.status {
            State::EnterTeamId => Line::from_iter([
                Span::from("please select team: ").bold().yellow(),
                Span::from(self.team_id.to_string()).white(),
            ]),
            _ => Line::from_iter([
                Span::from("team: ").bold().yellow(),
                Span::from(self.team_id.to_string()).yellow(),
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

        let list = List::new([name, team_id, time]).block(block);

        frame.render_widget(list, rect);

        let mut log = None;

        if self.status == State::Send {
            match GameTime::from_str(&self.time[..]) {
                Ok(time) => {
                    log = match game.add_player(self.team_id, &self.name[..], &time) {
                        Ok(str) => Some(Log::Result(str)),
                        Err(str) => Some(Log::Error(str)),
                    }
                }
                Err(str) => log = Some(Log::Error(str)),
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
                    KeyCode::Up => self.next_team_id(),
                    KeyCode::Down => self.prev_team_id(),
                    _ => {}
                };
            }
            _ => {}
        };
        GameEvent::None
    }
}
