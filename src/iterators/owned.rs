use crate::{num_widths::ConstWidth, BitMap};
use num_traits::{ConstOne, ConstZero, PrimInt};

pub struct IntoBitMapIter<T: PrimInt + ConstZero + ConstOne + ConstWidth> {
    pub(crate) bitmap: BitMap<T>,
    pub(crate) pos: usize,
    pub(crate) pos_back: usize,
}

impl<T: PrimInt + ConstZero + ConstOne + ConstWidth> Iterator for IntoBitMapIter<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.pos_back {
            return None;
        }

        let current = self.bitmap.nth_bit(self.pos)?;

        self.pos += 1;
        Some(current)
    }
}

impl<T: PrimInt + ConstZero + ConstOne + ConstWidth> DoubleEndedIterator for IntoBitMapIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.pos_back < self.pos {
            return None;
        }

        let current = self.bitmap.nth_bit(self.pos_back)?;

        self.pos_back -= 1;
        Some(current)
    }
}
