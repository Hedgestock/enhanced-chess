use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};

use bevy::ecs::resource::Resource;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitBoard(pub u64);

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 & other.0);
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 & other.0);
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 & other.0);
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 & other.0);
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 | other.0);
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 | other.0);
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 | other.0);
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 | other.0);
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 ^ other.0);
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    fn bitxor(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 ^ other.0);
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0 ^ other.0);
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn bitxor(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0 ^ other.0);
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    fn mul(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0.wrapping_mul(other.0));
    }
}

impl Mul for &BitBoard {
    type Output = BitBoard;

    fn mul(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0.wrapping_mul(other.0));
    }
}

impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    fn mul(self, other: &BitBoard) -> BitBoard {
        return BitBoard(self.0.wrapping_mul(other.0));
    }
}

impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    fn mul(self, other: BitBoard) -> BitBoard {
        return BitBoard(self.0.wrapping_mul(other.0));
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

#[derive(Resource, Debug)]
pub struct GameState {
    pub white_pawn: BitBoard,
    pub black_pawn: BitBoard,
    pub white_knight: BitBoard,
    pub black_knight: BitBoard,
    pub white_bishop: BitBoard,
    pub black_bishop: BitBoard,
    pub white_rook: BitBoard,
    pub black_rook: BitBoard,
    pub white_queen: BitBoard,
    pub black_queen: BitBoard,
    pub white_king: BitBoard,
    pub black_king: BitBoard,
}

impl GameState {
    pub fn white_pieces(&self) -> BitBoard {
        return &self.white_pawn
            | &self.white_knight
            | &self.white_bishop
            | &self.white_rook
            | &self.white_queen
            | &self.white_king;
    }

    pub fn black_pieces(&self) -> BitBoard {
        return &self.black_pawn
            | &self.black_knight
            | &self.black_bishop
            | &self.black_rook
            | &self.black_queen
            | &self.black_king;
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            white_pawn: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            ),
            black_pawn: BitBoard(
                0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            white_knight: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            ),
            black_knight: BitBoard(
                0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            white_bishop: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            ),
            black_bishop: BitBoard(
                0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            white_rook: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            ),
            black_rook: BitBoard(
                0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            white_queen: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            ),
            black_queen: BitBoard(
                0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            white_king: BitBoard(
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            ),
            black_king: BitBoard(
                0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
        }
    }
}
