mod game;

use crate::game::*;

fn main() {
    let mut game = TicTacToe::new();
    game.mark_cell(1, Player::P1);
    game.mark_cell(0, Player::P2);
    game.mark_cell(2, Player::P1);
    game.mark_cell(4, Player::P2);
    game.mark_cell(5, Player::P1);
    game.mark_cell(8, Player::P2);

    println!("{}", game);

    if let Some(winning_player) = game.winning_player() {
        println!("{} won!", winning_player);
    }
}
