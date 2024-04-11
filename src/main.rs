// - stg series
// - stg log

use core::panic;
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
    pub key_history: Vec<KeyCode>,
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
                Event::Key(KeyEvent {code, ..}) => app.state.key_history.push(code),
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

    frame.render_widget(stg_series_widget(), main_layout[0]);
    frame.render_widget(key_widget(&state.key_history), right_column[0]);
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
