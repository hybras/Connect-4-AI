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

pub struct Board {
	moves: Vec<usize>,
	height: usize,
	width: usize,
}

impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}
use std::fmt::{Display, Formatter};

impl Display for Board {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for col in 0..self.width {
			for cell in 0..self.height {
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

	fn moves(&self) -> u8 {
		self.moves.len() as u8
	}

	pub fn make_move(&mut self, col: usize) -> Result<(), String> {
		use std::convert::TryInto;

		if col < self.width {
			if self.moves() < (self.height * self.width).try_into().unwrap() {
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

	fn find_height(&self, col: usize) -> usize {
		self.moves.iter().filter(|&&c| col == c as usize).count()
	}

	fn is_playable(&self, col: usize) -> bool {
		self.find_height(col) < self.height
	}

	fn as_2d(&self) -> Vec<Vec<Piece>> {
		let mut grid = vec![vec![Piece::Empty; self.height]; self.width];
		let mut heights = vec![0; self.width];
		for &col_index in &self.moves {
			grid[col_index][heights[col_index]] = Piece::Blue;
			heights[col_index] += 1;
		}
		grid
	}

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
