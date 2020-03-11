//! Stores variables that change upon different reserved pointer widths, as set by features

pub use variables::*;

/// Macro to help generate documentation  
/// Note: Any actual expressions that cannot be coerced into an `ident` should be wrapped
/// in a combination of `stringify!` and `concat!`
macro_rules! doc {
    ($($expr:expr),*; $($tt:tt)*) => {
        doc!(@add_doc
            concat!($( doc!(@match $expr) ),*);
            $( $tt )*
        );
    };

    (@match $lit:literal) => {
        $lit
    };
    (@match $expr:expr) => {
        concat!($expr)
    };

    (@add_doc $s:expr; $($tt:tt)*) => {
        #[doc = $s]
        $( $tt )*
    };
}

/// Macro to help generate the discriminant variables for every reserved pointer width
macro_rules! generate_discriminants {
    ($([ $feature:ident, $discrim:ty, $max:expr, $ptr_width:expr, $free_bits:expr ]),*) => {
        $(
            #[cfg($feature)]
            mod variables {
                doc! {
                    "A discriminant stored in a [`TaggedPointer`], represented by a `", stringify!($discrim),
                    "` for the `", stringify!($feature), "` feature\n\n[`TaggedPointer`]: crate::TaggedPointer";
                    pub type Discriminant = $discrim;
                }

                doc! {
                    "The maximum allowed value of a discriminant, which for the `", stringify!($feature), "` feature is ", $max;
                    pub const MAX_DISCRIMINANT: Discriminant = $max;
                }

                doc! {
                    "The maximum allowed value of a pointer, which for the `", stringify!($feature), "` feature is `2 ^ ", $ptr_width, "`";
                    pub const MAX_POINTER_VALUE: usize = usize::max_value() >> $free_bits;
                }

                doc! {
                    "The reserved width of a pointer, which for the `", stringify!($feature), "` feature is ", $ptr_width, " bits";
                    pub const POINTER_WIDTH: usize = $ptr_width;
                }

                doc! {
                    "A mask to remove the upper free bits of a tagged pointer, which for the `", stringify!($feature),
                    "` feature is `usize::MAX >> ", $free_bits, "`";
                    pub const DISCRIMINANT_MASK: usize = usize::max_value() >> $free_bits;
                }

                doc! {
                    "The total number of bits reserved for a discriminant, which for the `", stringify!($feature), "`feature is `", $free_bits, "`";
                    pub const DISCRIMINANT_BITS: usize = $free_bits;
                }
            }
        )*
    };
}

generate_discriminants! {
    [tagged_box_reserve_63bits, u8,  1,     63, 1 ],
    [tagged_box_reserve_62bits, u8,  3,     62, 2 ],
    [tagged_box_reserve_61bits, u8,  7,     61, 3 ],
    [tagged_box_reserve_60bits, u8,  15,    60, 4 ],
    [tagged_box_reserve_59bits, u8,  31,    59, 5 ],
    [tagged_box_reserve_58bits, u8,  63,    58, 6 ],
    [tagged_box_reserve_57bits, u8,  127,   57, 7 ],
    [tagged_box_reserve_56bits, u8,  255,   56, 8 ],
    [tagged_box_reserve_55bits, u16, 511,   55, 9 ],
    [tagged_box_reserve_54bits, u16, 1023,  54, 10],
    [tagged_box_reserve_53bits, u16, 2047,  53, 11],
    [tagged_box_reserve_52bits, u16, 4068,  52, 12],
    [tagged_box_reserve_51bits, u16, 8191,  51, 13],
    [tagged_box_reserve_50bits, u16, 16383, 50, 14],
    [tagged_box_reserve_49bits, u16, 32767, 49, 15],
    [tagged_box_reserve_48bits, u16, 65535, 48, 16]
}
