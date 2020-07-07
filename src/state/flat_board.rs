use super::{ImplBoard, Piece};
use std::fmt::{Display, Formatter};

struct FlatBoard {
	height: usize,
	width: usize,
	board: Vec<Vec<Option<Piece>>>,
}

impl ImplBoard for FlatBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			height,
			width,
			board: vec![vec![None; height];width],
		}
	}
	fn is_playable(&self, col: &usize) -> bool {
		self.board[*col].iter().filter(|cell| cell.is_some()).count()<=self.height
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
