use crate::game::Game;
use crate::tui::{GameCommand, GameCommandHandler, GameEvent, Log};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};
use regex::Regex;

pub struct CommandNoneHandler {
    selected: usize,
    commands: Vec<&'static str>,
    commands_filtered: Vec<&'static str>,
    search: String,
}

impl CommandNoneHandler {
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
impl GameCommandHandler for CommandNoneHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
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

        Option::None
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
