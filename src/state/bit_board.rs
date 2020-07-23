use super::{Board, Piece};
use bitvec::prelude as bv;

struct BitBoard {
	blue_pieces: bv::BitBox,
	all_pieces: bv::BitBox,
	height: usize,
	width: usize,
}

impl Board for BitBoard {
	fn new(width: usize, height: usize) -> Self {
		todo!()
	}
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.height
	}
	fn find_height(&self, col: &usize) -> usize {
		todo!()
	}
	fn num_moves(&self) -> usize {
		todo!()
	}
	fn get_winner(&self) -> Option<Option<Piece>> {
		todo!()
	}
	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		todo!()
	}
}

impl Default for BitBoard {
	fn default() -> Self {
		Self::new(7, 6)
	}
}
