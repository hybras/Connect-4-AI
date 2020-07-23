use super::{Board, Piece};
use bitvec::prelude as bv;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct BitBoard {
	blue_pieces: bv::BitVec,
	all_pieces: bv::BitVec,
	height: usize,
	width: usize,
	is_blue_turn: bool,
}

impl Board for BitBoard {
	fn new(width: usize, height: usize) -> Self {
		use std::iter::repeat;

		let bit_size = width * (height + 1);

		let mut blue_pieces = bv::BitVec::with_capacity(bit_size);
		blue_pieces.extend(repeat(false).take(bit_size));

		let mut all_pieces = blue_pieces.clone();

		Self {
			height,
			width,
			blue_pieces,
			all_pieces,
			is_blue_turn: false,
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
	fn is_playable(&self, col: &usize) -> bool {
		*col < self.width()
			&& self.num_moves() < self.height() * self.width()
			&& self.all_pieces[(col + 1) * (self.height() + 1)]
	}
}

impl Default for BitBoard {
	fn default() -> Self {
		Self::new(7, 6)
	}
}

impl Display for BitBoard {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		for (blue_col, all_col) in self
			.blue_pieces
			.chunks_exact(self.height() + 1)
			.zip(self.all_pieces.chunks_exact(self.height() + 1))
		{
			for (blue, all) in blue_col.iter().take(self.height()).zip(all_col.iter()) {
				if !all {
					write!(out, "⚪ ")?
				} else {
					write!(out, "{} ", if *blue { Piece::Blue } else { Piece::Red })?
				}
			}
			writeln!(out)?;
		}
		Ok(())
	}
}
