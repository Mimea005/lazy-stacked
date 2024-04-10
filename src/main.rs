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
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::stdout;
use std::io::BufRead;
use std::process;

fn main() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let mut command_list = Vec::new();

    let main_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(term.size().unwrap());
    let column =
        Layout::vertical([Constraint::Length(5), Constraint::Min(5)]).split(main_layout[1]);

    loop {
        term.draw(|frame| {
            let text = process::Command::new("stg")
                .arg("series")
                .output()
                .unwrap()
                .stdout
                .lines()
                .map(Result::unwrap)
                .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
            frame.render_widget(
                Paragraph::new(text).block(Block::default().title("series").borders(Borders::ALL)),
                main_layout[0],
            );
            let text = command_list
                .iter()
                .map(|ev| format!("{:?}", ev))
                .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
            frame.render_widget(
                Paragraph::new(text).block(Block::default().title("Hist").borders(Borders::ALL)),
                column[0],
            );

            let text = process::Command::new("stg")
                .arg("log")
                .output()
                .unwrap()
                .stdout
                .lines()
                .map(Result::unwrap)
                .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
            frame.render_widget(
                Paragraph::new(text).block(Block::default().title("Log").borders(Borders::ALL)),
                column[1]
            );
        })
        .unwrap();

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
