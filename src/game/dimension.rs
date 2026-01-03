#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub struct Dimension {
    w: u16,
    h: u16
}

impl Dimension {
    pub fn new(width: u16, height: u16) -> Dimension {
        Dimension { w: width, h: height }
    }
    pub fn set(&mut self, width: u16, height: u16) {
        self.w = width;
        self.h = height;
    }
    pub fn get(&self) -> (u16,u16) {
        (self.w,self.h)
    }
    pub fn get_width(&self) -> u16 {
        self.w
    }
    pub fn get_height(&self) -> u16 {
        self.h
    }
}
