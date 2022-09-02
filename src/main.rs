#[allow(dead_code)]
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
            moves: vec![0, 1, 2, 3, 4, 5, 6],
            board: [None; 42],
            red: *turn,
            yellow: Self::invert(*turn),
            current_turn: *turn,
        }
    }
}

fn main() {
    todo!();
}
