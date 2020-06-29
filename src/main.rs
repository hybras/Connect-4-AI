use crate::state::{Board, Piece};

fn main() {
	let mut board: Board = Default::default();
	board.make_move(Piece::Blue, 0).unwrap();
	board.make_move(Piece::Red, 1).unwrap();
	println!("board:\n{}", board);
}

mod state;
