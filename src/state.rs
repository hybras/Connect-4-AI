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
	grid: [[Piece; 6]; 7],
	red_moves: Vec<u8>,
	blue_moves: Vec<u8>,
}

impl Default for Board {
	fn default() -> Self {
		Board {
			grid: [[Piece::Empty; 6]; 7],
			red_moves: vec![],
			blue_moves: vec![],
		}
	}
}
use std::fmt::{Display, Formatter};

impl Display for Board {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for col in 0..self.grid.len() {
			for cell in self.grid[col].iter() {
				write!(out, "{} ", cell)?;
			}
			writeln!(out)?;
		}
		Ok(())
	}
}

impl Board {
	fn new() -> Self {
		Default::default()
	}

	fn moves(&self) -> u8 {
		(self.red_moves.len() + self.blue_moves.len()) as u8
	}

	pub fn make_move(&mut self, player: Piece, col: usize) -> Result<(), String> {
		if col < self.grid.len() {
			match player {
				Piece::Blue => {
					if self.blue_moves.len() == self.red_moves.len() {
						self.grid[col][self.find_height(col)] = player;
						self.blue_moves.push(col as u8);
						Ok(())
					} else {
						Err(String::from("Its Red's turn"))
					}
				}
				Piece::Red => {
					if self.blue_moves.len() == self.red_moves.len() + 1 {
						self.grid[col][self.find_height(col)] = player;
						self.red_moves.push(col as u8);
						Ok(())
					} else {
						Err(String::from(format!(
							"Its Blue's turn, blue:{}, red:{}",
							self.blue_moves.len(),
							self.red_moves.len()
						)))
					}
				}
				Piece::Empty => Err(String::from("Can not make an Empty turn")),
			}
		} else {
			Err(String::from("Column too large"))
		}
	}

	fn find_height(&self, col: usize) -> usize {
		for cell_index in 0..self.grid[col].len() {
			match &self.grid[col][cell_index] {
				Piece::Empty => return cell_index,
				_ => {}
			}
		}
		0
	}

	pub fn get_winner(&self) -> Option<Piece> {
		for n in 0..self.grid.len() {
			for i in 0..self.grid[n].len() {
				let cell = self.grid[n][i];
				return match cell {
					Piece::Empty => continue,
					_ => {
						if (i < 4
							&& n > 2 && self.grid[n][i] == cell
							&& self.grid[n - 1][i + 1] == cell
							&& self.grid[n - 2][i + 2] == cell
							&& self.grid[n - 3][i + 3] == cell)
							|| (i > 2
								&& n > 2 && self.grid[n][i] == cell
								&& self.grid[n - 1][i - 1] == cell
								&& self.grid[n - 2][i - 2] == cell
								&& self.grid[n - 3][i - 3] == cell)
							|| (n < 3
								&& self.grid[n][i] == cell && self.grid[n + 1][i] == cell
								&& self.grid[n + 2][i] == cell && self.grid[n + 3][i] == cell)
							|| (i < 4
								&& self.grid[n][i] == cell && self.grid[n][i + 1] == cell
								&& self.grid[n][i + 2] == cell && self.grid[n][i + 3] == cell)
						{
							Some(cell.clone())
						} else if n == 0 && self.grid[n].iter().any(|cell| cell == &Piece::Empty) {
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
		board.make_move(Piece::Blue, 0).unwrap();

		for i in 0..3 {
			board.make_move(Piece::Red, i + 1).unwrap();

			board.make_move(Piece::Blue, 0).unwrap();
		}
		let winner = board.get_winner().unwrap();
		assert!(winner == Piece::Blue, "Winner is {}", winner);
	}
}
