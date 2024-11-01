#![cfg_attr(not(feature = "std"), no_std)]

use num_traits::{ConstOne, ConstZero, PrimInt};
mod error;
mod num_widths;
pub mod prelude;

/// A bitmap with an integer type backend.
///
/// Type `T` is the integer used to store the bits. **No arrays are used internally.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitMap<T: PrimInt + ConstZero + ConstOne + num_widths::ConstWidth>(T);

impl<T: PrimInt + ConstZero + ConstOne + num_widths::ConstWidth> BitMap<T> {
    /// Creates a new bitmap with all bits set to zero.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// // An eight-bit bitmap
    /// let bitmap_u8: BitMap<u8> = BitMap::new();
    ///
    /// // A sixteen-bit bitmap
    /// let bitmap_u16: BitMap<u16> = BitMap::new();
    ///
    /// // A thirty-two-bit bitmap
    /// let bitmap_u32: BitMap<u32> = BitMap::new();
    ///
    /// // A sixty-four-bit bitmap
    /// let bitmap_u64: BitMap<u64> = BitMap::new();
    /// ```
    pub const fn new() -> Self {
        Self(T::ZERO)
    }

    /// Returns the value of the bitmap. This is the integer that holds all the bits.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// assert_eq!(my_bitmap.value(), 0);
    ///
    /// my_bitmap.replace(4);
    /// assert_eq!(my_bitmap.value(), 4);
    ///
    /// my_bitmap.set_nth(0, true);
    /// assert_eq!(my_bitmap.value(), 5);
    /// ```
    pub const fn value(self) -> T {
        self.0
    }

    /// Replaces the value of the bitmap. This will change the integer that holds all the bits.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.replace(4);
    /// assert_eq!(my_bitmap.value(), 4);
    /// ```
    pub fn replace(&mut self, another: T) {
        self.0 = another;
    }

    /// Gets the nth bit of the bitmap.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.replace(4); // 00000100
    ///
    /// assert_eq!(my_bitmap.nth_bit(7), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(6), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(5), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(4), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(3), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(2), Some(true));
    /// assert_eq!(my_bitmap.nth_bit(1), Some(false));
    /// assert_eq!(my_bitmap.nth_bit(0), Some(false));
    /// ```
    pub fn nth_bit(self, index: usize) -> Option<bool> {
        if index >= T::WIDTH {
            None
        } else {
            Some(((self.0 >> index) & T::ONE) != T::ZERO)
        }
    }

    /// Sets the nth bit of the bitmap.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.set_nth(2, true);
    /// assert_eq!(my_bitmap.value(), 4);
    /// ```
    pub fn set_nth(&mut self, index: usize, value: bool) -> Result<(), error::IndexError> {
        if index >= T::WIDTH {
            return Err(error::IndexError);
        }

        if value {
            self.0 = self.0 | (T::ONE << index)
        } else {
            self.0 = self.0 & !(T::ONE << index)
        }

        Ok(())
    }

    /// Flips the nth bit of the bitmap.
    ///
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.flip_nth(2);
    /// assert_eq!(my_bitmap.value(), 4);
    /// ```
    pub fn flip_nth(&mut self, index: usize) -> Result<(), error::IndexError> {
        let Some(current) = self.nth_bit(index) else {
            return Err(error::IndexError);
        };

        if current {
            self.set_nth(index, false)?;
        } else {
            self.set_nth(index, true)?;
        }

        Ok(())
    }

    /// Counts all ones in the bitmap.
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.replace(4);
    /// assert_eq!(my_bitmap.count_ones(), 1);
    ///
    /// my_bitmap.replace(5);
    /// assert_eq!(my_bitmap.count_ones(), 2);
    /// ```
    pub fn count_ones(&self) -> usize {
        self.0.count_ones() as _
    }

    /// Counts all ones in the bitmap.
    /// ```rust
    /// # use easy_bitmap::BitMap;
    /// let mut my_bitmap: BitMap<u8> = BitMap::new();
    ///
    /// my_bitmap.replace(4);
    /// assert_eq!(my_bitmap.count_zeros(), 7);
    ///
    /// my_bitmap.replace(5);
    /// assert_eq!(my_bitmap.count_zeros(), 6);
    /// ```
    pub fn count_zeros(&self) -> usize {
        self.0.count_zeros() as _
    }
}
