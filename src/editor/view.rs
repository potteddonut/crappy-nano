use super::terminal::{Size, Terminal};
use std::io::Error;

use crate::editor::NAME;
use crate::editor::VERSION;

pub struct View;

impl View {
    // construct left-column of '~' based on window height
    pub fn render() -> Result<(), Error>{ 
        let height = Terminal::size()?.height;

        Terminal::clear_current_line()?;
        Terminal::print("hello world!\r\n")?;

        // height of 10 rows = print ~ for row 0-9
        for row in 1..height {
            Terminal::clear_current_line()?;
            
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