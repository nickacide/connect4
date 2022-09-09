#[allow(dead_code)]
use colored;
use colored::{ColoredString, Colorize};
#[derive(Clone, Copy)]
enum Turn {
    Player,
    Computer,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Piece {
    Red,
    Yellow,
}
struct  Best {
    best_score: isize,
    best_move: usize
}
fn vec_winner(vec: Vec<Option<Piece>>) -> Option<Piece> {
    for start in 0..=vec.len() - 4 {
        let s0 = vec[start + 0];
        let s1 = vec[start + 1];
        let s2 = vec[start + 2];
        let s3 = vec[start + 3];
        if s0 == s1 && s1 == s2 && s2 == s3 {
            return s0;
        }
    }
    None
}
struct GameState {
    board: [Option<Piece>; 42],
    red: Turn,
    yellow: Turn,
    current_turn: Turn,
}
impl GameState {
    fn invert(turn: Turn) -> Turn {
        match &turn {
            Turn::Player => Turn::Computer,
            Turn::Computer => Turn::Player,
        }
    }
    fn new(turn: &Turn) -> GameState {
        GameState {
            board: [None; 42], //7x6
            red: *turn,
            yellow: Self::invert(*turn),
            current_turn: *turn,
        }
    }
    fn apply(&mut self, position: usize, piece: Option<Piece>) -> &Self {
        self.board[position] = piece;
        self
    }
    fn winner(&self) -> Option<Piece> {
        for row in 0..6 {
            let mut current_row: Vec<Option<Piece>> = vec![];
            for col in 0..7 {
                current_row.push(self.board[row * 7 + col]);
            }
            let winner = vec_winner(current_row);
            if winner != None {
                return winner;
            }
        }
        for col in 0..7 {
            let mut current_col: Vec<Option<Piece>> = vec![];
            for row in 0..6 {
                current_col.push(self.board[row * 7 + col]);
            }
            let winner = vec_winner(current_col);
            if winner != None {
                return winner;
            }
        }

        None
    }
    fn print(&self) {
        let red = format!("O").bright_red().bold();
        let yellow = format!("O").bright_yellow().bold();
        let none = format!(" ").bold();
        let gui_board: Vec<&ColoredString> = self
            .board
            .iter()
            .map(|f| match f {
                Some(Piece::Red) => &red,
                Some(Piece::Yellow) => &yellow,
                None => &none,
            })
            .collect();
        for row in 0..6 {
            println!("+---+---+---+---+---+---+---+");
            for col in 0..7 {
                print!("| {} ", gui_board[row * 6 + col]);
            }
            println!("|")
        }
        println!("+---+---+---+---+---+---+---+")
    }
    fn moves(&self) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];
        for col in 0..7 {
            for row in 0..6 {
                let current_row = 5 - row;
                if self.board[row * 7 + col] != None {
                    moves.push(row * 7 + col);
                    continue;
                }
            }
        }
        moves
    }
    // fn full(&self) -> bool {
    //     self.moves().len() == 0
    // }
    fn minimax(&self, is_max: bool) -> Best {
        let winner = self.winner();
        if winner != None {
            return Best {
                best_score: match winner {
                    Some(Piece::Red) => 1,
                    Some(Piece::Yellow) => -1,
                    None => 0
                },
                best_move: 42
            }
        }
        if is_max {
            let mut best_move: usize = 0;
            let mut best_score: isize = -2;
            for current_move in &self.moves() {
                // let mut new_state = self.clone();
                // new_state.apply(*current_move, Some(Piece::Red));
            }
        } else {

        }
        Best {
            best_score : 0,
            best_move : 0
        }
    }
}

fn main() {
    let state = GameState::new(&Turn::Computer);
    state.print();
    let winner = vec_winner(vec![Some(Piece::Yellow),Some(Piece::Yellow),Some(Piece::Yellow),Some(Piece::Red),Some(Piece::Red),Some(Piece::Red),Some(Piece::Red)]);
    println!("{:?}", winner);
    state.winner();
}
