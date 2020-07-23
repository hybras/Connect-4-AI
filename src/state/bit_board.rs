use super::{Board, Piece};
use bitvec::prelude as bv;

struct BitBoard {
	blue_pieces: bv::BitBox,
	all_pieces: bv::BitBox,
	height: usize,
	width: usize,
}
