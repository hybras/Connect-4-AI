trait Repr {
	fn as_ascii(&self) -> char;
	fn as_emoji(&self) -> char;
}
#[derive(Copy, Clone)]
pub enum Piece {
	Red,
	Blue,
	Empty,
}

impl Repr for Piece {
	fn as_ascii(&self) -> char {
		match self {
			Piece::Red => 'X',
			Piece::Blue => 'O',
			Piece::Empty => ' ',
		}
	}
	fn as_emoji(&self) -> char {
		match self {
			Piece::Red => '🔴',
			Piece::Blue => '🔵',
			Piece::Empty => '⚪',
		}
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
				write!(out, "{} ", cell.as_emoji())?;
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
}
