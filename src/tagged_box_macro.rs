/// Constructs a wrapper type and an associated enum to be stored as a [`TaggedBox`] that
/// can be used safely.
///
/// # Example Usage
///
/// The recommended way to use this crate is through the `tagged_box` macro, used like this:
///
/// ```rust
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::tagged_box;
///
/// tagged_box! {
///     #[derive(Debug, Clone, PartialEq)] // This will be applied to both the inner enum and outer container
///     struct Container, enum Item {
///         Integer(i32),
///         Boolean(bool),
///         String(String),
///     }
/// }
/// ```
///
/// Note: The number of variants must be <= 128 if using the `57bits` feature and <= 65536 if using the `48bits` feature,
/// see [`Discriminant`] for more info
///
/// This will create a struct `Container` and an enum `Item`. Expanded, they will look like this:
///
/// ```rust
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::{TaggableInner, TaggedBox};
/// # use core::mem::ManuallyDrop;
///
/// #[derive(Debug, Clone, PartialEq)]
/// #[repr(transparent)] // repr(transparent) is automatically added to the generated structs
/// struct Container {
///     value: TaggedBox<Item>,
/// }
///
/// #[derive(Debug, Clone, PartialEq)]
/// enum Item {
///     Integer(i32),
///     Boolean(bool),
///     String(String),
/// }
///
/// # // Have to implement this to avoid compile error
/// # impl TaggableInner for Item {
/// #     fn into_tagged_box(self) -> TaggedBox<Self> {
/// #         match self {
/// #             Self::Integer(int) => TaggedBox::new(int, 0),
/// #             Self::Boolean(boolean) => TaggedBox::new(boolean, 1),
/// #             Self::String(string) => TaggedBox::new(string, 2),
/// #         }
/// #     }
/// #
/// #     fn from_tagged_box(tagged: TaggedBox<Self>) -> Self {
/// #         unsafe {
/// #             match tagged.discriminant() {
/// #                 0 => Self::Integer(TaggedBox::into_inner(tagged)),
/// #                 1 => Self::Boolean(TaggedBox::into_inner(tagged)),
/// #                 2 => Self::String(TaggedBox::into_inner(tagged)),
/// #                 _ => unreachable!(),
/// #             }
/// #         }
/// #     }
/// #
/// #     unsafe fn ref_from_tagged_box<F>(tagged: &TaggedBox<Self>, callback: F)
/// #     where
/// #         F: FnOnce(&Self),
/// #     {
/// #         let value = ManuallyDrop::new(match tagged.discriminant() {
/// #             0 => Self::Integer(*tagged.as_ptr::<i32>()),
/// #             1 => Self::Boolean(*tagged.as_ptr::<bool>()),
/// #             2 => Self::String(tagged.as_ptr::<String>().read()),
/// #             _ => unreachable!(),
/// #         });
/// #
/// #         (callback)(&value);
/// #     }
/// # }
///
/// // Omitted some generated code
/// ```
///
/// The omitted code will contain `From` implementations that allow you to get a `Container` from any value that would
/// be allowed to be inside of the `Item` enum, e.g.
///
/// ```compile_fail
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::tagged_box;
/// # tagged_box! {
/// #     #[derive(Debug, Clone, PartialEq)]
/// #     struct Container, enum Item {
/// #         Integer(i32),
/// #         Boolean(bool),
/// #         String(String),
/// #     }
/// # }
///
/// Container::from(10i32);         // Works!
/// Container::from(String::new()); // Works!
/// Container::from(Vec::new());    // Doesn't work :(
/// ```
///
/// With your freshly created container, you can now store an enum on the stack with only `usize` bytes of memory and
/// safely retrieve it.
///
/// To get the value of a `Container` instance, simply use [`into_inner`] after importing the [`TaggableContainer`] trait
///
/// ```rust
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::tagged_box;
/// # tagged_box! {
/// #     #[derive(Debug, Clone, PartialEq)]
/// #     struct Container, enum Item {
/// #         Integer(i32),
/// #         Boolean(bool),
/// #         String(String),
/// #     }
/// # }
///
/// use tagged_box::TaggableContainer;
///
/// let container = Container::from(String::from("Hello from tagged-box!"));
/// assert_eq!(container.into_inner(), Item::String(String::from("Hello from tagged-box!")));
/// ```
///
/// [`TaggedBox`]: crate::TaggedBox
/// [`Discriminant`]: crate::Discriminant
/// [`into_inner`]: crate::TaggableContainer#into_inner
/// [`TaggableContainer`]: crate::TaggableContainer
#[macro_export]
macro_rules! tagged_box {
    (
        $( #[$meta:meta] )*
        $struct_vis:vis struct $struct:ident, $enum_vis:vis enum $enum:ident {
            $( $variant:ident($ty:ty), )+
        }
    ) => {
        // #[doc(hidden)]
        // mod __tagged_box_generated {
        //     #[doc(hidden)]
        //     #[allow(dead_code, non_camel_case_types)]
        //     pub(super) enum __tagged_box_generated_count_enum {
        //         $( $variant ),+
        //     }
        // }

        $( #[$meta] )*
        #[repr(transparent)]
        $struct_vis struct $struct {
            value: $crate::TaggedBox<$enum>,
        }

        impl $crate::TaggableContainer for $struct {
            type Inner = $enum;

            #[allow(cast_possible_truncation, cast_lossless, unnecessary_cast, unused_assignments)]
            fn into_inner(self) -> $enum {
                let mut discriminant = 0;

                // Safety: The generated discriminants and their associated variants should be valid, as
                // they are macro generated. As such, when calling `into_inner` the requested type should
                // be valid for the tagged pointer
                unsafe {
                    // TODO: Enable match statement
                    // match self.value.discriminant() {
                    //     $(
                    //         i if i == __tagged_box_generated::__tagged_box_generated_count_enum::$variant as u16 =>
                    //             $enum::$variant($crate::TaggedBox::into_inner::<$ty>(self.value)),
                    //     )+
                    //     _ => panic!("Attempted to create an enum variant from a discriminant that doesn't exist!"),
                    // }

                    $(
                        if self.value.discriminant() == discriminant {
                            return $enum::$variant($crate::TaggedBox::into_inner::<$ty>(self.value));
                        } else {
                            discriminant += 1;
                        }
                    )+
                }

                panic!("Attempted to create an enum variant from a discriminant that doesn't exist!");
            }
        }

        impl From<$enum> for $struct {
            #[inline]
            fn from(variant: $enum) -> Self {
                use $crate::TaggableInner;

                Self {
                    value: variant.into_tagged_box(),
                }
            }
        }

        $(
            impl From<$ty> for $struct {
                #[inline]
                fn from(value: $ty) -> Self {
                    use $crate::TaggableInner;

                    Self {
                        value: $enum::$variant(value).into_tagged_box(),
                    }
                }
            }
        )+

        $( #[$meta] )*
        $enum_vis enum $enum {
            $( $variant($ty) ),+
        }

        impl $crate::TaggableInner for $enum {
            #[allow(
                unused_assignments,
                irrefutable_let_patterns,
                unused_assignments,
                cast_possible_truncation,
                cast_lossless,
                unnecessary_cast
            )]
            fn into_tagged_box(self) -> $crate::TaggedBox<Self> {
                // TODO: Enable match statement
                // match self {
                //     $(
                //         Self::$variant(value) => $crate::TaggedBox::new(
                //             value,
                //             __tagged_box_generated::__tagged_box_generated_count_enum::$variant as u16 as $crate::Discriminant
                //         ),
                //     )+
                // }

                let mut discriminant = 0;

                $(
                    if let Self::$variant(value) = self {
                        return $crate::TaggedBox::new(value, discriminant);
                    } else {
                        discriminant += 1;
                    }
                )+

                unreachable!("All possible variants should have been destructed");
            }

            fn from_tagged_box(tagged: $crate::TaggedBox<$enum>) -> Self {
                let mut discriminant = 0;

                // Safety: The discriminants and the enum variants should be synced, as they are all
                // generated by a macro. Therefore, when `tagged`'s discriminant and the current discriminant
                // are the same, the variant should be valid for the data stored at `tagged`
                #[allow(unused_assignments, cast_possible_truncation, cast_lossless, unnecessary_cast)]
                unsafe {
                    // match tagged.discriminant() {
                    //     $(
                    //         i if i == __tagged_box_generated::__tagged_box_generated_count_enum::$variant as u16 =>
                    //             Self::$variant($crate::TaggedBox::into_inner::<$ty>(tagged)),
                    //     )+
                    //
                    //     invalid => {
                    //         // Create an [&'static str] from the number of variants, and then get the length of that
                    //         const TOTAL_VARIANTS: usize = [$( stringify!($variant) ),+].len();
                    //         panic!(
                    //             "The number of variants in `{}` is {}, but a variant by the discriminant of {} was attempted to be created",
                    //             stringify!($enum),
                    //             TOTAL_VARIANTS,
                    //             invalid
                    //         );
                    //     }
                    // }

                    $(
                        if tagged.discriminant() == discriminant {
                            return Self::$variant($crate::TaggedBox::into_inner::<$ty>(tagged));
                        } else {
                            discriminant += 1;
                        }
                    )+
                }

                const TOTAL_VARIANTS: usize = [$( stringify!($variant) ),+].len();
                panic!(
                    "The number of variants in `{}` is {}, but a variant by the discriminant of {} was attempted to be created",
                    stringify!($enum),
                    TOTAL_VARIANTS,
                    discriminant
                );
            }

            #[allow(unused_assignments)]
            unsafe fn ref_from_tagged_box<F>(tagged: &$crate::TaggedBox<$enum>, callback: F)
            where
                F: FnOnce(&$enum),
            {
                let mut discriminant = 0;

                $(
                    if tagged.discriminant() == discriminant {
                        let variant = core::mem::ManuallyDrop::new(Self::$variant(tagged.as_ptr::<$ty>().read()));
                        (callback)(&variant);

                        return;
                    } else {
                        discriminant += 1;
                    }
                )+

                // Create an [&'static str] from the number of variants, and then get the length of that
                const TOTAL_VARIANTS: usize = [$( stringify!($variant) ),+].len();
                panic!(
                    "The number of variants in `{}` is {}, but a variant by the discriminant of {} was attempted to be referenced",
                    stringify!($enum),
                    TOTAL_VARIANTS,
                    discriminant
                );
            }
        }
    };
}
