use crate::editor::terminal::Position;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Location> for Position {
    fn from(loc: Location) -> Self {
        Self {
            x: loc.x,
            y: loc.y,
        }
    }
}