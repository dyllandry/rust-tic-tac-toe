mod game;
mod user_input;

use crate::game::*;
use crate::user_input::*;

fn main() {
    let mut game = TicTacToe::new();

    while !game.game_over() {
        println!("{}", game);
        if let Some(input) = get_user_input() {
            game.input(input)
        }
        if let Some(winning_player) = game.winning_player() {
            println!("{} won!", winning_player);
        }
    }

    // TODO: Here's the vision
    // let mut game = TicTacToe::new();
    // while (!game.is_over) {
    //     let input = get_user_input();
    //     game.input(input);
    // }
}
