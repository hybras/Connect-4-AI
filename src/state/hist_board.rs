use super::{Board, Piece};

#[derive(Clone)]
pub struct HistBoard {
	height: usize,
	width: usize,
	pub moves: Vec<usize>,
}

impl HistBoard {
	fn find_height(&self, col: &usize) -> usize {
		self.moves
			.iter()
			.filter(|&col_index| col_index == col)
			.count()
	}
}

use std::fmt::{Display, Formatter};

impl Display for HistBoard {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use super::flat_board::FlatBoard;
		let flat: FlatBoard = self.clone().into();

		writeln!(f, "{}", flat)?;
		Ok(())
	}
}

impl Board for HistBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			height,
			width,
			moves: Vec::with_capacity(width * height),
		}
	}

	fn is_playable(&self, col: &usize) -> bool {
		self.find_height(col) <= self.height
	}

	fn num_moves(&self) -> usize {
		self.moves.len()
	}

	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if *col < self.width {
			if self.num_moves() < self.height * self.width {
				if self.is_playable(col) {
					self.moves.push(*col);
					Ok(())
				} else {
					Err("Column is filled".to_string())
				}
			} else {
				Err("Board Filled".to_string())
			}
		} else {
			Err("Column out of bound".to_string())
		}
	}
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.height
	}
	fn get_winner(&self) -> Option<Option<Piece>> {
		use crate::state::flat_board::FlatBoard;
		FlatBoard::from(self.clone()).get_winner()
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
