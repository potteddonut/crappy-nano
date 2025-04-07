use std::io::Error;
use std::cmp::min;
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyModifiers};

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

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.parse_args();
        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    // retrieve filepath from args
    fn parse_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filepath) = args.get(1) {
            self.view.load(filepath);
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    // update pointer location in document
    // on every screen refresh called by REPL we move the cursor to pointer location
    fn move_pointer(&mut self, keycode: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

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
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {        
        if let Key(KeyEvent {
            code,
            modifiers,
            ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Left |
                KeyCode::Right |
                KeyCode::PageDown |
                KeyCode::PageUp |
                KeyCode::End |
                KeyCode:: Home => {
                    self.move_pointer(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    // called by the REPL loop
    // if the exit shortcut is not triggered, re-print left-column of '~'
    // 
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hidecursor()?;
        Terminal::set_cursor(Position::default())?;

        if self.should_quit {
            // Terminal::clear_screen()?;
            Terminal::print("bye nerd\r\n")?;
        } else {
            self.view.render()?;
            Terminal::set_cursor(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }

        Terminal::showcursor()?;
        Terminal::execute()?;
        Ok(())
    }

}