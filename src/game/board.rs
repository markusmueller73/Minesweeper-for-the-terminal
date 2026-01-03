// part of the Minesweeper game for the terminal
use rand::random_range;
use crate::game::cell::{Cell, CellMarker};

const MAX_BOARD_WIDTH: usize = 30;
const MAX_BOARD_HEIGHT: usize = 30;

/// An enum to describe the size of the game board
/// The size of the board is the difficulty of the game too
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum BoardSize {
    #[default]
    Small,
    Medium,
    Large
}

/// A struct to describe the game board
/// The descision goes here to an array instead of a rust vector, it's easier to handle
#[derive(Copy,Clone,Debug)]
pub struct Board {
    width: usize,
    height: usize,
    max_bombs: u16,
    cell: [[Cell; MAX_BOARD_HEIGHT]; MAX_BOARD_WIDTH]
}

impl Board {

    /// Creates a new game board, must be done once at program start
    pub fn new(board_size: BoardSize, difficult: bool) -> Board {
        let (w,h) = match board_size {
            BoardSize::Small => (MAX_BOARD_WIDTH-20,MAX_BOARD_HEIGHT-20),
            BoardSize::Medium => (MAX_BOARD_WIDTH-10,MAX_BOARD_HEIGHT-10),
            BoardSize::Large => (MAX_BOARD_WIDTH,MAX_BOARD_HEIGHT),
        };
        let mut new_board = Board {
            width: w,
            height: h,
            max_bombs: if difficult {
                (w * h / 5) as u16
            } else {
                (w * h / 10) as u16
            },
            cell: [[Cell::new(); MAX_BOARD_HEIGHT]; MAX_BOARD_WIDTH]
        };
        new_board.populate_cells();
        new_board
    }

    /// Clear all cells of the game board and reset them to default values
    pub fn clear(&mut self) {
        for y in 0..MAX_BOARD_HEIGHT {
            for x in 0..MAX_BOARD_WIDTH {
                self.cell[x][y].clear();
            }
        }
    }

    /// Check, if the given cell is a valid cell in the array
    fn is_cell_valid(&self, x: isize, y: isize) -> bool {
        let w = self.width as isize;
        let h = self.height as isize;
        if x >= 0 && x < w && y >= 0 && y < h {
            return true;
        }
        false
    }

    /// Count the bombs around the given cell
    fn count_bombs_around(&self, x_pos: usize, y_pos: usize) -> u8 {
        let mut bombs_around: u8 = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 { continue; }
                let index_x = x_pos as isize + x;
                let index_y = y_pos as isize + y;
                if self.is_cell_valid(index_x, index_y) {
                    let new_x = index_x as usize;
                    let new_y = index_y as usize;
                    if self.cell[new_x][new_y].is_bomb() {
                        bombs_around += 1;
                    }
                }
            }
        }
        bombs_around
    }

    /// Populate all cells of the playfield, with bombs or leave it empty
    /// For any cell the bombs around will be counted too
    pub fn populate_cells(&mut self) {
        let mut bombs: u16 = 0;
        while bombs < self.max_bombs {
            let x: usize = random_range(0..self.width);
            let y: usize = random_range(0..self.height);
            if !self.cell[x][y].is_bomb() {
                self.cell[x][y].set_content_bomb();
                bombs += 1;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                //self.cell[x][y].pick();
                if self.cell[x][y].is_bomb() { continue; }
                let bombs_around = self.count_bombs_around(x, y);
                self.cell[x][y].set_bombs_around(bombs_around);
            }
        }
    }

    /// Get a String of the cell
    pub fn print_cell(&self, x: usize, y: usize) -> String {
        if self.is_cell_valid(x as isize, y as isize) {
            return format!("{}", self.cell[x][y]);
        }
        String::from("No valid cell")
    }

    /// Get the width of the game board
    pub fn get_width(&self) -> u16 {
        self.width as u16
    }

    /// Get the height of the game board
    pub fn get_height(&self) -> u16 {
        self.height as u16
    }

    /// Get the whole game board,
    /// one element in the vector is one horizontal line of the game board.
    /// Use this function to view the game board to the user
    pub fn get_gfx(&self) -> Vec<String> {
        let mut gfx_vec = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                line += self.cell[x][y].get_gfx().as_str();
            }
            line += "\x1b[0m";
            gfx_vec.push(line);
        }
        gfx_vec
    }

    /// Get the whole game board for DEBUG view
    pub fn dbg_gfx(&mut self) -> Vec<String> {
        let mut gfx_vec = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let state = self.cell[x][y].is_revealed();
                self.cell[x][y].set_revealed(true);
                line += self.cell[x][y].get_gfx().as_str();
                self.cell[x][y].set_revealed(state);
            }
            line += "\x1b[0m";
            gfx_vec.push(line);
        }
        gfx_vec
    }

    /// Pick a cell and process with the game logic,
    /// returns TRUE if the user clicked/uncovered a bomb otherwise FALSE
    pub fn pick_cell(&mut self, cell_x: usize, cell_y: usize) -> bool {

        if !self.is_cell_valid(cell_x as isize, cell_y as isize) {
            return false;
        }

        if self.cell[cell_x][cell_y].get_state() != CellMarker::None {
            return false;
        }

        if !self.cell[cell_x][cell_y].is_revealed() {

            self.cell[cell_x][cell_y].set_revealed(true);

            if self.cell[cell_x][cell_y].is_bomb() {
                return true;
            } else {
                if self.cell[cell_x][cell_y].get_bombs_around() > 0 {
                    return false;
                }
                for y in -1..=1 {
                    for x in -1..=1 {
                        if x == 0 && y == 0 { continue }
                        let new_x: isize = cell_x as isize + x;
                        let new_y: isize = cell_y as isize + y;
                        if self.is_cell_valid(new_x, new_y) {
                            self.pick_cell(new_x as usize, new_y as usize);
                        }
                    }
                }
            }
        }

        false

    }

    /// If the user marked (right mouse button) a cell, switch the marker
    pub fn mark_cell(&mut self, x: usize, y: usize) {
        match self.cell[x][y].get_state() {
            CellMarker::GuessBomb => self.cell[x][y].set_state(CellMarker::None),
            CellMarker::HasBomb => self.cell[x][y].set_state(CellMarker::GuessBomb),
            CellMarker::None => self.cell[x][y].set_state(CellMarker::HasBomb),
        }
    }

    /// Count the number of bombs that are marked correctly by the user
    pub fn check_correct_flagged_bombs(&self) -> u16 {
        let mut correct_flag = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell[x][y].get_state() == CellMarker::HasBomb && self.cell[x][y].is_bomb() {
                    correct_flag += 1;
                }
            }
        }
        self.max_bombs - correct_flag
    }

    /// Check here if the user won the game,
    /// when all empty field are revealed and all bombs are correectly marked.
    pub fn check_win_condition(&self) -> bool {
        let mut correct_flag = 0;
        let mut empty_and_covered = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell[x][y].get_state() == CellMarker::HasBomb && self.cell[x][y].is_bomb() {
                    correct_flag += 1;
                }
                if self.cell[x][y].is_empty()  && !self.cell[x][y].is_revealed() {
                    empty_and_covered += 1;
                }
            }
        }
        if self.max_bombs == correct_flag && empty_and_covered == 0 {
            return true;
        }
        false
    }

}
