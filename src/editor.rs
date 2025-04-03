use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};

mod terminal;
use terminal::{Position, Terminal};

use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false, 
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }

    // called by the REPL loop
    // if the exit shortcut is not triggered, re-print left-column of '~'
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hidecursor()?;

        if self.should_quit {
            // Terminal::clear_screen()?;
            Terminal::print("bye nerd\r\n")?;
        } else {
            Self::draw_rows()?;

            Terminal::set_cursor(Position {
                x: 0,
                y: 0
            })?;
        }

        Terminal::showcursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    // construct left-column of '~' based on window height
    fn draw_rows() -> Result<(), Error>{ 
        let height = Terminal::size()?.height;

        // height of 10 rows = print ~ for row 0-9
        for row in 0..height {
            Terminal::clear_current_line()?;
            
            #[allow(clippy::integer_division)]
            if row == height / 3 {
                Self::display_name()?;
            } else {
                Self::draw_empty_row()?;
            }

            // bottom of terminal window
            if (row.saturating_add(1)) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    // saturating operations return valid value closest to correct value
    // e.g. for addition with results over MAX_VALUE, return MAX_VALUE
    fn display_name() -> Result<(), Error> {
        let mut title = format!("{NAME} -- {VERSION}");

        let width = Terminal::size()?.width;
        let len = title.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        title = format!("~{spaces}{title}");
        title.truncate(width);
        Terminal::print(&title)?;
        
        Ok(())
    }
}