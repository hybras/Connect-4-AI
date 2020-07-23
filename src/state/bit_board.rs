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
		use std::iter::repeat;

		let bit_size = width * (height + 1);

		let blue_pieces = bv::BitVec::with_capacity(bit_size);
		blue_pieces.extend(repeat(false).take(bit_size));
		let blue_pieces = blue_pieces.into_boxed_bitslice();

		let all_pieces = bv::BitVec::with_capacity(bit_size);
		all_pieces.extend(repeat(false).take(bit_size));
		let all_pieces = all_pieces.into_boxed_bitslice();

		Self {
			height,
			width,
			blue_pieces,
			all_pieces,
		}
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
