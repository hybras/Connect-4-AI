use super::{Board, Piece};
use bitvec::prelude as bv;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct BitBoard {
	blue_pieces: bv::BitVec,
	all_pieces: bv::BitVec,
	height: usize,
	width: usize,
	moves: usize,
}

impl Board for BitBoard {
	fn new(width: usize, height: usize) -> Self {
		use std::iter::repeat;

		let bit_size = width * height;

		let mut blue_pieces = bv::BitVec::with_capacity(bit_size);
		blue_pieces.extend(repeat(false).take(bit_size));

		let all_pieces = blue_pieces.clone();

		Self {
			height,
			width,
			blue_pieces,
			all_pieces,
			moves: 0,
		}
	}
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.height
	}
	fn find_height(&self, col: &usize) -> usize {
		self.all_pieces[col * self.height()..(col + 1) * (self.height())].count_ones()
	}
	fn num_moves(&self) -> usize {
		self.moves
	}
	fn get_winner(&self) -> Option<Option<Piece>> {
		todo!()
	}
	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if self.is_playable(col) {
			let idx = col * self.height + self.find_height(col);
			self.all_pieces.set(idx, true);
			if self.moves % 2 == 0 {
				self.blue_pieces.set(idx, true);
			}
			self.moves += 1;
			Ok(())
		} else {
			Err("Not playable".into())
		}
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
			.chunks_exact(self.height())
			.zip(self.all_pieces.chunks_exact(self.height()))
		{
			for (blue, is_filled) in blue_col.iter().zip(all_col.iter()) {
				if *is_filled {
					write!(out, "{} ", if *blue { Piece::Blue } else { Piece::Red })?
				} else {
					write!(out, "⚪ ")?
				}
			}
			writeln!(out)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::BitBoard;
	use crate::state::Board;
	#[test]
	fn print_bitboard() {
		let mut bb = BitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		println!("{}", bb);
	}
}
