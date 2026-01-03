use crate::game::board::{Board,BoardSize};
use crate::game::dimension::Dimension;
use crate::game::position::Position;
use crate::game::GAME_NAME;
use crossterm::terminal;

/// an enum to the games condition
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum GameState {
    GiveUp,
    Loose,
    Rules,
    Running,
    #[default]
    Start,
    Win,
}

/// the structure for the game screens often used vars
#[derive(Clone,Debug)]
pub struct Game {
    term_size: Dimension,
    title_bar: String,
    board: Board,
    board_difficult: BoardSize,
    board_pos: Position,
    board_size: Dimension,
    seconds_text: &'static str,
    pub seconds: u64,
    seconds_pos: Position,
    bombs_text: &'static str,
    bombs_pos: Position,
    state: GameState,
    pub pause: bool,
    pub update: bool,
    pub debug_mode: bool,
}

impl Game {

    pub fn new(board_size: BoardSize) -> Game {
        let (w,h) = terminal::size().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            (0,0)
        });
        let new_board = Board::new(board_size, false);
        let bw = new_board.get_width();
        let bh = new_board.get_height();
        let mut text = String::from("\x1b[97;100m");
        for _ in 0..w {
            text.push(' ');
        }
        let x1 = (w as usize - GAME_NAME.len()) / 2;
        let x2 = x1 + GAME_NAME.len();
        text.replace_range(x1.., GAME_NAME);
        text.push_str("\x1b[0m");
        Game {
            term_size: Dimension::new(w, h),
            title_bar: text,
            board: new_board,
            board_difficult: board_size,
            board_pos: Position::new((w - bw) / 2, (h - bh) / 2),
            board_size: Dimension::new(bw, bh),
            seconds_text: "Time: ",
            seconds: 0,
            seconds_pos: Position::new(w / 2 - 15, (h - bh) / 2 - 2),
            bombs_text: "Bombs left: ",
            bombs_pos: Position::new(w / 2 + 5, (h - bh) / 2 - 2),
            state: GameState::Start,
            pause: false,
            update: false,
            debug_mode: false
        }
    }

    pub fn reset_board(&mut self) {
        self.board.clear();
        self.board.populate_cells();
    }

    pub fn get_title(&self) -> &str {
        &self.title_bar
    }

    pub fn get_seconds_text(&self) -> String {
        format!("{}\x1b[32m{:8}\x1b[0m", self.seconds_text, self.get_formated_seconds())
    }

    pub fn get_bombs_text(&self) -> String {
        format!("{}\x1b[32m{:2}\x1b[0m", self.bombs_text, self.get_flagged_bombs())
    }

    pub fn get_formated_seconds(&self) -> String {
        let mut secs = self.seconds;
        if secs < 60 {
            format!("0:{:02}", secs)
        } else if secs < 3600 {
            let m = secs / 60;
            secs -= m * 60;
            format!("{}:{:02}", m, secs)
        } else {
            let h = secs / 3600;
            secs -= h * 3600;
            let m = secs / 60;
            secs -= m * 60;
            format!("{}:{:02}:{:02}", h, m, secs)
        }
    }

    pub fn get_term_width(&self) -> u16 {
        self.term_size.get_width()
    }

    pub fn get_term_height(&self) -> u16 {
        self.term_size.get_height()
    }

    pub fn set_gamestate(&mut self, game_state: GameState) {
        self.state = game_state;
    }

    pub fn get_gamestate(&self) -> GameState {
        self.state
    }

    pub fn get_board_x(&self) -> u16 {
        self.board_pos.get_x()
    }

    pub fn get_board_y(&self) -> u16 {
        self.board_pos.get_y()
    }

    pub fn get_board_width(&self) -> u16 {
        self.board_size.get_width()
    }

    pub fn get_board_height(&self) -> u16 {
        self.board_size.get_height()
    }

    pub fn get_board_gfx(&mut self) -> Vec<String> {
        if self.debug_mode {
            self.board.dbg_gfx()
        } else {
            self.board.get_gfx()
        }
    }

    pub fn get_seconds_x(&self) -> u16 {
        self.seconds_pos.get_x()
    }

    pub fn get_seconds_y(&self) -> u16 {
        self.seconds_pos.get_y()
    }

    pub fn get_bombs_x(&self) -> u16 {
        self.bombs_pos.get_x()
    }

    pub fn get_bombs_y(&self) -> u16 {
        self.bombs_pos.get_y()
    }

    pub fn get_flagged_bombs(&self) -> u16 {
        self.board.check_correct_flagged_bombs()
    }

    pub fn pick_board_cell(&mut self, x: i16, y: i16) -> bool {
        self.board.pick_cell(x as usize, y as usize)
    }

    pub fn mark_board_cell(&mut self, x: i16, y: i16) {
        self.board.mark_cell(x as usize, y as usize);
    }

    pub fn check_win_condition(&self) -> bool {
        self.board.check_win_condition()
    }
}
