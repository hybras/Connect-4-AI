use super::{ImplBoard, Piece};

#[derive(Clone)]
pub struct FlatBoard {
	height: usize,
	width: usize,
	board: Vec<Vec<Option<Piece>>>,
}

impl ImplBoard for FlatBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			height,
			width,
			board: vec![vec![None; height]; width],
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
		let heights = vec![0; hist_board.width()];
		let mut is_blue = true;
		for moveth in hist_board.moves {
			flat_board.board[moveth][heights[moveth]] =
				Some(if is_blue { Piece::Blue } else { Piece::Red });
			is_blue = !is_blue;
			heights[moveth] += 1;
		}
		flat_board
	}
}

#[cfg(test)]
mod test {
	use super::FlatBoard;
	use crate::state::{ImplBoard, hist_board::HistBoard};

	#[test]
	fn hist_to_flat_conversion() {
		let (height, width) = (6, 7);
		let mut hist_board = HistBoard::new(width, height);
		let moves = vec![0, 1, 2, 3, 4, 1, 3, 2];

		for col in moves {
			hist_board.make_move(col);
		}

		let flat = FlatBoard::from(hist_board);
		println!("{}", flat);
	}
}
