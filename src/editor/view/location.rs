use crate::editor::terminal::Position;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Into<Position> for Location {
    fn into(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}