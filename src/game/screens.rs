use crate::game::GAME_NAME;
use crate::game::board::BoardSize;
use crate::game::game_struct::{Game,GameState};
use crate::term::Term;
use crossterm::event;

/// this is the screen for the game start, the user selects a board size here
pub fn draw_start_screen(term: &mut Term) -> BoardSize {

    term.cls();
    term.print_xy(1, 1, format!("Welcome to {} !", GAME_NAME).as_str());

    term.print_xy(1, 3, "How difficult should be your minefield?");
    term.print_xy(1, 5, "1) \x1b[32ms\x1b[0mmall minefield");
    term.print_xy(1, 6, "2) \x1b[32mm\x1b[0medium minefield");
    term.print_xy(1, 7, "3) \x1b[32ml\x1b[0marge minefield");

    term.print_xy(1, 9, "Select your size or press ENTER. The default difficulty is \x1b[1msmall\x1b[0m.");
    term.update();

    let mut board_size = BoardSize::default();

    loop {

        match event::read().unwrap() {
            event::Event::Key(event) => {
                match event.code {
                    event::KeyCode::Char('1') | event::KeyCode::Char('s') => {
                        board_size = BoardSize::Small;
                        break;
                    }
                    event::KeyCode::Char('2') | event::KeyCode::Char('m') => {
                        board_size = BoardSize::Medium;
                        break;
                    }
                    event::KeyCode::Char('3') | event::KeyCode::Char('l') => {
                        board_size = BoardSize::Large;
                        break;
                    }
                    event::KeyCode::Char('q') => std::process::exit(0),
                    event::KeyCode::Enter => break,
                    _ => {}
                }
            },
            event::Event::FocusGained => {},
            event::Event::FocusLost => {},
            _ => {}

        }

    }
    term.update();
    board_size
}

/// this is the main screen to show the minefield
pub fn draw_main_screen(term: &mut Term, game: &Game, board_vec: Vec<String>) {

    term.cls();
    term.print_xy(0, 0, game.get_title());

    term.print_box(
        game.get_board_x() - 1,
        game.get_board_y() - 1,
        game.get_board_width() + 2,
        game.get_board_height() + 2
    );

    term.print_xy(game.get_bombs_x(), game.get_bombs_y(), &game.get_bombs_text());

    let mut x_pos = game.get_board_x();
    let mut y_pos = game.get_board_y();
    for line in board_vec {
        term.print_xy(x_pos, y_pos, line.as_str());
        y_pos += 1;
    }

    let rules_text = "\x1b[32;100m F1 \x1b[97;100m Rules \x1b[0m";
    let quit_text =  "\x1b[32;100m F10\x1b[97;100m Quit  \x1b[0m";
    x_pos = game.get_term_width() / 2;
    term.print_xy(x_pos - 15, y_pos + 1, rules_text);
    term.print_xy(x_pos + 5, y_pos + 1, quit_text);

    term.update();

}

/// show the rules
pub fn draw_rules_screen(term: &mut Term, game: &Game) {

    term.cls();
    term.print_xy(0, 0, game.get_title());

    let x_pos = 2;
    let mut y_pos = 1;

    term.print_xy(x_pos, y_pos + 1, "What is Minesweeper?");
    term.print_xy(x_pos, y_pos + 2, "--------------------");
    term.print_xy(x_pos, y_pos + 3, "Minesweeper is a game where mines are hidden in a grid of squares. Safe squares");
    term.print_xy(x_pos, y_pos + 4, "have numbers telling you how many mines touch the square. You can use the number");
    term.print_xy(x_pos, y_pos + 5, "clues to solve the game by opening all of the safe squares. If you click on a");
    term.print_xy(x_pos, y_pos + 6, "mine you lose the game!");

    y_pos = 8;
    term.print_xy(x_pos, y_pos + 1, "How to play?");
    term.print_xy(x_pos, y_pos + 2, "------------");
    term.print_xy(x_pos, y_pos + 3, "You open squares with the left mouse button and put flags on mines with the");
    term.print_xy(x_pos, y_pos + 4, "right mouse button. Pressing the right mouse button again changes your flag into");
    term.print_xy(x_pos, y_pos + 5, "a questionmark. When you open a square that does not touch any mines, it will be");
    term.print_xy(x_pos, y_pos + 6, "empty and the adjacent squares will automatically open in all directions until");
    term.print_xy(x_pos, y_pos + 7, "reaching squares that contain numbers.");

    y_pos = 17;
    term.print_xy(x_pos, y_pos + 1, "End of game");
    term.print_xy(x_pos, y_pos + 2, "-----------");
    term.print_xy(x_pos, y_pos + 3, "You won the game when all mines are correctly flagged. Otherwise, if you click");
    term.print_xy(x_pos, y_pos + 4, "a field with a bomb to open it you loose.");

    y_pos = 23;
    term.print_xy(x_pos, y_pos + 1, "Have fun and good luck!");

    term.print_xy(x_pos, y_pos + 3, "Press \x1b[32me\x1b[0m to exit to game.");

    term.update();

}

/// end of game screen
pub fn draw_end_screen(term: &mut Term, game: &Game) {

    let end_msg = match game.get_gamestate() {
        GameState::Loose => "Boom, you lost!",
        GameState::Win => "Congratulations, you won!",
        _ => "",
    };

    let mut x = (game.get_term_width() - (end_msg.len() as u16)) / 2;
    let mut y = game.get_term_height() / 2 - 1;

    term.print_box(x-1, y-1, (end_msg.len() as u16)+2, 3);
    term.print_xy(x, y, end_msg);

    let rpl_msg = "Do you want to \x1b[32ms\x1b[0mtart a new game or to \x1b[32mq\x1b[0muit?";
    x = (game.get_term_width() - (rpl_msg.len() as u16 - 22)) / 2;
    y = (game.get_board_y() + game.get_board_height()) + 3;
    term.print_xy(x, y, rpl_msg);

    term.update();

}
