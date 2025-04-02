use crossterm::cursor::{self, MoveTo};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use std::io::stdout;

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        Self::clear_screen()?;
        Self::set_cursor(0, 0)?;

        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn set_cursor(x: u16, y: u16) -> Result<(), std::io::Error> {
        // reset cursor to top left
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn hidecursor() -> Result<(), std::io::Error> {
        execute!(stdout(), cursor::Hide)
    }

    pub fn showcursor() -> Result<(), std::io::Error> {
        execute!(stdout(), cursor::Show)
    }
}