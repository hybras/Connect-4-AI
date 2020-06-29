trait Repr {
	fn as_ascii(&self) -> char;
	fn as_emoji(&self) -> char;
}
#[derive(Copy, Clone)]
enum Piece {
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

struct Board {
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

impl Board {
	fn new() -> Self {
		Default::default()
	}

	pub fn make_move(&mut self, player: Piece, col: usize) -> Result<(), String> {
		if col < 7 {
			match player {
				Piece::Blue => {
					if self.blue_moves.len() == self.red_moves.len() {
						self.grid[col][self.find_height(col)] = player;
						Ok(())
					} else {
						Err(String::from("Wrong Player Type"))
					}
				}
				Piece::Red => {
					if self.blue_moves.len() - 1 == self.red_moves.len() {
						self.grid[col][self.find_height(col)] = player;
						Ok(())
					} else {
						Err(String::from("Wrong Player Type"))
					}
				}
				Piece::Empty => Err(String::from("Wrong Player Type")),
			}
		} else {
			Err(String::from("Column too large"))
		}
	}

	fn find_height(&self, col: usize) -> usize {
		let mut height = 0;

		for cell_index in 0..7 {
			match &self.grid[col][cell_index] {
				Piece::Empty => height = cell_index,
				_ => continue,
			}
		}
		height
	}
}

fn main() {}
