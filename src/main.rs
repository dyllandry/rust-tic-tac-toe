mod game;
mod user_input;

use crate::game::*;
use crate::user_input::*;

fn main() {
    let mut game = TicTacToe::new();

    loop {
        game.render();
        if !game.game_over() {
            if let Some(input) = get_user_input() {
                game.input(input)
            }
        }
    }
}
