use super::{Board, Piece};

use crate::state::flat_board::FlatBoard;

#[derive(Clone)]
pub struct HistBoard {
	pub moves: Vec<usize>,
	flat: FlatBoard,
}

impl Board for HistBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			moves: Vec::with_capacity(width * height),
			flat: FlatBoard::new(width, height),
		}
	}

	fn find_height(&self, col: &usize) -> usize {
		self.moves
			.iter()
			.filter(|&col_index| col_index == col)
			.count()
	}

	fn num_moves(&self) -> usize {
		self.moves.len()
	}

	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if self.is_playable(col) {
			self.moves.push(*col);
			self.flat.make_move(col)?;
			Ok(())
		} else {
			Err("Column is filled".to_string())
		}
	}
	fn width(&self) -> usize {
		self.flat.width()
	}
	fn height(&self) -> usize {
		self.flat.height()
	}
	fn get_winner(&self) -> Option<Option<Piece>> {
		self.flat.get_winner()
	}
	fn column_order(&self) -> Vec<usize> {
		let mut column_order = vec![0; self.width()];
		for i in 0..self.width() {
			column_order[i] = self.width() / 2 + (1 - 2 * (i % 2)) * (i + 1) / 2;
		}
		column_order
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
}
use std::fmt::{Display, Formatter};

impl Display for HistBoard {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Ok(writeln!(f, "{}", self.flat)?)
	}
}

impl Default for HistBoard {
	fn default() -> Self {
		Self::new(6, 7)
	}
}
