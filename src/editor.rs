use std::io::Error;
use std::cmp::min;
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

mod terminal;
use terminal::{Position, Size, Terminal};

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
}

impl Editor {
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
                x = 0;
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
            Self::draw_rows()?;

            Terminal::set_cursor(Position {
                x: self.location.x,
                y: self.location.y,
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
    // e.g. addition with results over MAX_VALUE, return MAX_VALUE
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