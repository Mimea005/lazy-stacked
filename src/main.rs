// - stg series log

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::BufRead;
use std::process;
use std::io::stdout;

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        let event = event::read()?;

        let mut text = String::new();
        match event {
            Event::Key(key) => {
                if key.code == KeyCode::Char('l') {
                    let out = process::Command::new("stg").arg("series").output()?;
                    text = out
                        .stdout
                        .lines()
                        .map(Result::unwrap)
                        .map(|line, | format!("- {line}"))
                        .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line))
                }
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }

        let _ = term.draw(|frame| {
            frame.render_widget(
                Paragraph::new(text).block(Block::default().title("series").borders(Borders::ALL))
                , frame.size())
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
