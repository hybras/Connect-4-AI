/// A board piece. Used to represent the a Piece (and the absence of one) in a board grid, as well as the players. That last one was a bad design choice.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
	Red,
	Blue,
	Empty,
}

impl Display for Piece {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		Ok(write!(
			out,
			"{}",
			match self {
				Piece::Red => '🔴',
				Piece::Blue => '🔵',
				Piece::Empty => '⚪',
			}
		)?)
	}
}
/// The board of connect 4 game. Contains the board's height and width, as well as move history, as a Vec of columns.
pub struct Board {
	moves: Vec<usize>,
	height: usize,
	width: usize,
}

/// Constructs the default 6x7 board.
impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}
use std::fmt::{Display, Formatter};

impl Display for Board {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let grid = self.as_2d();
		for col in grid {
			for cell in col {
				write!(out, "{} ", cell)?;
			}
			writeln!(out)?;
		}
		Ok(())
	}
}

impl Board {
	fn new() -> Self {
		Self {
			moves: vec![],
			width: 7,
			height: 6,
		}
	}

	/// The number of moves played so far
	fn num_moves(&self) -> u8 {
		self.moves.len() as u8
	}

	/// Function checks if a column is playable (ie not full) and records the move.
	pub fn make_move(&mut self, col: usize) -> Result<(), String> {
		use std::convert::TryInto;

		if col < self.width {
			if self.num_moves() < (self.height * self.width).try_into().unwrap() {
				if self.is_playable(col) {
					self.moves.push(col);
					Ok(())
				} else {
					Err("Column is filled".to_string())
				}
			} else {
				Err("Column Filled".to_string())
			}
		} else {
			Err(String::from("Column too large"))
		}
	}

	/// The number of pieces, or height, in a given column of the board.
	fn find_height(&self, col: usize) -> usize {
		self.moves.iter().filter(|&&c| col == c as usize).count()
	}

	/// Whether the number of pieces in a column is below max height
	fn is_playable(&self, col: usize) -> bool {
		self.find_height(col) < self.height
	}

	/// The board as a 2d grid, instead of as a list of moves. The innermost vec is a columns. Access cells as `as_2d()[col][row]`
	fn as_2d(&self) -> Vec<Vec<Piece>> {
		let mut grid = vec![vec![Piece::Empty; self.height]; self.width];
		let mut heights = vec![0; self.width];
		let mut is_blue = true;
		for &col_index in &self.moves {
			grid[col_index][heights[col_index]] = if is_blue { Piece::Blue } else { Piece::Red };
			heights[col_index] += 1;
			is_blue = !is_blue;
		}
		grid
	}

	/// The option represents whether a winner exists. `Some(Piece::Empty)` indicates a tie.
	pub fn get_winner(&self) -> Option<Piece> {
		let grid = self.as_2d();
		for n in 0..self.width {
			for i in 0..self.height {
				let cell = grid[n][i];
				return match cell {
					Piece::Empty => continue,
					_ => {
						if (i < 4
							&& n > 2 && grid[n][i] == cell
							&& grid[n - 1][i + 1] == cell
							&& grid[n - 2][i + 2] == cell
							&& grid[n - 3][i + 3] == cell)
							|| (i > 2
								&& n > 2 && grid[n][i] == cell && grid[n - 1][i - 1] == cell
								&& grid[n - 2][i - 2] == cell && grid[n - 3][i - 3] == cell)
							|| (n < 3
								&& grid[n][i] == cell && grid[n + 1][i] == cell
								&& grid[n + 2][i] == cell && grid[n + 3][i] == cell)
							|| (i < 4
								&& grid[n][i] == cell && grid[n][i + 1] == cell
								&& grid[n][i + 2] == cell && grid[n][i + 3] == cell)
						{
							Some(cell.clone())
						} else if n == 0 && grid[n].iter().any(|cell| cell == &Piece::Empty) {
							Some(Piece::Empty)
						} else {
							continue;
						}
					}
				};
			}
		}
		Option::None
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
		let winner = board.get_winner().unwrap();
		assert!(winner == Piece::Blue, "Winner is {}", winner);
	}
}
