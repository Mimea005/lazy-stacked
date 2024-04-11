// - stg series
// - stg log

use core::panic;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use std::io::stdout;

mod app;
mod widgets;
use app::App;
use widgets::*;

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
