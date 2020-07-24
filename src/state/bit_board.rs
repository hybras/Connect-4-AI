use super::{Board, Piece};
use bitvec::prelude as bv;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct BitBoard {
	blue_pieces: bv::BitBox,
	all_pieces: bv::BitBox,
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
		let blue_pieces = blue_pieces.into_boxed_bitslice();

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
		if self.num_moves() == self.height() * self.width() {
			return Some(None);
		}
		let shifts = [
			self.height(),     // horizontal
			self.height() - 1, // diagonal 1
			self.height() + 1, // diagonal 2
			1,                 // vertical
		];
		let mut red_pieces = self.blue_pieces.clone();
		red_pieces ^= self.all_pieces.clone();
		let player_to_pieces = [
			(Piece::Blue, self.blue_pieces.clone()),
			(Piece::Red, red_pieces),
		];

		for (player, pieces) in player_to_pieces.iter() {
			for &shift in shifts.iter() {
				let [mut pieces0, mut pieces1, mut pieces2] =
					[pieces.clone(), pieces.clone(), pieces.clone()];
				pieces1.rotate_right(shift);
				pieces2.rotate_right(2 * shift);
				pieces0 &= pieces1 & pieces2;
				if pieces0.any() {
					return Some(Some(*player));
				}
			}
		}

		None
	}
	fn make_move(&mut self, col: &usize) -> Result<(), String> {
		if self.is_playable(col) {
			let idx = col * self.height() + self.find_height(col);
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
	use crate::state::{Board, Piece};

	#[test]
	fn print_bitboard() {
		let mut bb = BitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		println!("{}", bb);
	}

	#[test]
	fn blue_wins() {
		let mut bb = BitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		bb.make_move(&2).unwrap();
		assert!(Piece::Blue == bb.get_winner().unwrap().unwrap());
	}

	#[test]
	fn red_wins() {
		let mut bb = BitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		bb.make_move(&1).unwrap();
		bb.make_move(&4).unwrap();
		assert!(Piece::Red == bb.get_winner().unwrap().unwrap());
	}
}
