// part of the Minesweeper game for the terminal
use std::fmt;

#[derive(Copy,Clone,Debug,Default,PartialEq)]
enum CellContent {
    Bomb,
    #[default]
    Empty
}

#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum CellMarker {
    GuessBomb,
    HasBomb,
    #[default]
    None,
}

#[derive(Copy,Clone,Debug,Default)]
pub struct Cell {
    content: CellContent,
    state: CellMarker,
    is_revealed: bool,
    bombs_around: u8,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();
        match self.content {
            CellContent::Bomb => text.push_str("Bomb"),
            CellContent::Empty => text.push_str("Empty")
        }
        if self.bombs_around > 0 {
            text += &format!(", {} bombs around", self.bombs_around);
        }
        if self.is_revealed{
            text.push_str(" (revealed)");
        }
        write!(f, "{}", text)
    }
}

impl Cell {

    pub fn new() -> Cell {
        Cell::default()
    }

    pub fn clear(&mut self) {
        self.content = CellContent::Empty;
        self.state = CellMarker::None;
        self.is_revealed = false;
        self.bombs_around = 0;
    }

    pub fn set_content_bomb(&mut self) {
        self.content = CellContent::Bomb;
    }

    pub fn set_state(&mut self, mark: CellMarker) {
        self.state = mark;
    }

    pub fn set_bombs_around(&mut self, bombs: u8) {
        self.bombs_around = bombs;
    }

    pub fn set_revealed(&mut self, state: bool) {
        self.is_revealed = state;
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn is_bomb(&self) -> bool {
        if self.content == CellContent::Bomb {
            return true;
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        if self.content == CellContent::Empty {
            return true;
        }
        false
    }

    pub fn get_bombs_around(&self) -> u8 {
        self.bombs_around
    }

    pub fn get_state(&self) -> CellMarker {
        self.state
    }

    pub fn get_gfx(&self) -> String {
        let gfx: String;
        if self.is_revealed {
            match self.content {
                CellContent::Bomb => gfx = String::from("\x1b[31;40m•"),
                CellContent::Empty => {
                    match self.bombs_around {
                        0 => gfx = String::from("\x1b[30;40m "),
                        1 => gfx = format!("\x1b[94;40m{}", self.bombs_around),
                        2 => gfx = format!("\x1b[96;40m{}", self.bombs_around),
                        3 => gfx = format!("\x1b[93;40m{}", self.bombs_around),
                        4..8 => gfx = format!("\x1b[91;40m{}", self.bombs_around),
                        _ => gfx = String::from("\x1b[91;40m?")
                    }
                }
            }
        } else {
            match self.state {
                CellMarker::GuessBomb => gfx = String::from("\x1b[93;100m?"),
                CellMarker::HasBomb => gfx = String::from("\x1b[91;100m⚑"),
                _ => gfx = String::from("\x1b[97;100m■")
            }
        }
        gfx
    }

}
