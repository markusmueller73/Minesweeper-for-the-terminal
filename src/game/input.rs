use crate::game::game_struct::{Game,GameState};
use crossterm::event;

/// this function catches all user events and process or returns em
pub fn user_input(game: &mut Game) -> GameState {

    let mut current_state = game.get_gamestate();

    // this function did not block the whole terminal for user input
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {

        match event::read().unwrap() {

            // process keyboard events
            event::Event::Key(event) => {
                match event.code {

                    // show the rules screen and stop the timer
                    event::KeyCode::F(1) => {
                        game.set_gamestate(GameState::Rules);
                        game.pause = true;
                    },

                    // exit the loop, to quit the game
                    event::KeyCode::F(10) | event::KeyCode::Char('q') => {
                        if game.get_gamestate() != GameState::Rules {
                            game.set_gamestate(GameState::GiveUp);
                        }
                    },

                    // toggle the debug mode
                    event::KeyCode::F(12) => {
                        // and do it only if the app compiled in debug mode
                        if cfg!(debug_assertions) {
                            game.debug_mode = !game.debug_mode;
                            game.update = true;
                        }
                    },

                    event::KeyCode::Char('e') => {
                        game.set_gamestate(GameState::Running);
                        game.pause = false;
                        game.update = true;
                    },

                    event::KeyCode::Char('p') => {
                        game.pause = !game.pause;
                        if !game.pause {
                            game.update = true;
                        }
                    },

                    event::KeyCode::Char('s') => {
                        if game.get_gamestate() == GameState::Loose || game.get_gamestate() == GameState::Win {
                            game.set_gamestate(GameState::Start);
                        }
                    }

                    _ => {}
                }

                // in raw terminal mode, the CTRL + C ist deactivated, so manually activate it
                if event.modifiers == event::KeyModifiers::CONTROL && event.code == event::KeyCode::Char('c') {
                    std::process::exit(99);
                }

            },

            // process mouse events
            event::Event::Mouse(mouse_event) => {

                let mouse_x = mouse_event.column;
                let mouse_y = mouse_event.row;
                let cell_x = mouse_x as i16 - game.get_board_x() as i16;
                let cell_y = mouse_y as i16 - game.get_board_y() as i16;

                // process left click
                if mouse_event.kind == event::MouseEventKind::Down(event::MouseButton::Left) {
                    // if pick_cell() is true, the player clicked a field with a bomb
                    if game.pick_board_cell(cell_x, cell_y) {
                        game.set_gamestate(GameState::Loose);
                        game.pause = true;
                    }
                    game.update = true;

                // process right click
                } else if mouse_event.kind == event::MouseEventKind::Down(event::MouseButton::Right) {
                    game.mark_board_cell(cell_x, cell_y);
                    game.update = true;
                }



            },

            // check the focus here, to stop the timer
            event::Event::FocusLost => {
                game.pause = true;
            }

            event::Event::FocusGained => {
                game.pause = false;
            }

            _ => {}

        }

    }

    current_state
}
