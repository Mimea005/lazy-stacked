// - stg series
// - stg log

use core::panic;
use anyhow::Result;
use std::io::BufRead;
use std::process;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io::stdout;

mod app;
use app::App;

#[derive(Default)]
struct State {
    pub key_history: Vec<KeyEvent>,
}

fn main() {
    let mut late_panic = false;
    {
        let stdout = stdout();
        let mut app = App::<State>::default();
        let mut term = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

        loop {
            term.draw(|frame| ui(frame, &mut app.state)).unwrap();

            let event = event::read().unwrap();

            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers,
                    ..
                }) => {
                    if modifiers.intersects(KeyModifiers::CONTROL) {
                        late_panic = true;
                        break;
                    } else {
                        panic!("Manually triggered panic!")
                    }
                }
                Event::Key(key) => app.state.key_history.push(key),
                _ => {}
            }
        }
    }
    println!("Exited normally");

    if late_panic {
        panic!("A late panic!")
    }
}

fn ui(frame: &mut Frame, state: &mut State) {
    let main_layout =
        Layout::horizontal(Constraint::from_percentages([50, 50])).split(frame.size());

    let right_column =
        Layout::vertical([Constraint::Max(7), Constraint::Fill(1)]).split(main_layout[1]);

    frame.render_widget(StgSeries, main_layout[0]);
    frame.render_stateful_widget(KeyHistory, right_column[0], &mut state.key_history);
    frame.render_widget(StgLog, right_column[1])
}

struct StgSeries;

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

struct StgLog;
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

struct KeyHistory;
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
        Paragraph::new(text).block(Block::default().title("Hist").borders(Borders::ALL)).render(area, buf)
    }
}
