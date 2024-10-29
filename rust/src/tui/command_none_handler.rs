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

#[derive(PartialEq, Eq)]
enum State {
    EnterNumberOfRound,
    EnterNumberOfAct,
    EnterCommand,
    FinishGame,
}

pub struct CommandNoneHandler {
    selected: usize,
    commands: Vec<&'static str>,
    commands_filtered: Vec<&'static str>,
    search: String,

    state: State,

    round_number: String,
    current_round_number: u8,
    number_of_act: String,
    current_act_number: u8,
}

impl CommandNoneHandler {
    pub fn default() -> Self {
        Self {
            selected: 0,
            commands: GameCommand::get_commands_name(),
            commands_filtered: GameCommand::get_commands_name(),
            search: "".to_owned(),
            state: State::EnterNumberOfRound,
            round_number: "".to_owned(),
            current_round_number: 1,
            number_of_act: "".to_owned(),
            current_act_number: 0,
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
        if self.state == State::EnterCommand {
            self.selected = 0;
        }
    }

    fn select_next(&mut self) {
        if self.state == State::EnterCommand {
            self.selected = if !self.commands_filtered.is_empty()
                && self.commands_filtered.len() - 1 != self.selected
            {
                self.selected + 1
            } else {
                self.selected
            };
        }
    }

    fn select_prev(&mut self) {
        if self.state == State::EnterCommand {
            self.selected = if self.selected != 0 {
                self.selected - 1
            } else {
                0
            };
        }
    }

    fn select_first(&mut self) {
        if self.state == State::EnterCommand {
            self.selected = 0;
        }
    }

    fn select_last(&mut self) {
        if self.state == State::EnterCommand {
            self.selected = if self.commands_filtered.is_empty() {
                0
            } else {
                self.commands_filtered.len() - 1
            };
        }
    }

    fn push(&mut self, c: char) {
        match self.state {
            State::EnterNumberOfRound if c.is_ascii_digit() => {
                self.round_number.push(c);
            }

            State::EnterNumberOfAct if c.is_ascii_digit() => {
                self.number_of_act.push(c);
            }

            State::EnterCommand => {
                self.reset_selected();
                self.search.push(c);
                self.filter_commands();
            }
            _ => {}
        }
    }

    fn pop(&mut self) {
        match self.state {
            State::EnterNumberOfRound => {
                self.round_number.pop();
            }
            State::EnterNumberOfAct => {
                self.number_of_act.pop();
            }
            State::EnterCommand if !self.search.is_empty() => {
                self.reset_selected();
                self.search.pop();
                self.filter_commands();
            }
            _ => {}
        }
    }

    fn enter(&mut self) -> GameEvent {
        match self.state {
            State::EnterNumberOfRound => {
                self.state = State::EnterNumberOfAct;
                GameEvent::None
            }
            State::EnterNumberOfAct => {
                self.current_act_number = 1;
                self.state = State::EnterCommand;
                GameEvent::None
            }
            State::EnterCommand => {
                self.current_act_number += 1;
                GameEvent::ChangeState(self.commands_filtered[self.selected].to_string())
            }
            _ => GameEvent::None,
        }
    }
}
impl GameCommandHandler for CommandNoneHandler {
    fn run(&mut self, frame: &mut Frame, rect: Rect, game: &mut Game) -> Option<Log> {
        let mut log = Option::None;
        if self.state == State::EnterCommand
            && self.current_act_number > self.number_of_act.parse::<u8>().unwrap()
        {
            self.current_round_number += 1;
            self.state = State::EnterNumberOfAct;
            self.number_of_act.clear();
            log = Some(Log::Result(game.end_of_round().to_string()));
        }

        if self.state != State::EnterNumberOfRound
            && self.current_round_number > self.round_number.parse::<u8>().unwrap()
        {
            self.state = State::FinishGame;
        }

        let block = match self.state {
            State::FinishGame => Block::default().borders(Borders::ALL).title(
                Line::from(" Bay, I hope, you are enjoyed! ")
                    .centered()
                    .green(),
            ),
            State::EnterNumberOfRound => Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" Hi, Welcome ").centered()),
            State::EnterNumberOfAct => Block::default()
                .borders(Borders::ALL)
                .title(Line::from(" New Round ").centered()),
            State::EnterCommand => Block::default()
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
                .title(
                    Line::from_iter([
                        Span::from(" Round: "),
                        Span::from(self.current_round_number.to_string()).fg(Color::Red),
                        Span::from("/"),
                        Span::from(&self.round_number),
                        Span::from(" "),
                        Span::from(" Action: "),
                        Span::from(self.current_act_number.to_string()).fg(Color::Red),
                        Span::from("/"),
                        Span::from(&self.number_of_act),
                        Span::from(" "),
                    ])
                    .left_aligned(),
                )
                .title_bottom(
                    Line::from(" type to search command ".to_uppercase())
                        .left_aligned()
                        .bold()
                        .red(),
                )
                .title_bottom(
                    Line::from(" use ↑↓ for change command ".to_uppercase())
                        .right_aligned()
                        .bold()
                        .red(),
                )
                .border_type(BorderType::Rounded),
        };

        let lines = match self.state {
            State::FinishGame => vec![],
            State::EnterNumberOfRound => vec![ListItem::new(Line::from_iter([
                Span::from("please enter number of round in current game: ").yellow(),
                Span::from(&self.round_number).white(),
            ]))],
            State::EnterNumberOfAct => vec![ListItem::new(Line::from_iter([
                Span::from("please enter number of act in current round: ").yellow(),
                Span::from(&self.number_of_act).white(),
            ]))],
            State::EnterCommand => self
                .commands_filtered
                .iter()
                .enumerate()
                .map(|(index, command)| {
                    ListItem::new(Line::from(*command).style(if self.selected != index {
                        Style::new().yellow()
                    } else {
                        Style::new().black().bg(Color::Yellow)
                    }))
                })
                .collect(),
        };

        let list = List::new(lines).block(block);

        frame.render_widget(list, rect);

        log
    }

    fn event_handler(&mut self, event: Event) -> GameEvent {
        if self.state == State::FinishGame {
            return GameEvent::Finish;
        }
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Backspace => self.pop(),
                    KeyCode::Enter => return self.enter(),
                    KeyCode::Up => self.select_prev(),
                    KeyCode::Down => self.select_next(),
                    KeyCode::Home => self.select_first(),
                    KeyCode::End => self.select_last(),
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
