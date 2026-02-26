macro_rules! impl_bit_ops {
    ($(($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident)),*) => {
        $(

        // --- Value and Reference Permutations ---
        // BitBoard op BitBoard
        impl std::ops::$trait<BitBoard> for BitBoard {
            type Output = BitBoard;
            #[inline] fn $method(self, rhs: BitBoard) -> BitBoard { BitBoard(self.0.$method(rhs.0)) }
        }
        // &BitBoard op BitBoard
        impl std::ops::$trait<BitBoard> for &BitBoard {
            type Output = BitBoard;
            #[inline] fn $method(self, rhs: BitBoard) -> BitBoard { BitBoard(self.0.$method(rhs.0)) }
        }
        // BitBoard op &BitBoard
        impl std::ops::$trait<&BitBoard> for BitBoard {
            type Output = BitBoard;
            #[inline] fn $method(self, rhs: &BitBoard) -> BitBoard { BitBoard(self.0.$method(rhs.0)) }
        }
        // &BitBoard op &BitBoard
        impl std::ops::$trait<&BitBoard> for &BitBoard {
            type Output = BitBoard;
            #[inline] fn $method(self, rhs: &BitBoard) -> BitBoard { BitBoard(self.0.$method(rhs.0)) }
        }

        // --- Assignment Permutations ---
        // BitBoard op= BitBoard
        impl std::ops::$assign_trait<BitBoard> for BitBoard {
            #[inline] fn $assign_method(&mut self, rhs: BitBoard) { self.0.$assign_method(rhs.0); }
        }
        // BitBoard op= &BitBoard
        impl std::ops::$assign_trait<&BitBoard> for BitBoard {
            #[inline] fn $assign_method(&mut self, rhs: &BitBoard) { self.0.$assign_method(rhs.0); }
        }
        )*
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl_bit_ops!(
    (BitAnd, bitand, BitAndAssign, bitand_assign),
    (BitOr, bitor, BitOrAssign, bitor_assign),
    (BitXor, bitxor, BitXorAssign, bitxor_assign),
    (Add, add, AddAssign, add_assign),
    (Sub, sub, SubAssign, sub_assign),
    (Mul, mul, MulAssign, mul_assign),
    (Div, div, DivAssign, div_assign)
);

impl BitBoard {
    pub fn get_piece_positions(&self) -> Vec<u8> {
        let mut positions = Vec::new();
        let mut board = self.clone();
        while board != BitBoard(0) {
            // Get index of the lowest set bit (0-63)
            let sq = board.0.trailing_zeros() as u8;
            positions.push(sq);
            // Clear the lowest set bit
            board &= board - BitBoard(1);
        }
        positions
    }
}
