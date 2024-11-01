/// Generic trait for primitive integers which provides a single constant that stores the integers size.
///
/// This is used as an alternative to `T::zero().count_zeros()` provided by [`num_traits`]. This way, the size
/// can be retrieved in *O(1)* time, making it possible to use it within `const` contexts.
pub trait ConstWidth {
    /// The size *(amount of bits)* of the integer.
    const WIDTH: usize;
}

/// Auto-implements the `ConstWidth` trait.
macro_rules! impl_const_width {
    ($itype: ty, $width: literal) => {
        impl ConstWidth for $itype {
            const WIDTH: usize = $width;
        }
    };
}

impl_const_width!(u8, 8);
impl_const_width!(u16, 16);
impl_const_width!(u32, 32);
impl_const_width!(u64, 64);
impl_const_width!(u128, 128);

impl_const_width!(i8, 8);
impl_const_width!(i16, 16);
impl_const_width!(i32, 32);
impl_const_width!(i64, 64);
impl_const_width!(i128, 128);
