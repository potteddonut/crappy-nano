use crossterm::cursor::{self, MoveTo};
use crossterm::{ queue, Command };
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};

use core::fmt::Display;
use std::io::{ Write, stdout, Error };

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_current_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn print(string: impl Display) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (x_u16, y_u16) = size()?;

        #[allow(clippy::as_conversions)]
        let width = x_u16 as usize;
        #[allow(clippy::as_conversions)]
        let height = y_u16 as usize;

        Ok(Size { height, width })
    }

    pub fn hidecursor() -> Result<(), Error> {
        Self::queue_command(cursor::Hide)
    }

    pub fn showcursor() -> Result<(), Error> {
        Self::queue_command(cursor::Show)
    }

    // truncated to u16::MAX if pos.x or pos.y is larger
    #[allow(clippy::cast_possible_truncation)]
    pub fn set_cursor(pos: Position) -> Result<(), Error> {
        // reset cursor to top left
        #[allow(clippy::as_conversions)]
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))
    }

    pub fn execute() -> Result<(), Error> {
        // ensure all pending writes are going out
        stdout().flush()?;
        Ok(())
    }

    // offloading queue! macro calls to a helper function
    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}