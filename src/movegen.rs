use std::sync::LazyLock;

use crate::{
    bitboard::BitBoard,
    game::{GameState, Move, MoveFlag},
    rendering::{PieceColor, PieceType},
};

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

        // <<7 goes NW (file-1, rank+1): A-file pawns wrap to H-file, filter !H_FILE
        // <<9 goes NE (file+1, rank+1): H-file pawns wrap to A-file, filter !A_FILE
        table[PieceColor::White as usize][sq] =
            BitBoard(((bit << 7) & !H_FILE) | ((bit << 9) & !A_FILE));

        // >>7 goes SE (file+1, rank-1): H-file pawns wrap to A-file, filter !A_FILE
        // >>9 goes SW (file-1, rank-1): A-file pawns wrap to H-file, filter !H_FILE
        table[PieceColor::Black as usize][sq] =
            BitBoard(((bit >> 7) & !A_FILE) | ((bit >> 9) & !H_FILE));
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

// Rank masks (0-indexed: rank 0 = white's back rank)
const RANK_1: u64 = 0x00000000000000FF; // bits  0-7  (black promotes here)
const RANK_2: u64 = 0x000000000000FF00; // bits  8-15 (white pawn starting rank)
const RANK_7: u64 = 0x00FF000000000000; // bits 48-55 (black pawn starting rank)
const RANK_8: u64 = 0xFF00000000000000; // bits 56-63 (white promotes here)

const PROMO_PIECES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

/// Returns true if `sq` is attacked by any piece of `by_color`.
pub fn is_attacked(sq: u8, by_color: PieceColor, state: &GameState) -> bool {
    let sq_bb = BitBoard::from_index(sq);
    let occ = state.occupancy();
    let opp = by_color.opponent();

    // Pawn: a pawn of `by_color` attacks `sq` iff a pawn of `opp` on `sq` would attack a `by_color` pawn
    let enemy_pawns = *state.pieces.get(&(PieceType::Pawn, by_color)).unwrap_or(&BitBoard(0));
    if pawn_attacks(sq_bb, opp) & enemy_pawns != BitBoard(0) {
        return true;
    }

    let enemy_knights = *state.pieces.get(&(PieceType::Knight, by_color)).unwrap_or(&BitBoard(0));
    if knight_attacks(sq_bb) & enemy_knights != BitBoard(0) {
        return true;
    }

    let enemy_king = *state.pieces.get(&(PieceType::King, by_color)).unwrap_or(&BitBoard(0));
    if king_attacks(sq_bb) & enemy_king != BitBoard(0) {
        return true;
    }

    let enemy_bishops = *state.pieces.get(&(PieceType::Bishop, by_color)).unwrap_or(&BitBoard(0));
    let enemy_queens = *state.pieces.get(&(PieceType::Queen, by_color)).unwrap_or(&BitBoard(0));
    if bishop_attacks(sq_bb, occ) & (enemy_bishops | enemy_queens) != BitBoard(0) {
        return true;
    }

    let enemy_rooks = *state.pieces.get(&(PieceType::Rook, by_color)).unwrap_or(&BitBoard(0));
    if rook_attacks(sq_bb, occ) & (enemy_rooks | enemy_queens) != BitBoard(0) {
        return true;
    }

    false
}

/// Returns true if the king of `color` is currently in check.
pub fn is_in_check(color: PieceColor, state: &GameState) -> bool {
    let king_bb = *state.pieces.get(&(PieceType::King, color)).unwrap_or(&BitBoard(0));
    if king_bb == BitBoard(0) {
        return false;
    }
    is_attacked(king_bb.0.trailing_zeros() as u8, color.opponent(), state)
}

