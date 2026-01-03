// part of the Minesweeper game for the terminal
use crossterm::{cursor, event, style, terminal, tty::IsTty, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Stdout, Write};

pub struct Term {
    stdout: Stdout,
    width: u16,
    height: u16,
}

#[allow(unused)]
impl Term {

    pub fn new() -> Term {
        let (w,h) = terminal::size().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            (0,0)
        });
        Term {
            stdout: stdout(),
            width: w,
            height: h,
        }
    }

    pub fn is_tty(&self) -> bool {
        self.stdout.is_tty()
    }

    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }

    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }

    pub fn enable_mouse_events(&mut self) {
        self.stdout.execute(event::EnableMouseCapture).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }

    pub fn disable_mouse_events(&mut self) {
        self.stdout.execute(event::DisableMouseCapture).unwrap();
    }

    pub fn enable_focus_events(&mut self) {
        self.stdout.execute(event::EnableFocusChange).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }

    pub fn disable_focus_events(&mut self) {
        self.stdout.execute(event::DisableFocusChange).unwrap();
    }

    pub fn hide_cursor(&mut self) {
        self.stdout.execute(cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        self.stdout.execute(cursor::Show).unwrap();
    }

    pub fn update(&mut self) {
        self.stdout.flush().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
        });
    }

    pub fn get_size(&self) -> (u16,u16) {
        let (w,h) = terminal::size().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            (0,0)
        });
        (w,h)
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn cls(&mut self) {
        self.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn clear_line(&mut self, line_no: u16) {
        self.move_xy(1, line_no);
        self.stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
    }

    pub fn print(&mut self, text: &str) {
        self.stdout.queue(style::Print(text)).unwrap();
    }

    pub fn move_xy(&mut self, x: u16, y: u16) {
        self.stdout.queue(cursor::MoveTo(x, y)).unwrap();
    }

    pub fn print_xy(&mut self, x: u16, y: u16, text: &str) {
        self.move_xy(x, y);
        self.print(text);
    }

    pub fn print_box(&mut self, x_pos: u16, y_pos: u16, width: u16, height: u16) {
        let mut h_line = String::new();
        for _ in 0..width {
            h_line.push('─');
        }
        for y in y_pos..y_pos + height {
            for x in x_pos..x_pos + width {
                if x == x_pos && y == y_pos {
                    self.print_xy(x, y, "┌");
                } else if x == x_pos + width - 1 && y == y_pos {
                    self.print_xy(x, y, "┐");
                } else if x == x_pos && y == y_pos + height - 1 {
                    self.print_xy(x, y, "└");
                } else if x == x_pos + width - 1 && y == y_pos + height - 1 {
                    self.print_xy(x, y, "┘");
                } else if x == x_pos || x == x_pos + width - 1 {
                    self.print_xy(x, y, "│");
                } else if y == y_pos || y == y_pos + height - 1 {
                    self.print_xy(x, y, "─");
                } else {
                    self.print_xy(x, y, " ");
                }
            }
        }
    }

}
