// - stg series
// - stg log

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Widget;
use ratatui::Frame;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::stdout;
use std::io::BufRead;
use std::process;

enum Focus {
    Diff,
    Log,
}

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let mut command_list = Vec::new();

    loop {
        term.draw(|frame| ui(frame, &command_list)).unwrap();

        let event = event::read().unwrap();

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                key => command_list.push(key),
            },
            _ => {}
        }
    }

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn ui(frame: &mut Frame, command_list: &Vec<KeyCode>) {
    let main_layout =
        Layout::horizontal(Constraint::from_percentages([50, 50])).split(frame.size());

    let right_column =
        Layout::vertical([Constraint::Max(7), Constraint::Fill(1)]).split(main_layout[1]);

    frame.render_widget(stg_series_widget(), main_layout[0]);
    frame.render_widget(key_widget(command_list), right_column[0]);
    frame.render_widget(stg_log_widget(), right_column[1])
}

fn stg_series_widget() -> impl Widget {
    let text = process::Command::new("stg")
        .arg("series")
        .output()
        .unwrap()
        .stdout
        .lines()
        .map(Result::unwrap)
        .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));

    Paragraph::new(text).block(Block::default().title("series").borders(Borders::ALL))
}

fn stg_log_widget() -> impl Widget {
    let text = process::Command::new("stg")
        .arg("log")
        .output()
        .unwrap()
        .stdout
        .lines()
        .map(Result::unwrap)
        .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
    Paragraph::new(text).block(Block::default().title("Log").borders(Borders::ALL))
}

fn key_widget(command_list: &Vec<KeyCode>) -> impl Widget {
    let text: String = command_list
        .iter()
        .rev()
        .take(5)
        .map(|key| format!("{key:?}\n"))
        .collect();
    Paragraph::new(text).block(Block::default().title("Hist").borders(Borders::ALL))
}
