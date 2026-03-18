use std::fmt;

use bevy::{ecs::resource::Resource, platform::collections::HashMap};

use crate::{
    bitboard::BitBoard,
    rendering::{PieceColor, PieceType},
};

// --- Move types ---

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flag: MoveFlag,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveFlag {
    Quiet,
    DoublePawnPush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    Promotion(PieceType),
    PromotionCapture(PieceType),
}

// --- Castling rights ---

#[derive(Clone, Copy, Debug, Default)]
pub struct CastlingSides {
    pub kingside: bool,
    pub queenside: bool,
}

/// Castling availability per color. Adding a new color (e.g. for 4-player chess)
/// only requires inserting an entry — no struct fields to change.
#[derive(Clone, Debug)]
pub struct CastlingRights(pub HashMap<PieceColor, CastlingSides>);

impl CastlingRights {
    pub fn kingside(&self, color: PieceColor) -> bool {
        self.0.get(&color).map_or(false, |s| s.kingside)
    }

    pub fn queenside(&self, color: PieceColor) -> bool {
        self.0.get(&color).map_or(false, |s| s.queenside)
    }

    pub fn revoke_kingside(&mut self, color: PieceColor) {
        if let Some(s) = self.0.get_mut(&color) {
            s.kingside = false;
        }
    }

    pub fn revoke_queenside(&mut self, color: PieceColor) {
        if let Some(s) = self.0.get_mut(&color) {
            s.queenside = false;
        }
    }

    pub fn revoke_all(&mut self, color: PieceColor) {
        if let Some(s) = self.0.get_mut(&color) {
            s.kingside = false;
            s.queenside = false;
        }
    }
}

// --- GameState ---

