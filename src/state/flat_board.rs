use super::{Board, Piece};

#[derive(Clone)]
pub struct FlatBoard {
	height: usize,
	width: usize,
	board: Vec<Vec<Option<Piece>>>,
	num_moves: usize,
	is_blue_turn: bool,
	heights: Vec<usize>,
}

impl Board for FlatBoard {
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

	fn find_height(&self, col: &usize) -> usize {
		self.heights[*col]
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
		if self.is_playable(col) {
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

impl Default for FlatBoard {
	fn default() -> Self {
		Self::new(7, 6)
	}
}
