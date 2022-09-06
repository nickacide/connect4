#[allow(dead_code)]
use colored;
use colored::{Colorize, ColoredString};
#[derive(Clone, Copy)]
enum Turn {
    Player,
    Computer,
}
#[derive(Clone, Copy)]
enum Piece {
    Red,
    Yellow,
}
struct GameState {
    moves: Vec<usize>,
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
            moves: vec![35, 36, 37, 38, 39, 40, 41],
            board: [None; 42], //7x6
            red: *turn,
            yellow: Self::invert(*turn),
            current_turn: *turn,
        }
    }
    fn apply(&mut self) -> &Self {
        todo!();
    }
    fn print(&self) {
        let red = format!("O").bright_red().bold();
        let yellow = format!("O").bright_yellow().bold();
    let none = format!("").bold();
        let gui_board: Vec<&ColoredString> = self.board.iter().map(|f| match f {
            Some(Piece::Red) => &red,
            Some(Piece::Yellow) => &yellow,
            None => &none,
        }).collect();
        for row in 0..6 {
            println!("+---+---+---+---+---+---+---+");
            for col in 0..7 {
                print!("| {} ", gui_board[row * 6 + col]);
            }
            print!("|\n")
        }
        println!("+---+---+---+---+---+---+---+")
    }
}

fn main() {
    let state = GameState::new(&Turn::Computer);
    state.print();
}
