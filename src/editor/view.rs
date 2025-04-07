use super::terminal::{ Terminal, Size, Position };
use std::io::Error;

mod buffer;
use buffer::Buffer;

use crate::editor::NAME;
use crate::editor::VERSION;

pub struct View {
    buffer: Buffer,
    size: Size,
    need_redraw: bool,
}

impl View {
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.need_redraw = true;
    }

    // method to load file contents into buffer
    pub fn load(&mut self, filepath: &str) {
        if let Ok(buffer) = Buffer::load(filepath) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }

    fn render_line(at: usize, text: &str) -> Result<(), Error> {
        Terminal::set_cursor(Position {
            x: 0,
            y: at
        })?;

        Terminal::clear_line()?;
        Terminal::print(text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        // redraw checking
        if !self.need_redraw {
            return Ok(())
        }

        // if line length > window width, truncate string
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }

        let y_centre = height / 3;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row) {
                let truncated_line: &str;
                if line.len() >= width {
                    truncated_line = &line[0..width];
                } else {
                    truncated_line = line;
                };

                Self::render_line(row, truncated_line)?;
            } else if row == y_centre && self.buffer.is_empty() {
                // render welcome
                Self::render_line(row, &Self::build_welcome_message(width))?;
            } else {
                // render ~ for empty
                Self::render_line(row, "~")?;
            }
        }
        self.need_redraw = false;
        Ok(())
    }

    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let message = format!("{NAME} -- {VERSION}");
        let len = message.len();
        if width <= len {
            return "~".to_string();
        }

        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut final_message = format!("~{}{}", " ".repeat(padding), message);
        final_message.truncate(width);

        final_message
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            need_redraw: true,
        }
    }
}