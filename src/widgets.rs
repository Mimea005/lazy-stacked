use std::{io::BufRead, process};
use crossterm::event::KeyEvent;
use ratatui::widgets::*;


pub struct StgSeries;
impl Widget for StgSeries {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text = process::Command::new("stg")
            .arg("series")
            .output()
            .unwrap()
            .stdout
            .lines()
            .map(Result::unwrap)
            .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
        Paragraph::new(text)
            .block(Block::default().title("series").borders(Borders::ALL))
            .render(area, buf);
    }
}

pub struct StgLog;
impl Widget for StgLog {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text = process::Command::new("stg")
            .arg("log")
            .output()
            .unwrap()
            .stdout
            .lines()
            .map(Result::unwrap)
            .fold(String::new(), |acc, line| format!("{:}\n{:}", acc, line));
        Paragraph::new(text)
            .block(Block::default().title("Log").borders(Borders::ALL))
            .render(area, buf)
    }
}

pub struct KeyHistory;
impl StatefulWidget for KeyHistory {
    type State = Vec<KeyEvent>;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let text: String = state
            .iter()
            .rev()
            .take(5)
            .map(
                |KeyEvent {
                     code, modifiers, ..
                 }| format!("{modifiers:?} {code:?}\n"),
            )
            .collect();
        Paragraph::new(text)
            .block(Block::default().title("Hist").borders(Borders::ALL))
            .render(area, buf)
    }
}
