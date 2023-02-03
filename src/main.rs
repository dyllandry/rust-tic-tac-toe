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

struct TicTacToe {
    board: Board,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
        }
    }

    pub fn mark_cell(&mut self, cell: usize, player: Player) {
        self.board.cells[cell].player = Some(player);
    }

    pub fn is_players_turn(&self, player: Player) -> bool {
        let filled_cells = self
            .board
            .cells
            .iter()
            .map(|cell| cell.player)
            .filter_map(std::convert::identity)
            .count();
        let is_p1_turn = (filled_cells % 2) == 0;
        match player {
            Player::P1 => is_p1_turn,
            Player::P2 => !is_p1_turn,
        }
    }

    pub fn winning_player(&self) -> Option<Player> {
        fn player_won(board: &Board, player_to_check: Player) -> bool {
            let possible_winning_lines: [[usize; 3]; 8] = [
                [0, 1, 2], // top row
                [3, 4, 5], // middle row
                [6, 7, 8], // bottom row
                [0, 3, 6], // left column
                [1, 4, 7], // middle column
                [2, 5, 8], // right column
                [0, 4, 8], // diagonal top left to bottom right
                [2, 4, 6], // diagonal top right to bottom left
            ];
            possible_winning_lines.iter().any(|line| {
                line.iter().all(|cell| match board.cells[*cell].player {
                    Some(player_in_cell) => player_in_cell == player_to_check,
                    _ => false,
                })
            })
        }
        if player_won(&self.board, Player::P1) {
            Some(Player::P1)
        } else if player_won(&self.board, Player::P2) {
            Some(Player::P2)
        } else {
            None
        }
    }
}

impl std::fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)
    }
}

struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Self {
        let mut cells: [Cell; 9] = [Cell {
            player: None,
            index: 0,
        }; 9];
        for i in 0..9 {
            cells[i].index = i;
        }
        return Board { cells };
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} | {} | {}",
            self.cells[0], self.cells[1], self.cells[2]
        )
        .unwrap();
        writeln!(f, "---------",).unwrap();
        writeln!(
            f,
            "{} | {} | {}",
            self.cells[3], self.cells[4], self.cells[5]
        )
        .unwrap();
        writeln!(f, "---------",).unwrap();
        writeln!(
            f,
            "{} | {} | {}",
            self.cells[6], self.cells[7], self.cells[8]
        )
    }
}

#[derive(Clone, Copy)]
struct Cell {
    player: Option<Player>,
    index: usize,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.player {
            None => format!("{}", self.index + 1),
            Some(player) => match player {
                Player::P1 => "x".into(),
                Player::P2 => "o".into(),
            },
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    P1,
    P2,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_number = match self {
            Player::P1 => 1,
            Player::P2 => 2,
        };
        write!(f, "Player {}", player_number)
    }
}

#[cfg(test)]
mod tests {
    mod is_players_turn_tests {
        use crate::*;

        #[test]
        fn returns_true_when_it_is_the_passed_players_turn() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            game.mark_cell(1, Player::P2);
            assert!(game.is_players_turn(Player::P1));
        }

        #[test]
        fn returns_false_when_it_is_not_the_passed_players_turn() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            assert!(game.is_players_turn(Player::P2));
        }
    }

    mod winning_player_tests {
        use crate::*;

        #[test]
        fn returns_player1_if_they_won() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            game.mark_cell(1, Player::P1);
            game.mark_cell(2, Player::P1);
            let winning_player = game.winning_player().unwrap();
            assert!(matches!(winning_player, Player::P1));
        }

        #[test]
        fn returns_player2_if_they_won() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P2);
            game.mark_cell(1, Player::P2);
            game.mark_cell(2, Player::P2);
            let winning_player = game.winning_player().unwrap();
            assert!(matches!(winning_player, Player::P2));
        }

        #[test]
        fn returns_none_if_no_one_won() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            game.mark_cell(1, Player::P2);
            let winning_player = game.winning_player();
            assert!(matches!(winning_player, None));
        }
    }
}
