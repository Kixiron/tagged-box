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
    ($([ $feature:literal, $discrim:ty, $max:expr, $ptr_width:expr, $free_bits:expr ]),*) => {
        $(
            #[cfg(feature = $feature)]
            mod variables {
                doc! {
                    "A discriminant stored in a [`TaggedPointer`], represented by a `", stringify!($discrim),
                    "` for the `", $feature, "` feature\n\n[`TaggedPointer`]: crate::TaggedPointer";
                    pub type Discriminant = $discrim;
                }

                doc! {
                    "The maximum allowed value of a discriminant, which for the `", $feature, "` feature is ", $max;
                    pub const MAX_DISCRIMINANT: Discriminant = $max;
                }

                doc! {
                    "The maximum allowed value of a pointer, which for the `", $feature, "` feature is `2 ^ ", $ptr_width, "`";
                    pub const MAX_POINTER_VALUE: usize = usize::max_value() >> $free_bits;
                }

                doc! {
                    "The reserved width of a pointer, which for the `", $feature, "` feature is ", $ptr_width, " bits";
                    pub const POINTER_WIDTH: usize = $ptr_width;
                }

                doc! {
                    "A mask to remove the upper free bits of a tagged pointer, which for the `", $feature,
                    "` feature is `usize::MAX >> ", $free_bits, "`";
                    pub const DISCRIMINANT_MASK: usize = usize::max_value() >> $free_bits;
                }
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
