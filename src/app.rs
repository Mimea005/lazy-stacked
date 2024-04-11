use std::io::stdout;
use std::mem::ManuallyDrop;
use std::panic::{self, PanicInfo};
use std::thread;

use crossterm::cursor::Show;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};

pub struct App<State> {
    pub state: State,
    panic_hook: ManuallyDrop<Box<dyn Fn(&PanicInfo) + Send + Sync>>,
}

impl<State> App<State> {
    pub fn new(state: State) -> App<State> {
        let original_hook = panic::take_hook();

        let app = unsafe {
            App {
                state,
                panic_hook: std::mem::transmute_copy(&original_hook),
            }
        };

        std::panic::set_hook(Box::new(move |panic| {
            reset_terminal();
            original_hook(panic);
        }));

        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        app
    }
}

impl<State: Default> Default for App<State> {
    fn default() -> Self {
        App::new(State::default())
    }
}

impl<State> Drop for App<State> {
    fn drop(&mut self) {
        reset_terminal();

        if thread::panicking() {
            return;
        }

        let original_hook: Box<dyn Fn(&PanicInfo) + Send + Sync> =
            unsafe { std::mem::transmute_copy(&mut self.panic_hook) };
        panic::set_hook(Box::new(move |panic| original_hook(panic)))
    }
}

fn reset_terminal() {
    let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    let _ = disable_raw_mode();
}
