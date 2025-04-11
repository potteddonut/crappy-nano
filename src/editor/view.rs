use std::cmp::min;

use super::terminal::{ Terminal, Size };
use super::Editor;

mod buffer;
use buffer::Buffer;
use crossterm::event::KeyCode;

use crate::editor::NAME;
use crate::editor::VERSION;
use crate::editor::Location;

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

    fn render_line(at: usize, text: &str) {
        let result = Terminal::print_row(at, text);
        debug_assert!(result.is_ok(), "Failed to render line at {at}");
    }

    pub fn render(&mut self) {
        // redraw checking
        if !self.need_redraw {
            return;
        }

        // if line length > window width, truncate string
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let y_centre = height / 3;

        for row in 0..height {

            if let Some(line) = self.buffer.lines.get(row) {
                let truncated_line: &str = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };

                Self::render_line(row, truncated_line);
            } else if row == y_centre && self.buffer.is_empty() {
                // render welcome
                Self::render_line(row, &Self::build_welcome_message(width));
            } else {
                // render ~ for empty
                Self::render_line(row, "~");
            }
        }
        self.need_redraw = false;
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

    // update pointer location in document
    // on every screen refresh we move the cursor to pointer location
    pub fn move_pointer(editor: &mut Editor, keycode: KeyCode) {
        let Location { mut x, mut y } = editor.location;
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
        editor.location = Location { x, y };
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