/// Generates all pseudo-legal moves for the side to move.
pub fn generate_pseudo_legal_moves(state: &GameState) -> Vec<Move> {
    let color = state.side_to_move;
    let occ = state.occupancy();
    let own = state.pieces(color);
    let enemy = state.pieces(color.opponent());

    // Collect first to avoid holding a borrow on `state` while passing it to subfunctions
    let piece_bbs: Vec<(PieceType, BitBoard)> = state
        .pieces
        .iter()
        .filter(|((_, pc), _)| *pc == color)
        .map(|((pt, _), bb)| (*pt, *bb))
        .collect();

    let mut moves = Vec::new();

    for (pt, bb) in piece_bbs {
        for from in bb.get_piece_positions() {
            let from_bb = BitBoard::from_index(from);
            match pt {
                PieceType::Pawn => {
                    gen_pawn_moves(from, from_bb, color, occ, enemy, state.en_passant, &mut moves);
                }
                PieceType::Knight => {
                    gen_leaper_moves(from, knight_attacks(from_bb) & !own, enemy, &mut moves);
                }
                PieceType::Bishop => {
                    gen_leaper_moves(from, bishop_attacks(from_bb, occ) & !own, enemy, &mut moves);
                }
                PieceType::Rook => {
                    gen_leaper_moves(from, rook_attacks(from_bb, occ) & !own, enemy, &mut moves);
                }
                PieceType::Queen => {
                    gen_leaper_moves(from, queen_attacks(from_bb, occ) & !own, enemy, &mut moves);
                }
                PieceType::King => {
                    gen_leaper_moves(from, king_attacks(from_bb) & !own, enemy, &mut moves);
                    gen_castling_moves(from, color, occ, state, &mut moves);
                }
            }
        }
    }

    moves
}

/// Filters pseudo-legal moves to only those that don't leave the moving side's king in check.
pub fn generate_legal_moves(state: &GameState) -> Vec<Move> {
    let color = state.side_to_move;
    generate_pseudo_legal_moves(state)
        .into_iter()
        .filter(|&mv| !is_in_check(color, &state.apply_move(mv)))
        .collect()
}

fn gen_leaper_moves(from: u8, targets: BitBoard, enemy: BitBoard, moves: &mut Vec<Move>) {
    for to in targets.get_piece_positions() {
        let flag = if BitBoard::from_index(to) & enemy != BitBoard(0) {
            MoveFlag::Capture
        } else {
            MoveFlag::Quiet
        };
        moves.push(Move { from, to, flag });
    }
}

fn gen_pawn_moves(
    from: u8,
    from_bb: BitBoard,
    color: PieceColor,
    occ: BitBoard,
    enemy: BitBoard,
    en_passant: Option<u8>,
    moves: &mut Vec<Move>,
) {
    let (push_shift, promo_rank, start_rank, nw_mask, ne_mask): (
        fn(BitBoard) -> BitBoard,
        u64,
        u64,
        BitBoard,
        BitBoard,
    ) = match color {
        PieceColor::White => (
            |b: BitBoard| b << 8,
            RANK_8,
            RANK_2,
            !BitBoard(H_FILE), // NW captures: filter H-file wrapping
            !BitBoard(A_FILE), // NE captures: filter A-file wrapping
        ),
        PieceColor::Black => (
            |b: BitBoard| b >> 8,
            RANK_1,
            RANK_7,
            !BitBoard(A_FILE), // SE captures: filter A-file wrapping
            !BitBoard(H_FILE), // SW captures: filter H-file wrapping
        ),
    };

    let promo = BitBoard(promo_rank);
    let start = BitBoard(start_rank);

    // Single push
    let push1 = push_shift(from_bb) & !occ;
    for to in push1.get_piece_positions() {
        if BitBoard::from_index(to) & promo != BitBoard(0) {
            for pt in PROMO_PIECES {
                moves.push(Move { from, to, flag: MoveFlag::Promotion(pt) });
            }
        } else {
            moves.push(Move { from, to, flag: MoveFlag::Quiet });
        }
    }

    // Double push from starting rank
    if from_bb & start != BitBoard(0) {
        for to in (push_shift(push1) & !occ).get_piece_positions() {
            moves.push(Move { from, to, flag: MoveFlag::DoublePawnPush });
        }
    }

    // Diagonal captures
    // For white: <<7 is NW, <<9 is NE (using nw_mask/ne_mask set above)
    // For black: >>7 is SE, >>9 is SW
    let (left_cap, right_cap) = match color {
        PieceColor::White => (
            (from_bb << 7) & nw_mask & enemy,
            (from_bb << 9) & ne_mask & enemy,
        ),
        PieceColor::Black => (
            (from_bb >> 7) & nw_mask & enemy,
            (from_bb >> 9) & ne_mask & enemy,
        ),
    };
    for to in (left_cap | right_cap).get_piece_positions() {
        if BitBoard::from_index(to) & promo != BitBoard(0) {
            for pt in PROMO_PIECES {
                moves.push(Move { from, to, flag: MoveFlag::PromotionCapture(pt) });
            }
        } else {
            moves.push(Move { from, to, flag: MoveFlag::Capture });
        }
    }

    // En passant
    if let Some(ep_sq) = en_passant {
        let ep_bb = BitBoard::from_index(ep_sq);
        let ep_attacks = match color {
            PieceColor::White => ((from_bb << 7) & nw_mask) | ((from_bb << 9) & ne_mask),
            PieceColor::Black => ((from_bb >> 7) & nw_mask) | ((from_bb >> 9) & ne_mask),
        };
        if ep_attacks & ep_bb != BitBoard(0) {
            moves.push(Move { from, to: ep_sq, flag: MoveFlag::EnPassant });
        }
    }
}

