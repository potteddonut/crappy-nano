use crossterm::cursor::{self, MoveTo};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};

use std::io::{ Write, stdout, Error };

pub struct Terminal;

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;

        Self::clear_screen()?;
        Self::set_cursor(Position {
            x: 0,
            y: 0,
        })?;

        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn set_cursor(pos: Position) -> Result<(), Error> {
        // reset cursor to top left
        queue!(stdout(), MoveTo(pos.x, pos.y))
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))
    }

    pub fn clear_current_line() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::CurrentLine))
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (x, y) = size()?;
        Ok(Size {
            width: x,
            height: y,
        })
    }

    pub fn hidecursor() -> Result<(), Error> {
        queue!(stdout(), cursor::Hide)
    }

    pub fn showcursor() -> Result<(), Error> {
        queue!(stdout(), cursor::Show)
    }

    pub fn execute() -> Result<(), Error> {
        // ensure all pending writes are going out
        stdout().flush()?;
        Ok(())
    }
}