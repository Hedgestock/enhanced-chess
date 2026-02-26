use bevy::{ecs::resource::Resource, platform::collections::HashMap};

use crate::{bitboard::BitBoard, pieces::{PieceColor, PieceType}};



#[derive(Resource)]
pub struct GameState {
    pub pieces: HashMap<(PieceType, PieceColor), BitBoard>,
}

impl GameState {
    pub fn white_pieces(&self) -> BitBoard {
        return self
            .pieces
            .iter()
            .filter(|(k, _v)| k.1 == PieceColor::White)
            .fold(BitBoard(0), |acc, x| acc | x.1);
    }

    pub fn black_pieces(&self) -> BitBoard {
        return self
            .pieces
            .iter()
            .filter(|(k, _v)| k.1 == PieceColor::Black)
            .fold(BitBoard(0), |acc, x| acc | x.1);
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
        }
    }
}
