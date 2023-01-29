fn main() {
    let mut board = Board::new();
    board.cells[0].player = Some(Player::P1);
    board.cells[1].player = Some(Player::P1);
    board.cells[2].player = Some(Player::P1);
    println!("{}", board);
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
            cells[i].index = i as i32;
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
    index: i32,
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

#[derive(Clone, Copy)]
enum Player {
    P1,
    P2,
}