#[derive(Resource, Clone)]
pub struct GameState {
    pub pieces: HashMap<(PieceType, PieceColor), BitBoard>,
    pub side_to_move: PieceColor,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl GameState {
    pub fn pieces(&self, color: PieceColor) -> BitBoard {
        self.pieces
            .iter()
            .filter(|(k, _v)| k.1 == color)
            .fold(BitBoard(0), |acc, x| acc | x.1)
    }

    pub fn occupancy(&self) -> BitBoard {
        self.pieces.iter().fold(BitBoard(0), |acc, x| acc | x.1)
    }

    pub fn piece_at(&self, sq: u8) -> Option<(PieceType, PieceColor)> {
        let bit = BitBoard::from_index(sq);
        for ((pt, pc), bb) in &self.pieces {
            if (*bb & bit) != BitBoard(0) {
                return Some((*pt, *pc));
            }
        }
        None
    }

    pub fn apply_move(&self, mv: Move) -> GameState {
        let mut state = self.clone();
        let from_bit = BitBoard::from_index(mv.from);
        let to_bit = BitBoard::from_index(mv.to);
        let (moving_pt, moving_pc) = self
            .piece_at(mv.from)
            .expect("apply_move: no piece at from square");

        // Remove moving piece from source
        *state.pieces.get_mut(&(moving_pt, moving_pc)).unwrap() &= !from_bit;

        // Handle captures
        match mv.flag {
            MoveFlag::Capture | MoveFlag::PromotionCapture(_) => {
                if let Some((cap_pt, cap_pc)) = self.piece_at(mv.to) {
                    *state.pieces.get_mut(&(cap_pt, cap_pc)).unwrap() &= !to_bit;
                }
            }
            MoveFlag::EnPassant => {
                let cap_sq = if moving_pc == PieceColor::White {
                    mv.to - 8
                } else {
                    mv.to + 8
                };
                let enemy = moving_pc.opponent();
                *state.pieces.get_mut(&(PieceType::Pawn, enemy)).unwrap() &=
                    !BitBoard::from_index(cap_sq);
            }
            _ => {}
        }

        // Place piece at destination (swapped for promotions)
        let placed_pt = match mv.flag {
            MoveFlag::Promotion(pt) | MoveFlag::PromotionCapture(pt) => pt,
            _ => moving_pt,
        };
        *state.pieces.get_mut(&(placed_pt, moving_pc)).unwrap() |= to_bit;

        // Castling: also move the rook
        match mv.flag {
            MoveFlag::KingsideCastle => {
                let (rf, rt) = if moving_pc == PieceColor::White {
                    (7u8, 5u8)
                } else {
                    (63u8, 61u8)
                };
                let bb = state.pieces.get_mut(&(PieceType::Rook, moving_pc)).unwrap();
                *bb = (*bb & !BitBoard::from_index(rf)) | BitBoard::from_index(rt);
            }
            MoveFlag::QueensideCastle => {
                let (rf, rt) = if moving_pc == PieceColor::White {
                    (0u8, 3u8)
                } else {
                    (56u8, 59u8)
                };
                let bb = state.pieces.get_mut(&(PieceType::Rook, moving_pc)).unwrap();
                *bb = (*bb & !BitBoard::from_index(rf)) | BitBoard::from_index(rt);
            }
            _ => {}
        }

        // En passant target square (the square the pawn skipped over)
        state.en_passant = if mv.flag == MoveFlag::DoublePawnPush {
            Some((mv.from + mv.to) / 2)
        } else {
            None
        };

        // Castling rights: king moves forfeit both sides
        if moving_pt == PieceType::King {
            state.castling_rights.revoke_all(moving_pc);
        }
        // Castling rights: rook leaving its corner forfeits that side
        for sq in [mv.from, mv.to] {
            match sq {
                0 => state.castling_rights.revoke_queenside(PieceColor::White),
                7 => state.castling_rights.revoke_kingside(PieceColor::White),
                56 => state.castling_rights.revoke_queenside(PieceColor::Black),
                63 => state.castling_rights.revoke_kingside(PieceColor::Black),
                _ => {}
            }
        }

        // Side to move
        state.side_to_move = moving_pc.opponent();

        // Halfmove clock
        if moving_pt == PieceType::Pawn
            || matches!(
                mv.flag,
                MoveFlag::Capture | MoveFlag::EnPassant | MoveFlag::PromotionCapture(_)
            )
        {
            state.halfmove_clock = 0;
        } else {
            state.halfmove_clock += 1;
        }

        // Fullmove number increments after Black's move
        if moving_pc == PieceColor::Black {
            state.fullmove_number += 1;
        }

        state
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut array = ['o'; 64];
        for ((piece, color), bb) in self.pieces.iter() {
            for i in bb.get_piece_positions() {
                array[i as usize] = match piece {
                    PieceType::Pawn => {
                        if *color == PieceColor::White {
                            'P'
                        } else {
                            'p'
                        }
                    }
                    PieceType::Knight => {
                        if *color == PieceColor::White {
                            'N'
                        } else {
                            'n'
                        }
                    }
                    PieceType::Bishop => {
                        if *color == PieceColor::White {
                            'B'
                        } else {
                            'b'
                        }
                    }
                    PieceType::Rook => {
                        if *color == PieceColor::White {
                            'R'
                        } else {
                            'r'
                        }
                    }
                    PieceType::Queen => {
                        if *color == PieceColor::White {
                            'Q'
                        } else {
                            'q'
                        }
                    }
                    PieceType::King => {
                        if *color == PieceColor::White {
                            'K'
                        } else {
                            'k'
                        }
                    }
                };
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{}", array[i * 8 + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            pieces: HashMap::from([
                (
                    (PieceType::Pawn, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                    ),
                ),
                (
                    (PieceType::Pawn, PieceColor::Black),
                    BitBoard(
                        0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
                (
                    (PieceType::Knight, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                    ),
                ),
                (
                    (PieceType::Knight, PieceColor::Black),
                    BitBoard(
                        0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
                (
                    (PieceType::Bishop, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                    ),
                ),
                (
                    (PieceType::Bishop, PieceColor::Black),
                    BitBoard(
                        0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
                (
                    (PieceType::Rook, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                    ),
                ),
                (
                    (PieceType::Rook, PieceColor::Black),
                    BitBoard(
                        0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
                (
                    (PieceType::Queen, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                    ),
                ),
                (
                    (PieceType::Queen, PieceColor::Black),
                    BitBoard(
                        0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
                (
                    (PieceType::King, PieceColor::White),
                    BitBoard(
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
                    ),
                ),
                (
                    (PieceType::King, PieceColor::Black),
                    BitBoard(
                        0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                    ),
                ),
            ]),
            side_to_move: PieceColor::White,
            castling_rights: CastlingRights(HashMap::from([
                (PieceColor::White, CastlingSides { kingside: true, queenside: true }),
                (PieceColor::Black, CastlingSides { kingside: true, queenside: true }),
            ])),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}
