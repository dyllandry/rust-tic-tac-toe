fn main() {
    let mut board = Board::new();
    board.marks[0].player = Some(Player::P1);
    board.marks[1].player = Some(Player::P1);
    board.marks[2].player = Some(Player::P1);
    println!("{}", board);
}

struct Board {
    marks: [Mark; 9],
}

impl Board {
    fn new() -> Self {
        let mut marks: [Mark; 9] = [Mark {
            player: None,
            index: 0,
        }; 9];
        for i in 0..9 {
            marks[i].index = i as i32;
        }
        return Board { marks };
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} | {} | {}",
            self.marks[0], self.marks[1], self.marks[2]
        )
        .unwrap();
        writeln!(f, "---------",).unwrap();
        writeln!(
            f,
            "{} | {} | {}",
            self.marks[3], self.marks[4], self.marks[5]
        )
        .unwrap();
        writeln!(f, "---------",).unwrap();
        writeln!(
            f,
            "{} | {} | {}",
            self.marks[6], self.marks[7], self.marks[8]
        )
    }
}

#[derive(Clone, Copy)]
struct Mark {
    player: Option<Player>,
    index: i32,
}

impl std::fmt::Display for Mark {
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
