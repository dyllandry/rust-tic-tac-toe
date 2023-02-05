use crate::user_input::Input;

pub struct TicTacToe {
    board: Board,
    showed_title_screen: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
            showed_title_screen: false,
        }
    }

    pub fn input(&mut self, input: Input) {
        match input {
            Input::MarkCell(cell) => {
                self.mark_cell(cell, self.player_with_current_turn());
            }
        }
    }

    pub fn render(&mut self) {
        if !self.showed_title_screen {
            println!();
            render_title_screen();
            self.showed_title_screen = true;
        }

        println!();
        self.board.render();
        if let Some(winner) = self.winning_player() {
            println!();
            println!("{} won!", winner);
        } else {
            println!();
            print!("{} pick a cell 1-9: ", self.player_with_current_turn());
        }
        // Output to stdout is line-buffered. Flushing buffer ensures output is emitted
        // immediately.
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    pub fn game_over(&self) -> bool {
        self.winning_player().is_some()
    }

    /// Might be made private. Idea is that game receives input and decides what
    /// should happen.
    pub fn mark_cell(&mut self, cell: usize, player: Player) {
        if self.board.cells[cell].player.is_none() {
            self.board.cells[cell].player = Some(player);
        }
    }

    fn player_with_current_turn(&self) -> Player {
        let filled_cells = self
            .board
            .cells
            .iter()
            .map(|cell| cell.player)
            .filter_map(std::convert::identity)
            .count();
        if (filled_cells % 2) == 0 {
            Player::P1
        } else {
            Player::P2
        }
    }

    fn winning_player(&self) -> Option<Player> {
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

    pub fn render(&self) {
        println!("{} | {} | {}", self.cells[0], self.cells[1], self.cells[2]);
        println!("---------",);
        println!("{} | {} | {}", self.cells[3], self.cells[4], self.cells[5]);
        println!("---------",);
        println!("{} | {} | {}", self.cells[6], self.cells[7], self.cells[8]);
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
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

fn render_title_screen() {
    let lines = vec![
        r##"                    _..._                                           _..._                              .-'''-.                        "##,
        r##"                 .-'_..._''.                                     .-'_..._''.                          '   _    \                      "##,
        r##"         .--.  .' .'      '.\                                  .' .'      '.\                       /   /` '.   \      __.....__      "##,
        r##"         |__| / .'                                            / .'                                 .   |     \  '  .-''         '.    "##,
        r##"     .|  .--.. '                                .|           . '                                .| |   '      |  '/     .-''"'-.  `.  "##,
        r##"   .' |_ |  || |             ,.----------.    .' |_     __   | |             ,.----------.    .' |_\    \     / //     /________\   \ "##,
        r##" .'     ||  || |            //            \ .'     | .:--.'. | |            //            \ .'     |`.   ` ..' / |                  | "##,
        r##"'--.  .-'|  |. '            \\            /'--.  .-'/ |   \ |. '            \\            /'--.  .-'   '-...-'`  \    .-------------' "##,
        r##"   |  |  |  | \ '.          .`'----------'    |  |  `" __ | | \ '.          .`'----------'    |  |                \    '-.____...---. "##,
        r##"   |  |  |__|  '. `._____.-'/                 |  |   .'.''| |  '. `._____.-'/                 |  |                 `.             .'  "##,
        r##"   |  '.'        `-.______ /                  |  '.'/ /   | |_   `-.______ /                  |  '.'                 `''-...... -'    "##,
        r##"   |   /                  `                   |   / \ \._,\ '/            `                   |   /                                   "##,
        r##"   `'-'                                       `'-'   `--'  `"                                 `'-'                                    "##,
    ];
    for line in lines.iter() {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    mod player_with_current_turn_tests {
        use crate::game::*;

        #[test]
        fn returns_player1_when_it_is_player1s_turn() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            game.mark_cell(1, Player::P2);
            assert_eq!(game.player_with_current_turn(), Player::P1);
        }

        #[test]
        fn returns_player2_when_it_is_player2s_turn() {
            let mut game = TicTacToe::new();
            game.mark_cell(0, Player::P1);
            assert_eq!(game.player_with_current_turn(), Player::P2);
        }
    }

    mod winning_player_tests {
        use crate::game::*;

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
