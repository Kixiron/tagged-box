//! Stores variables that change upon different reserved pointer widths, as set by features

/// A discriminant stored in a [`TaggedPointer`]
///
/// [`TaggedPointer`]: crate::TaggedPointer
pub type Discriminant = variables::InnerDiscriminant;

/// The maximum allowed value of a discriminant, determined by the number of free bits
pub const MAX_DISCRIMINANT: Discriminant = variables::MAX_DISCRIMINANT_INNER;

/// The number of reserved bits of a pointer
pub const POINTER_WIDTH: usize = variables::POINTER_WIDTH_INNER;

/// A mask to remove the upper free bits of a tagged pointer
pub const DISCRIMINANT_MASK: usize = variables::DISCRIMINANT_MASK_INNER;

/// Macro to help generate the discriminant variables for every reserved pointer width
macro_rules! generate_discriminants {
    ($([$feature:literal, $discrim:ty, $max:expr, $ptr_width:expr, $free_bits:expr]),*) => {
        $(
            #[cfg(feature = $feature)]
            mod variables {
                pub(super) type InnerDiscriminant = $discrim;
                pub(super) const MAX_DISCRIMINANT_INNER: super::Discriminant = $max;
                pub(super) const POINTER_WIDTH_INNER: usize = $ptr_width;
                pub(super) const DISCRIMINANT_MASK_INNER: usize = usize::max_value() >> $free_bits;
            }
        )*
    };
}

generate_discriminants! {
    ["57bits", u8,  127,   57, 7 ],
    ["56bits", u8,  254,   56, 8 ],
    ["55bits", u16, 511,   55, 9 ],
    ["54bits", u16, 1023,  54, 10],
    ["53bits", u16, 2047,  53, 11],
    ["52bits", u16, 4068,  52, 12],
    ["51bits", u16, 8191,  51, 13],
    ["50bits", u16, 16383, 50, 14],
    ["49bits", u16, 32767, 49, 15],
    ["48bits", u16, 65535, 48, 16]
}
