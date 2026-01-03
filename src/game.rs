// part of the Minesweeper game for the terminal
mod board;
mod cell;
mod dimension;
mod game_struct;
mod input;
mod position;
mod screens;

use crate::game::{
    game_struct::{Game,GameState},
    input::user_input,
    screens::*
};
use crate::term::Term;

/// the name of the game ;-)
const GAME_NAME: &str = "M I N E S W E E P E R";

/// the main function to process the game and the user input
pub fn run() -> Result<(), i32> {

    // create handle to Stdout and init crossterm-terminal
    let mut term = Term::new();
    term.enable_raw_mode();
    term.enable_focus_events();
    term.enable_mouse_events();
    term.hide_cursor();
    term.cls();

    // create game struct and draw the start screen, the player select the size of the board here
    let mut game = Game::new(draw_start_screen(&mut term));
    let mut gfx = game.get_board_gfx();

    // thats the main function to draw the board to the terminal
    draw_main_screen(&mut term, &game, gfx);

    // init the timer and print it to the screen, this is independed of the main screen
    let mut timer_start = std::time::Instant::now();
    term.print_xy(game.get_seconds_x(), game.get_seconds_y(), &game.get_seconds_text());
    term.update();

    game.set_gamestate(GameState::Running);

    // main game loop
    loop {

        // get the user input here
        user_input(&mut game);

        if game.check_win_condition() && game.get_gamestate() == GameState::Running {
            game.set_gamestate(GameState::Win);
            game.pause = true;
        }

        // update timer only, if the game has the focus
        if !game.pause {
            let now = std::time::Instant::now();
            if now.duration_since(timer_start).as_millis() >= 1_000 {
                game.seconds += 1;
                timer_start = now;
            }
            term.print_xy(game.get_seconds_x(), game.get_seconds_y(), &game.get_seconds_text());
            term.update();
        }

        // if the game screen should be updated, repaint the whole terminal
        if game.update {
            // select here the board to view
            gfx = game.get_board_gfx();
            draw_main_screen(&mut term, &game, gfx);
            game.update = false;
        }

        // now select here the ending condition
        match game.get_gamestate() {

            GameState::GiveUp => break,

            GameState::Loose | GameState::Win => {
                term.print_xy(game.get_seconds_x(), game.get_seconds_y(), &game.get_seconds_text());
                draw_end_screen(&mut term, &game);
                game.update = false;
            }

            GameState::Start => {
                game.pause = false;
                game.set_gamestate(GameState::Running);
                game.seconds = 0;
                game.reset_board();
                game.update = true;
            }

            _ => {}
        }

    }

    // clean up the crossterm terminal functions
    term.update();
    term.cls();
    term.show_cursor();

    term.disable_mouse_events();
    term.disable_focus_events();

    term.disable_raw_mode();

    // and leave the game
    Ok(())
}
