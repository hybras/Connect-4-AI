use super::{ImplBoard, Piece};

#[derive(Clone)]
pub struct FlatBoard {
	height: usize,
	width: usize,
	board: Vec<Vec<Option<Piece>>>,
	num_moves: usize,
	is_blue_turn: bool,
	heights: Vec<usize>,
}

impl ImplBoard for FlatBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			height,
			width,
			board: vec![vec![None; height]; width],
			num_moves: 0,
			is_blue_turn: true,
			heights: vec![0; width],
		}
	}
	fn is_playable(&self, col: &usize) -> bool {
		self.board[*col]
			.iter()
			.filter(|cell| cell.is_some())
			.count() <= self.height
	}
	fn get_winner(&self) -> Option<Option<Piece>> {
		let grid = &self.board;
		for n in 0..self.width {
			for i in 0..self.height {
				let opt_cell = grid[n][i];
				return match opt_cell {
					None => continue,
					Some(_) => {
						if (i < 4
							&& n > 2 && grid[n][i] == opt_cell
							&& grid[n - 1][i + 1] == opt_cell
							&& grid[n - 2][i + 2] == opt_cell
							&& grid[n - 3][i + 3] == opt_cell)
							|| (i > 2
								&& n > 2 && grid[n][i] == opt_cell
								&& grid[n - 1][i - 1] == opt_cell
								&& grid[n - 2][i - 2] == opt_cell
								&& grid[n - 3][i - 3] == opt_cell)
							|| (n < 3
								&& grid[n][i] == opt_cell && grid[n + 1][i] == opt_cell
								&& grid[n + 2][i] == opt_cell && grid[n + 3][i] == opt_cell)
							|| (i < 4
								&& grid[n][i] == opt_cell && grid[n][i + 1] == opt_cell
								&& grid[n][i + 2] == opt_cell && grid[n][i + 3] == opt_cell)
						{
							Some(opt_cell.clone())
						} else if n == 0 && !grid[n].iter().any(|cell| cell.is_none()) {
							Some(None)
						} else {
							continue;
						}
					}
				};
			}
		}
		None
	}
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.height
	}
	fn num_moves(&self) -> usize {
		self.num_moves
	}
	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if *col < self.width {
			if self.num_moves() < self.height * self.width {
				if self.is_playable(&col) {
					self.board[*col][self.heights[*col]] = Some(if self.is_blue_turn {
						Piece::Blue
					} else {
						Piece::Red
					});
					self.heights[*col] += 1;
					self.is_blue_turn = !self.is_blue_turn;
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
}

use std::fmt::{Display, Formatter};

impl Display for FlatBoard {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for col in &self.board {
			for opt_cell in col {
				match opt_cell {
					Some(cell) => write!(out, "{} ", cell)?,
					None => write!(out, "⚪ ")?,
				}
			}
			writeln!(out)?;
		}
		Ok(())
	}
}

use crate::state::hist_board::HistBoard;
use std::convert::From;

impl From<HistBoard> for FlatBoard {
	fn from(hist_board: HistBoard) -> Self {
		let mut flat_board = Self::new(hist_board.width(), hist_board.height());
		for moveth in hist_board.moves {
			flat_board.make_move(&moveth);
		}
		flat_board
	}
}

#[cfg(test)]
mod test {
	use super::FlatBoard;
	use crate::state::{hist_board::HistBoard, ImplBoard};

	#[test]
	fn hist_to_flat_conversion() {
		let (height, width) = (6, 7);
		let mut hist_board = HistBoard::new(width, height);
		let moves = vec![0, 1, 2, 3, 4, 1, 3, 2];

		for col in moves {
			hist_board.make_move(&col);
		}

		let flat = FlatBoard::from(hist_board);
		println!("{}", flat);
	}
}
