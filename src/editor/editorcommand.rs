use super::{terminal::Size, Editor};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, Event};
use std::convert::TryFrom;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                    (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                    (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                    (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                    (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                    (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                    (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                    (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                    (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                    _ => Err(format!("Keycode not supported: {code:?}")),
                }
            },
            Event::Resize(col_u16, row_u16) => {
                let width = col_u16 as usize;
                let height = row_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}