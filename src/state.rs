/// A board piece. Used to represent the a Piece (and the absence of one) in a board grid, as well as the players.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
	Red,
	Blue,
}
use std::ops::RangeInclusive;

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
/// The board of connect 4 game. Contains the board's height and width, as well as move history, as a Vec of columns.
pub struct Board {
	moves: Vec<usize>,
	height: usize,
	width: usize,
	column_order: Vec<usize>,
}

/// Constructs the default 6x7 board.
impl Default for Board {
	fn default() -> Self {
		Self::new(7,6)
	}
}
use std::fmt::{Display, Formatter};

impl Display for Board {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let grid = self.as_2d();
		for col in grid {
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

impl Board {
	fn new(width: usize, height: usize) -> Self {
		let mut column_order = vec![0; width];
		for i in 0..width {
			column_order[i] = width / 2 + (1 - 2 * (i % 2)) * (i + 1) / 2;
		}
		Self {
			moves: vec![],
			width,
			height,
			column_order,
		}
	}

	/// The number of moves played so far
	fn num_moves(&self) -> usize {
		self.moves.len()
	}

	/// The number of pieces, or height, in a given column of the board.
	fn find_height(&self, col: usize) -> usize {
		self.moves.iter().filter(|&&c| col == c).count()
	}

	/// Whether the number of pieces in a column is below max height
	fn is_playable(&self, col: usize) -> bool {
		self.find_height(col) < self.height
	}

	/// Function checks if a column is playable (ie not full) and records the move.
	pub fn make_move(&mut self, col: usize) -> Result<(), String> {
		if col < self.width {
			if self.num_moves() < self.height * self.width {
				if self.is_playable(col) {
					self.moves.push(col);
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

	/// The board as a 2d grid, instead of as a list of moves. The innermost vec is a columns. Access cells as `as_2d()[col][row]`
	fn as_2d(&self) -> Vec<Vec<Option<Piece>>> {
		let mut grid = vec![vec![None; self.height]; self.width];
		let mut heights = vec![0; self.width];
		let mut is_blue = true;
		for &col_index in &self.moves {
			grid[col_index][heights[col_index]] =
				Some(if is_blue { Piece::Blue } else { Piece::Red });
			heights[col_index] += 1;
			is_blue = !is_blue;
		}
		grid
	}

	/// The option represents whether a winner exists. `Some(Piece::Empty)` indicates a tie.
	pub fn get_winner(&self) -> Option<Option<Piece>> {
		let grid = self.as_2d();
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

	fn is_winning_move(&mut self, col: usize) -> Result<bool, ()> {
		match self.make_move(col) {
			Ok(_) => {
				let winner = self.get_winner();
				let res = winner.is_some() && winner.unwrap().is_some();
				self.moves.pop();
				Ok(res)
			}
			Err(_) => Err(()),
		}
	}

	pub fn negamax_score(&mut self, mut range: RangeInclusive<i32>) -> i32 {
		use std::convert::TryInto;
		if self.num_moves() >= self.width * self.height {
			return 0;
		}
		for col_index in 0..self.width {
			if self.is_winning_move(col_index).is_ok() {
				return (self.width * self.height + 1 - self.num_moves() / 2)
					.try_into()
					.unwrap();
			}
		}
		let best = (self.width * self.height - 1 - self.num_moves() / 2) as i32;
		if *range.end() > best {
			if *range.start() >= best {
				return best;
			} else {
				range = *range.start()..=best;
			}
		}
		for col_index in 0..self.width {
			if self.is_playable(col_index) {
				match self.make_move(col_index) {
					Ok(_) => {
						let score = -self.negamax_score(-range.end()..=-range.start());
						if score >= *range.end() {
							return score;
						}
						if score > *range.start() {
							range = score..=*range.end();
						}
						self.moves.pop();
					}
					Err(_) => {
						//Should be impossible
					}
				}
			}
		}
		best
	}
}

#[cfg(test)]
mod test {
	use super::{Board, Piece};

	#[test]
	fn test_blue_wins() {
		let mut board: Board = Default::default();
		board.make_move(0).unwrap();

		for i in 0..3 {
			board.make_move(i + 1).unwrap();
			board.make_move(0).unwrap();
		}
		let winner = board.get_winner().unwrap().unwrap();
		assert!(winner == Piece::Blue, "Winner is {}", winner);
	}
}
