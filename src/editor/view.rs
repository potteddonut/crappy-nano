use super::terminal::{ Terminal, Size };
use std::io::Error;

mod buffer;
use buffer::Buffer;

use crate::editor::NAME;
use crate::editor::VERSION;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    // method to load file contents into buffer
    pub fn load(&mut self, filepath: &str) {
        if let Ok(buffer) = Buffer::load(filepath) {
            self.buffer = buffer;
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome()?;
        } else {
            self.render_buffer()?;
        }

        Ok(())
    }

    pub fn render_welcome() -> Result<(), Error>{ 
        let height = Terminal::size()?.height;

        // height of 10 rows = print ~ for row 0-9
        for row in 0..height {
            Terminal::clear_line()?;
            
            // welcome message insertion logic
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

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }

            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
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