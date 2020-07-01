use crate::state::{Board, Piece};
use std::io::stdin;
fn main() -> Result<(), std::io::Error> {
	let mut board = Board::default();
	let winner = main_loop(&mut board);
	game_end_message(winner);
	Ok(())
}

fn main_loop(board: &mut Board) -> Option<Piece> {
	let stdin = stdin();

	let mut is_blue_turn = true;

	let winner;
	loop {
		if is_blue_turn {
			println!("It's blue's turn!");
		} else {
			println!("It's red's turn!");
		}
		println!("{}", board);
		print!("Your move: ");
		let mut col = String::new();
		stdin.read_line(&mut col).unwrap();
		println!();
		let col = col.trim().parse::<usize>().unwrap();
		board.make_move(col).unwrap();
		if let Some(won) = board.get_winner() {
			winner = won;
			break;
		} else {
			is_blue_turn = !is_blue_turn;
		}
	}
	winner
}

fn game_end_message(winner: Option<Piece>) {
	match winner {
		Some(winner) => match winner {
			Piece::Red => println!("Red won"),
			Piece::Blue => println!("Blue won"),
		},
		None => println!("It was a tie"),
	}
}

mod state;
