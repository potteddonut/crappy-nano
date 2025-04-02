use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};

mod terminal;
use terminal::Terminal;

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

    fn repl(&mut self) -> Result<(), std::io::Error> {
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
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hidecursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            print!("bye nerd\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::set_cursor(0, 0)?;
        }

        Terminal::showcursor()?;
        Ok(())
    }

    // construct left-column of '~' based on window height
    fn draw_rows() -> Result<(), std::io::Error>{ 
        let height = Terminal::size()?.1;

        // height of 10 rows = print ~ for row 0-9
        for row in 0..height {
            print!("~");

            // bottom of terminal window
            if (row +1) < height {
                print!("\r\n")
            }
        }

        Ok(())
    }
}