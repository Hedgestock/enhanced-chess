macro_rules! impl_bit_op {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident) => {
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
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitBoard( 
    pub u64
);

impl_bit_op!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_bit_op!(BitOr,  bitor,  BitOrAssign,  bitor_assign);
impl_bit_op!(BitXor, bitxor, BitXorAssign, bitxor_assign);
impl_bit_op!(Add, add, AddAssign, add_assign);
impl_bit_op!(Sub, sub, SubAssign, sub_assign);
impl_bit_op!(Mul, mul, MulAssign, mul_assign);
impl_bit_op!(Div, div, DivAssign, div_assign);