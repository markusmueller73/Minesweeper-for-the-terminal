#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub struct Position {
    x: u16,
    y: u16
}

impl Position {
    pub fn new(x_pos: u16, y_pos: u16) -> Position {
        Position { x: x_pos, y: y_pos }
    }
    pub fn set(&mut self, x_pos: u16, y_pos: u16) {
        self.x = x_pos;
        self.y = y_pos;
    }
    pub fn set_x(&mut self, x_pos: u16) {
        self.x = x_pos;
    }
    pub fn set_y(&mut self, y_pos: u16) {
        self.y = y_pos;
    }
    pub fn get(&self) -> (u16,u16) {
        (self.x,self.y)
    }
    pub fn get_x(&self) -> u16 {
        self.x
    }
    pub fn get_y(&self) -> u16 {
        self.y
    }
}
