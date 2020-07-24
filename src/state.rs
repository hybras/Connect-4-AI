use std::ops::RangeInclusive;
/// A board piece. Used to represent the a Piece (and the absence of one) in a board grid, as well as the players.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
	Red,
	Blue,
}

use std::fmt::{Display, Formatter};

impl Display for Piece {
	fn fmt(&self, out: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		Ok(write!(
			out,
			"{}",
			match self {
				Piece::Red => '🔴',
				Piece::Blue => '🔵',
			}
		)?)
	}
}

pub trait Board: Display + Clone + Default {
	fn new(width: usize, height: usize) -> Self;

	fn width(&self) -> usize;
	fn height(&self) -> usize;
	fn column_order(&self) -> Vec<usize> {
		let mut column_order = vec![0; self.width()];
		for i in 0..self.width() {
			column_order[i] = self.width() / 2 + (1 - 2 * (i % 2)) * (i + 1) / 2;
		}
		column_order
	}

	fn find_height(&self, col: &usize) -> usize;

	fn num_moves(&self) -> usize;
	fn is_playable(&self, col: &usize) -> bool {
		*col < self.width()
			&& self.num_moves() < self.height() * self.width()
			&& self.find_height(col) < self.height()
	}
	fn is_winning_move(&self, col: &usize) -> Result<bool, ()> {
		let mut copy = self.clone();
		match copy.make_move(col) {
			Ok(_) => {
				let winner = copy.get_winner();
				Ok(winner.is_some() && winner.unwrap().is_some())
			}
			Err(_) => Err(()),
		}
	}

	fn score(&self) -> i32 {
		let capacity = (self.width() * self.height()) as i32;
		self.clone().score_in_range(-capacity..=capacity)
	}

	fn score_in_range(&mut self, mut range: RangeInclusive<i32>) -> i32 {
		use std::convert::TryInto;
		if self.num_moves() >= self.width() * self.height() {
			return 0;
		}
		for col_index in 0..self.width() {
			if self.is_winning_move(&col_index).is_ok() {
				return (self.width() * self.height() + 1 - self.num_moves() / 2)
					.try_into()
					.unwrap();
			}
		}
		let best = (self.width() * self.height() - 1 - self.num_moves() / 2) as i32;
		if *range.end() > best {
			if *range.start() >= best {
				return best;
			} else {
				range = *range.start()..=best;
			}
		}
		let column_order = self.column_order();

		for col_index in 0..self.width() {
			if self.is_playable(&self.column_order()[col_index]) {
				match self.make_move(&column_order[col_index]) {
					Ok(_) => {
						let score = -self.score_in_range(-range.end()..=-range.start());
						if score >= *range.end() {
							return score;
						}
						if score > *range.start() {
							range = score..=*range.end();
						}
						// self.moves.pop();
					}
					Err(_) => {
						//Should be impossible
					}
				}
			}
		}
		best
	}

	/// The option represents whether a winner exists. `Some(Piece::Empty)` indicates a tie.
	fn get_winner(&self) -> Option<Option<Piece>>;

	/// Function checks if a column is playable (ie not full) and records the move.
	fn make_move(&mut self, col: &usize) -> Result<(), String>;

	// TODO fn current_player(&mut self) -> Option<Piece>
}

mod bit_board;
mod flat_board;
mod hist_board;

pub use self::flat_board::FlatBoard;
pub use self::hist_board::HistBoard;
pub use self::bit_board::BitBoard;
