use super::{Board, Piece};
use bitvec::prelude as bv;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct AltBitBoard {
	current_pieces: bv::BitBox,
	all_pieces: bv::BitBox,
	height: usize,
	width: usize,
	num_moves: usize,
}

impl AltBitBoard {
	pub(crate) fn reset(&mut self) {
		self.current_pieces.set_all(false);
		self.all_pieces.set_all(false);
		self.num_moves = 0;
	}
}

impl Board for AltBitBoard {
	fn new(width: usize, height: usize) -> Self {
		use std::iter::repeat;

		let bit_size = width * height;

		let mut current_pieces = bv::BitVec::with_capacity(bit_size);
		current_pieces.extend(repeat(false).take(bit_size));
		let current_pieces = current_pieces.into_boxed_bitslice();

		let all_pieces = current_pieces.clone();

		Self {
			height,
			width,
			current_pieces,
			all_pieces,
			num_moves: 0,
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
		self.num_moves
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
		let mut alt_pieces = self.current_pieces.clone();
		alt_pieces ^= self.all_pieces.clone();
		let alt_player;
		let current_player = if self.num_moves() % 2 == 0 {
			alt_player = Piece::Red;
			Piece::Blue
		} else {
			alt_player = Piece::Blue;
			Piece::Red
		};

		let player_to_pieces = [
			(current_player, self.current_pieces.clone()),
			(alt_player, alt_pieces),
		];

		for (player, pieces) in player_to_pieces.iter() {
			for &shift in shifts.iter() {
				let pieces_loop = vec![pieces.clone(); 4];
				if pieces_loop
					.into_iter()
					.enumerate()
					.fold_first(|(_, a), (idx_b, mut b)| {
						b.rotate_right(idx_b * shift);
						(0, a & b)
					})
					.unwrap()
					.1
					.any()
				{
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
			self.current_pieces.set(idx, true);
			self.current_pieces = self.all_pieces.clone() ^ self.current_pieces.clone();
			self.num_moves += 1;
			Ok(())
		} else {
			Err("Not playable".into())
		}
	}
}

impl Default for AltBitBoard {
	fn default() -> Self {
		Self::new(7, 6)
	}
}

impl Display for AltBitBoard {
	fn fmt(&self, out: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let alt_player;
		let current_player = if self.num_moves() % 2 == 0 {
			alt_player = Piece::Red;
			Piece::Blue
		} else {
			alt_player = Piece::Blue;
			Piece::Red
		};
		for (current_player_col, all_col) in self
			.current_pieces
			.chunks_exact(self.height())
			.zip(self.all_pieces.chunks_exact(self.height()))
		{
			for (current_players_piece, is_filled) in current_player_col.iter().zip(all_col.iter())
			{
				if *is_filled {
					write!(
						out,
						"{} ",
						if *current_players_piece {
							current_player
						} else {
							alt_player
						}
					)?
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
	use super::AltBitBoard;
	use crate::state::{Board, Piece};

	#[test]
	fn print_bitboard() {
		let mut bb = AltBitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		println!("{}", bb);
	}

	#[test]
	fn blue_wins() {
		let mut bb = AltBitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		bb.make_move(&2).unwrap();
		assert!(Piece::Blue == bb.get_winner().unwrap().unwrap());
	}

	#[test]
	fn red_wins() {
		let mut bb = AltBitBoard::default();
		(0..3).for_each(|_| {
			bb.make_move(&2).unwrap();
			bb.make_move(&4).unwrap();
		});
		bb.make_move(&1).unwrap();
		bb.make_move(&4).unwrap();
		assert!(Piece::Red == bb.get_winner().unwrap().unwrap());
	}
}