fn gen_castling_moves(
    king_sq: u8,
    color: PieceColor,
    occ: BitBoard,
    state: &GameState,
    moves: &mut Vec<Move>,
) {
    let enemy = color.opponent();
    match color {
        PieceColor::White => {
            // Kingside: f1 (sq 5) and g1 (sq 6) must be empty; e1, f1, g1 not attacked
            if state.castling_rights.kingside(PieceColor::White)
                && occ & BitBoard(0x0000000000000060) == BitBoard(0)
                && !is_attacked(4, enemy, state)
                && !is_attacked(5, enemy, state)
                && !is_attacked(6, enemy, state)
            {
                moves.push(Move { from: king_sq, to: 6, flag: MoveFlag::KingsideCastle });
            }
            // Queenside: b1 (sq 1), c1 (sq 2), d1 (sq 3) empty; e1, d1, c1 not attacked
            if state.castling_rights.queenside(PieceColor::White)
                && occ & BitBoard(0x000000000000000E) == BitBoard(0)
                && !is_attacked(4, enemy, state)
                && !is_attacked(3, enemy, state)
                && !is_attacked(2, enemy, state)
            {
                moves.push(Move { from: king_sq, to: 2, flag: MoveFlag::QueensideCastle });
            }
        }
        PieceColor::Black => {
            // Kingside: f8 (sq 61) and g8 (sq 62) empty; e8, f8, g8 not attacked
            if state.castling_rights.kingside(PieceColor::Black)
                && occ & BitBoard(0x6000000000000000) == BitBoard(0)
                && !is_attacked(60, enemy, state)
                && !is_attacked(61, enemy, state)
                && !is_attacked(62, enemy, state)
            {
                moves.push(Move { from: king_sq, to: 62, flag: MoveFlag::KingsideCastle });
            }
            // Queenside: b8 (sq 57), c8 (sq 58), d8 (sq 59) empty; e8, d8, c8 not attacked
            if state.castling_rights.queenside(PieceColor::Black)
                && occ & BitBoard(0x0E00000000000000) == BitBoard(0)
                && !is_attacked(60, enemy, state)
                && !is_attacked(59, enemy, state)
                && !is_attacked(58, enemy, state)
            {
                moves.push(Move { from: king_sq, to: 58, flag: MoveFlag::QueensideCastle });
            }
        }
    }
}