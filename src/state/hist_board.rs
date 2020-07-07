use super::{ImplBoard, Piece};

struct HistBoard {
	height: usize,
	width: usize,
	moves: Vec<usize>,
}

impl HistBoard {
	fn find_height(&self, col: &usize) -> usize {
		self.moves
			.iter()
			.filter(|&col_index| col_index == col)
			.count()
	}
}

impl ImplBoard for HistBoard {
	fn new(width: usize, height: usize) -> Self {
		Self {
			height,
			width,
			moves: Vec::with_capacity(width * height),
		}
	}

	fn is_playable(&self, col: &usize) -> bool {
		self.find_height(col) <= self.height
	}

	fn num_moves(&self) -> usize {
		self.moves.len()
	}

	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if col < self.width {
			if self.num_moves() < self.height * self.width {
				if self.is_playable(&col) {
					self.moves.push(col);
					self.grid[col][self.heights[col]] = Some(if self.moves.len() % 2 == 0 {
						Piece::Blue
					} else {
						Piece::Red
					});
					self.heights[col] += 1;
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
