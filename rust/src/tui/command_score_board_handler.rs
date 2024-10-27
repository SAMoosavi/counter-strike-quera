use crate::{game::Game, game_time::GameTime};
use crate::tui::{GameCommandHandler, GameEvent, Log};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};
use std::str::FromStr;

pub struct CommandScoreBoardHandler {
    time: String,
    send: bool,
}

impl CommandScoreBoardHandler {
    pub fn default() -> Self {
        Self {
            time: "".to_string(),
            send: false,
        }
    }

    fn push_time(&mut self, c: char) {
        if c.is_ascii_digit() || c == ':' {
            self.time.push(c);
        }
    }

    fn pop_time(&mut self) {
        if !self.time.is_empty() {
            self.time.pop();
        }
    }
}

impl GameCommandHandler for CommandScoreBoardHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Game Score Board ").right_aligned())
            .border_type(BorderType::Rounded);

        let list = List::new([ListItem::new(Line::from_iter([
            Span::from("please enter time: ").bold().yellow(),
            Span::from(&self.time).white(),
        ]))])
            .block(block);

        frame.render_widget(list, rect);

        let mut log = None;

        if self.send {
            match GameTime::from_str(&self.time[..]) {
                Ok(time) => log = Some(Log::Result(game.score_board(&time))),
                Err(str) => log = Some(Log::Error(str)),
            };
            self.send = false;
        }
        log
    }

    fn event_handler(&mut self, event: Event) -> GameEvent {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Backspace => self.pop_time(),
                    KeyCode::Enter => self.send = true,
                    KeyCode::Delete => self.pop_time(),
                    KeyCode::Char(x) => self.push_time(x),
                    KeyCode::Null | KeyCode::Esc => return GameEvent::Back,
                    _ => {}
                };
            }
            _ => {}
        };
        GameEvent::None
    }
}