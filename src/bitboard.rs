use std::fmt;

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

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, "{}", (self.0 >> (7 - i) * 8 + j) & 1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bb_add_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(8);
        assert_eq!(bblhs + bbrhs, bbres);
    }

    #[test]
    fn bb_sub_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(4);
        assert_eq!(bblhs - bbrhs, bbres);
    }

    #[test]
    fn bb_mul_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(12);
        assert_eq!(bblhs * bbrhs, bbres);
    }

    #[test]
    fn bb_div_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(3);
        assert_eq!(bblhs / bbrhs, bbres);
    }

    #[test]
    fn rbb_sub_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(4);
        assert_eq!(&bblhs - bbrhs, bbres);
    }

    #[test]
    fn rbb_mul_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(12);
        assert_eq!(&bblhs * bbrhs, bbres);
    }

    #[test]
    fn rbb_div_bb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(3);
        assert_eq!(&bblhs / bbrhs, bbres);
    }

        #[test]
    fn bb_add_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(8);
        assert_eq!(bblhs + &bbrhs, bbres);
    }

    #[test]
    fn bb_sub_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(4);
        assert_eq!(bblhs - &bbrhs, bbres);
    }

    #[test]
    fn bb_mul_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(12);
        assert_eq!(bblhs * &bbrhs, bbres);
    }

    #[test]
    fn bb_div_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(3);
        assert_eq!(bblhs / &bbrhs, bbres);
    }

        #[test]
    fn rbb_add_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(8);
        assert_eq!(&bblhs + &bbrhs, bbres);
    }

    #[test]
    fn rbb_sub_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(4);
        assert_eq!(&bblhs - &bbrhs, bbres);
    }

    #[test]
    fn rbb_mul_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(12);
        assert_eq!(&bblhs * &bbrhs, bbres);
    }

    #[test]
    fn rbb_div_rbb() {
        let bblhs = BitBoard(6);
        let bbrhs = BitBoard(2);
        let bbres = BitBoard(3);
        assert_eq!(&bblhs / &bbrhs, bbres);
    }
}
