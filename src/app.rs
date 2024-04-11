use std::io::{stdout, Stdout};
use std::mem::ManuallyDrop;
use std::panic::{self, PanicInfo};

use crossterm::cursor::Show;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, ExecutableCommand};

pub struct App {
    stdout: Stdout,
    panic_hook: ManuallyDrop<Box<dyn Fn(&PanicInfo) + Send + Sync>>,
}

impl App {
    pub fn new() -> App {
        let original_hook = panic::take_hook();

        let mut app = unsafe {
            App {
                stdout: stdout(),
                panic_hook: std::mem::transmute_copy(&original_hook),
            }
        };

        std::panic::set_hook(Box::new(move |panic| {
            App::reset_terminal();
            original_hook(panic);
        }));

        enable_raw_mode().unwrap();
        app.stdout().execute(EnterAlternateScreen).unwrap();
        app
    }

    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }

    fn reset_terminal() {
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

impl Drop for App {
    fn drop(&mut self) {
        App::reset_terminal();
        let original_hook: Box<dyn Fn(&PanicInfo) + Send + Sync> =
            unsafe { std::mem::transmute_copy(&mut self.panic_hook) };
        panic::set_hook(Box::new(move |panic| original_hook(panic)))
    }
}
