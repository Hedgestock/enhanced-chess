use std::sync::LazyLock;

use bevy::platform::collections::HashMap;

use crate::{bitboard::BitBoard, rendering::PieceColor};

// I don't fully understand that code, but I've read about Hyperbola Quintessence
// and the code works, so I'll study it further when I'll feel like switching to magic numbers

const A_FILE: u64 = 0x0101010101010101;
const H_FILE: u64 = 0x8080808080808080;
const RANK: u64 = 0xFF;
const KNIGHT_MOVES: [(i32,i32);8] = [
    (2,1),(2,-1),(-2,1),(-2,-1),
    (1,2),(1,-2),(-1,2),(-1,-2),
];
const KING_MOVES: [(i32,i32);8] = [
    (1,0),(-1,0),(0,1),(0,-1),
    (1,1),(1,-1),(-1,1),(-1,-1),
];

#[inline]
fn file_mask(square: BitBoard) -> BitBoard {
    let file = square.0.trailing_zeros() & 7;
    BitBoard(A_FILE << file)
}

#[inline]
fn rank_mask(square: BitBoard) -> BitBoard {
    let sq = square.0.trailing_zeros();
    let rank = sq >> 3;
    BitBoard(RANK << (rank * 8))
}

#[inline]
fn diag_mask(square: BitBoard) -> BitBoard {
    let sq = square.0.trailing_zeros() as i32;
    let rank = sq >> 3;
    let file = sq & 7;

    let mut mask = 0u64;

    for r in 0..8 {
        let f = r + file - rank;
        if f >= 0 && f < 8 {
            mask |= 1u64 << (r * 8 + f);
        }
    }

    BitBoard(mask)
}

// look into not computing that but using masks like for the rooks
#[inline]
fn anti_diag_mask(square: BitBoard) -> BitBoard {
    let sq = square.0.trailing_zeros() as i32;
    let rank = sq >> 3;
    let file = sq & 7;

    let mut mask = 0u64;

    for r in 0..8 {
        let f = rank + file - r;
        if f >= 0 && f < 8 {
            mask |= 1u64 << (r * 8 + f);
        }
    }

    BitBoard(mask)
}

#[inline]
fn sliding_attacks(occ: BitBoard, sq: BitBoard, mask: BitBoard) -> BitBoard {
    let forward = occ & mask;
    let reverse = forward.reverse_bits();

    let sq_rev = sq.reverse_bits();

    let left = forward.wrapping_sub(sq << 1);
    let right = reverse.wrapping_sub(sq_rev << 1);

    (left ^ right.reverse_bits()) & mask
}

static KNIGHT_ATTACKS: LazyLock<[BitBoard; 64]> = LazyLock::new(|| {
    generate_table(&KNIGHT_MOVES)
});

static KING_ATTACKS: LazyLock<[BitBoard; 64]> = LazyLock::new(|| {
    generate_table(&KING_MOVES)
});

fn generate_table(moves: &[(i32, i32)]) -> [BitBoard; 64] {
    let mut table = [BitBoard(0); 64];

    for sq in 0..64 {
        let rank = (sq / 8) as i32;
        let file = (sq % 8) as i32;

        let mut attacks = 0u64;

        for (dr, df) in moves {
            let r = rank + dr;
            let f = file + df;

            if r >= 0 && r < 8 && f >= 0 && f < 8 {
                let target = (r * 8 + f) as u64;
                attacks |= 1u64 << target;
            }
        }

        table[sq] = BitBoard(attacks);
    }
    table
}

static PAWN_ATTACKS: LazyLock<[[BitBoard; 64]; 2]> = LazyLock::new(|| {
    let mut table = [[BitBoard(0); 64]; 2];
    for sq in 0..64 {
        let bit = 1u64 << sq;

        table[PieceColor::White as usize][sq] =
            BitBoard(((bit << 7) & !A_FILE) | ((bit << 9) & !H_FILE));

        table[PieceColor::Black as usize][sq] =
            BitBoard(((bit >> 7) & !H_FILE) | ((bit >> 9) & !A_FILE));
    }
    table
});

#[inline]
pub fn rook_attacks(square: BitBoard, occ: BitBoard) -> BitBoard {
    sliding_attacks(occ, square, rank_mask(square))
        | sliding_attacks(occ, square, file_mask(square))
}

#[inline]
pub fn bishop_attacks(square: BitBoard, occ: BitBoard) -> BitBoard {
    sliding_attacks(occ, square, diag_mask(square))
        | sliding_attacks(occ, square, anti_diag_mask(square))
}

#[inline]
pub fn queen_attacks(square: BitBoard, occ: BitBoard) -> BitBoard {
    rook_attacks(square, occ) | bishop_attacks(square, occ)
}

#[inline]
pub fn knight_attacks(square: BitBoard) -> BitBoard {
    KNIGHT_ATTACKS[square.0.trailing_zeros() as usize]
}

#[inline]
pub fn king_attacks(square: BitBoard) -> BitBoard {
    KING_ATTACKS[square.0.trailing_zeros() as usize]
}

#[inline]
pub fn pawn_attacks(square: BitBoard, color: PieceColor) -> BitBoard {
    PAWN_ATTACKS[color as usize][square.0.trailing_zeros() as usize]
}