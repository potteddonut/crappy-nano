use std::cmp;

use super::terminal::{ Position, Size, Terminal };

mod buffer;
use buffer::Buffer;

mod location;
use location::Location;

use super::editorcommand::{Direction, EditorCommand};

use crate::editor::NAME;
use crate::editor::VERSION;

pub struct View {
    buffer: Buffer,
    size: Size,
    need_redraw: bool,
    location: Location,
    scroll_offset: Location,
}

impl View {
    pub fn getposition(&self) -> Position {
        self.location.into()
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_into_view();
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
        let top = self.scroll_offset.y;

        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                
                // nasty horizontal scrolling fix
                let str = String::from(line);
                let range = left..right;
                let start = range.start;
                let end = cmp::min(range.end, str.len());

                Self::render_line(row, str.get(start..end).unwrap_or_default());
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

    fn scroll_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // vertical scrolling
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            // cursor goes beyond window bounds
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        // horizontal scrolling
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            // cursor goes beyond window bounds
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.need_redraw = offset_changed;
    }

    // update pointer location in document
    // on every screen refresh we move the cursor to pointer location
    pub fn move_pointer(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            },
            Direction::Down => {
                // take either MAX_HEIGHT or actual position
                // y = min(height.saturating_sub(1), y.saturating_add(1));
                y = y.saturating_add(1);
            },
            Direction::Left => {
                x = x.saturating_sub(1);
            },
            Direction::Right => {
                // take either MAX_WIDTH or actual position
                // x = min(width.saturating_sub(1), x.saturating_add(1));
                x = x.saturating_add(1);
            },
            Direction::PageUp => {
                y = 0;
            },
            Direction::PageDown => {
                y = height.saturating_sub(1);
            },
            Direction::Home => {
                x = 0;
            },
            Direction::End => {
                x = width.saturating_sub(1);
            },
            _ => (),
        }
        self.location = Location { x, y };
        self.scroll_into_view();
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_pointer(&direction),
            EditorCommand::Quit => {},
        }
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            need_redraw: true,
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}