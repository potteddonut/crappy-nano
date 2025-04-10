use std::panic::{set_hook, take_hook};
use std::io::Error;
use std::cmp::min;
use crossterm::event::{read, Event::{self}, KeyCode, KeyEvent, KeyModifiers};

mod terminal;
use terminal::{Position, Size, Terminal};

mod view;
use view::View;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        // on error, terminate crappy-nano instance
        // set custom panic hook
        let currenthook = take_hook();

        // use boxes to ensure valid error handling for the lifetime of the program
        set_hook(Box::new(move |err| {
            let _ = Terminal::terminate();
            currenthook(err);
        }));

        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(filepath) = args.get(1) {
            view.load(filepath);
        }

        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })

    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit { break; }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    // update pointer location in document
    // on every screen refresh called by REPL we move the cursor to pointer location
    fn move_pointer(&mut self, keycode: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match keycode {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                // take either MAX_HEIGHT or actual position
                y = min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },
            KeyCode::Right => {
                // take either MAX_WIDTH or actual position
                x = min(width.saturating_sub(1), x.saturating_add(1));
            },
            KeyCode::PageUp => {
                y = 0;
            },
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            },
            KeyCode::Home => {
                x = 0;
            },
            KeyCode::End => {
                x = width.saturating_sub(1);
            },
            _ => (),
        }
        self.location = Location { x, y };
    }

    fn evaluate_event(&mut self, event: Event) {    
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) => {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    },
                    ( 
                        KeyCode::Up |
                        KeyCode::Down |
                        KeyCode::Left |
                        KeyCode::Right |
                        KeyCode::PageDown |
                        KeyCode::PageUp |
                        KeyCode::End |
                        KeyCode:: Home,
                        _, 
                    ) => {
                       self.move_pointer(code);
                    }
                    _ => {},
                }
            },
            Event::Resize(col_u16, row_u16) => {
                let col = col_u16 as usize;
                let row = row_u16 as usize;
                self.view.resize(Size {
                    height: row,
                    width: col,
                });
            },
            _ => {}
        }
    }

    // called by the REPL loop
    // if the exit shortcut is not triggered, re-print left-column of '~'
    fn refresh_screen(&mut self) {
        let _ = Terminal::hidecursor();
        let _ = Terminal::set_cursor(Position::default());

        self.view.render();

        let _ = Terminal::set_cursor(Position {
            x: self.location.x,
            y: self.location.y,
        });

        let _ = Terminal::showcursor();
        let _ = Terminal::execute();
    }

}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();

        // if self.should_quit {
        //     let _ = Terminal::print("bye nerd\r\n");
        // }
    }
}