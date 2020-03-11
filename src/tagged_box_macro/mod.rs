mod meta_macros;

#[doc(hidden)]
#[macro_export]
macro_rules! __make_enum {
    ($vis:vis, $enum:ident [$($meta:meta)*] $($rest:tt)*) => {
        $crate::__make_enum! { @inner $vis, $enum, $( $meta ),* [] $( $rest )* }
    };

    (@inner $vis:vis, $enum:ident, $($meta:meta),* [$($finished:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__make_enum! { @inner $vis, $enum, $( $meta ),* [$( $finished )* $variant($( $ty ),*),] $( $rest )* }
    };

    (@inner $vis:vis, $enum:ident, $($meta:meta),* [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__make_enum! { @inner $vis, $enum, $( $meta ),* [$( $finished )* $variant { $( $member: $ty ),* },] $( $rest )* }
    };
    (@inner $vis:vis, $enum:ident, $($meta:meta),* [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__make_enum! { @inner $vis, $enum, $( $meta ),* [$( $finished )* $variant { $( $member: $ty ),* },] $( $rest )* }
    };

    (@inner $vis:vis, $enum:ident, $($meta:meta),* [$($finished:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__make_enum! { @inner $vis, $enum, $( $meta ),* [$( $finished )* $variant,] $( $rest )* }
    };

    (@inner $vis:vis, $enum:ident, $($meta:meta),* [$($finished:tt)*]) => {
        $( #[$meta] )*
        $vis enum $enum {
            $( $finished )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __taggable_into_box {
    ($tagged:expr, $enum:ident, $counter:ident, $($rest:tt)*) => {
        $crate::__taggable_into_box!(@inner $tagged, $enum, $counter [] $( $rest )*)
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident [$($finished:tt)*] $variant:ident($ty:ty), $($rest:tt)* ) => {
        $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $finished )*
                $enum::$variant(var) => {
                    $crate::TaggedBox::new(var, $counter::$variant as _)
                },
            ]
            $( $rest )*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident [$($finished:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)* ) => {
        $crate::__expand_tuple_arm!($tagged, $enum, $counter, $variant, $( $ty ),* [$( $finished )*] [$( $rest )*] $( $ty ),*)
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident [$($tt:tt)*] $variant:ident { $($member:ident: $ty:ty),* }, $($rest:tt)* ) => {
        $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $tt )*
                $enum::$variant { $( $member ),* } => $crate::TaggedBox::new({
                    #[repr(C)]
                    struct $variant {
                        $( $member: $ty ),*
                    }
                    $variant { $( $member ),* }
                }, $counter::$variant as _),
            ]
            $( $rest )*
        )
    };
    (@inner $tagged:expr, $enum:ident, $counter:ident [$($tt:tt)*] $variant:ident { $($member:ident: $ty:ty,)* }, $($rest:tt)* ) => {
        $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $tt )*
                $enum::$variant { $( $member ),* } => $crate::TaggedBox::new({
                    #[repr(C)]
                    struct $variant {
                        $( $member: $ty ),*
                    }
                    $variant { $( $member ),* }
                }, $counter::$variant as _),
            ]
            $( $rest )*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident [$($finished:tt)*] $variant:ident, $($rest:tt)* ) => {
        $crate::__taggable_into_box!(@inner $tagged, $enum, $counter
            [
                $( $finished )*
                $enum::$variant => $crate::TaggedBox::dangling($counter::$variant as _),
            ]
            $( $rest )*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident [$($finished:tt)*]) => {
        match $tagged {
            $( $finished )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_from {
    (@inner $struct:ident, $enum:ident, $variant:ident($ty:ty), $($rest:tt)*) => {
        impl From<$ty> for $struct {
            #[inline]
            fn from(val: $ty) -> Self {
                #[allow(unused_imports)]
                use $crate::TaggableInner;

                Self {
                    value: $enum::$variant(val).into_tagged_box(),
                }
            }
        }

        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };

    (@inner $struct:ident, $enum:ident, $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        impl From<($( $ty, )*)> for $struct {
            #[inline]
            fn from(tuple: ($( $ty, )*)) -> Self {
                #![allow(unused_imports, unused_variables)]
                use $crate::TaggableInner;

                let variant = $crate::__expand_tuple!($enum::$variant, tuple, $($ty),*);
                Self {
                    value: variant.into_tagged_box(),
                }
            }
        }

        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };

    (@inner $struct:ident, $enum:ident, $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };
    (@inner $struct:ident, $enum:ident, $variant:ident { $($ident:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };

    (@inner $struct:ident, $enum:ident, $variant:ident, $($rest:tt)*) => {
        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };

    (@inner $struct:ident, $enum:ident,) => { };

    ($struct:ident, $enum:ident, $($rest:tt)*) => {
        $crate::__derive_from! { @inner $struct, $enum, $( $rest )* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __generate_const_sizes {
    ($($ty:ty),*) => {{
        $crate::__generate_const_sizes!(@inner [0] $( $ty, )*)
    }};

    (@inner [$($finished:tt)*] $ty:ty, $($rest:ty,)*) => {
        $crate::__generate_const_sizes!(@inner
            [
            $( $finished )* + {
                const __NUMBER_VAR_BITS: usize = core::mem::size_of::<$ty>() * 8;
                __NUMBER_VAR_BITS
            }]
            $( $rest, )*
        )
    };

    (@inner [$($finished:tt)*]) => {
        $( $finished )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __boxed_into_inner {
    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                if $crate::__generate_const_sizes!($ty) <= $crate::discriminant::DISCRIMINANT_BITS {
                    todo!("Store small variables directly in the pointer")
                } else {
                    $enum::$variant($crate::TaggedBox::into_inner::<$ty>($tagged))
                }
            },
        ] $( $rest )*);
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                #[allow(dead_code)]
                #[repr(C)]
                struct $variant($( $ty ),*);

                if $crate::__generate_const_sizes!($( $ty ),*) <= $crate::discriminant::DISCRIMINANT_BITS {
                    todo!("Store small variables directly in the pointer")
                } else {
                    $crate::__expand_tuple!($enum::$variant, $crate::TaggedBox::into_inner::<$variant>($tagged), $($ty),*)
                }
            },
        ] $( $rest )*);
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                // TODO: Miniscule pointer storage can be preformed here too
                #[repr(C)]
                struct $variant {
                    $( $ident: $ty ),*
                }
                let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                $enum::$variant { $( $ident ),* }
            },
        ] $( $rest )*);
    };
    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                #[repr(C)]
                struct $variant {
                    $( $ident: $ty ),*
                }
                let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                $enum::$variant { $( $ident ),* }
            },
        ] $( $rest )*);
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => $enum::$variant,
        ] $( $rest )*);
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, [$($tt:tt)*]) => {
        #[allow(unused_parens)]
        match $tagged.discriminant() {
            $( $tt )*
            _ => panic!("Attempted to create an enum variant from a discriminant that doesn't exist!"),
        }
    };

    ($tagged:expr, $enum:ident, $counter:ident, $($rest:tt)*) => {
        $crate::__boxed_into_inner!(@inner $tagged, $enum, $counter, [] $( $rest )*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_tagged_box {
    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, $($rest:tt)*) => {
        $crate::__from_tagged_box!(@inner $tagged, $enum, $counter, $total_variants, [] $( $rest )*)
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            @inner
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    $enum::$variant($crate::TaggedBox::into_inner::<$ty>($tagged))
                },
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            @inner
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    $crate::__expand_tuple!($enum::$variant, $crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged), $( $ty ),*)
                },
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            @inner
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                    $enum::$variant { $( $ident ),* }
                },
            ] $($rest)*
        )
    };
    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            @inner
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                    $enum::$variant { $( $ident ),* }
                },
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            @inner
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => $enum::$variant,
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*]) => {
        #[allow(unused_parens)]
        match $tagged.discriminant() {
            $( $tt )*

            discriminant => {
                #[allow(non_upper_case_globals)]
                panic!(
                    "The number of variants in `{}` is {}, but a variant by the discriminant of {} was attempted to be created",
                    stringify!($enum),
                    $total_variants,
                    discriminant
                );
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ref_from_tagged {
    ($tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(@inner $tagged, $callback, $enum, $counter, $total_variants, [] $( $rest )*)
    };

    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            @inner
            $tagged,
            $callback,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = core::mem::ManuallyDrop::new($enum::$variant($tagged.as_ptr::<$ty>().read()));
                    ($callback)(&variant);
                }
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            @inner
            $tagged,
            $callback,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    use $crate::__expand_tuple;
                    let variant = core::mem::ManuallyDrop::new(__expand_tuple!(
                        $enum::$variant,
                        core::mem::ManuallyDrop::new($tagged.as_ptr::<($( $ty, )*)>().read()),
                        $($ty),*
                    ));
                    ($callback)(&variant);
                }
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            @inner
            $tagged,
            $callback,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $tagged.as_ptr::<$variant>().read();
                    let variant = core::mem::ManuallyDrop::new($enum::$variant { $( $ident ),* });
                    ($callback)(&variant);
                }
            ] $($rest)*
        )
    };
    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            @inner
            $tagged,
            $callback,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $tagged.as_ptr::<$variant>().read();
                    let variant = core::mem::ManuallyDrop::new($enum::$variant { $( $ident ),* });
                    ($callback)(&variant);
                }
            ] $($rest)*
        )
    };

    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            @inner
            $tagged,
            $callback,
            $enum,
            $counter,
            $total_variants,
            [
                $( $tt )*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = $enum::$variant;
                    ($callback)(&variant);
                }
            ] $( $rest )*
        )
    };

    (@inner $tagged:expr, $callback:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($rest:tt)*]) => {
        #[allow(unused_parens)]
        match $tagged.discriminant() {
            $( $rest )*

            discriminant => {
                #[allow(non_upper_case_globals)]
                panic!(
                    "The number of variants in `{}` is {}, but a variant by the discriminant of {} was attempted to be referenced",
                    stringify!($enum),
                    $total_variants,
                    discriminant
                );
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __counter_enum {
    (@inner [$($finished:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__counter_enum!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__counter_enum!(@inner [$( $finished )* $variant,] $( $rest )*)
    };
    (@inner [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__counter_enum!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($finished:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__counter_enum!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($finished:tt)*]) => {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        enum __tagged_box_enum_counter {
            $( $finished )*
        }
    };

    (@inner [$($finished:tt)*] $($tt:tt)*) => {
        $crate::__counter_enum!(@error stringify!($( $tt )*))
    };

    (@error $code:literal) => {
        compile_error!("Invalid enum definition: {}", $literal);
    };

    ($($rest:tt)*) => {
        $crate::__counter_enum!(@inner [] $( $rest )*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __count_variants {
    (@inner [$($finished:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__count_variants!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__count_variants!(@inner [$( $finished )* $variant,] $( $rest )*)
    };
    (@inner [$($finished:tt)*] $variant:ident { $($member:ident: $ty:ty,)* }, $($rest:tt)*) => {
        $crate::__count_variants!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($finished:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__count_variants!(@inner [$( $finished )* $variant,] $( $rest )*)
    };

    (@inner [$($variant:ident,)*]) => {
        [$( stringify!($variant) ),*].len()
    };

    ($($rest:tt)*) => {
        $crate::__count_variants!(@inner [] $( $rest )*)
    };
}

// TODO: Finish verifier macro
#[doc(hidden)]
#[macro_export]
macro_rules! __verify_variants {
    ($variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__verify_variants!($( $rest )*)
    };
    ($variant:ident($($ty:ty,)*), $($rest:tt)*) => {
        $crate::__verify_variants!($( $rest )*)
    };
}

/// Constructs a wrapper type and an associated enum to be stored as a [`TaggedBox`] that
/// can be used safely.  
/// For more implementation details, see [manually implementing a tagged enum]
///
/// # Example Usage
///
/// ```rust
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::tagged_box;
/// #
/// tagged_box! {
///     #[derive(Debug, Clone, PartialEq)] // This will be applied to both the inner enum and outer container
///     struct Container, enum Item {
///         Integer(i32),
///         Numbers(usize, f32, u8),
///         Unsigned {
///             big: u128,
///             large: u64,
///             medium: u32,
///             small: u16,
///             tiny: u8,
///         },
///     }
/// }
/// ```
///
/// Note: The number of variants must be <= [`MAX_DISCRIMINANT`]
///
/// This will create a struct `Container` and an enum `Item`. Expanded, they will look like this:
///
/// ```rust
/// # extern crate alloc;
/// # use alloc::string::String;
/// # use tagged_box::{TaggableInner, TaggedBox};
/// # use core::mem::ManuallyDrop;
/// #
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
/// // ..Omitted some generated code
/// # // Have to implement this to avoid compile error
/// # impl TaggableInner for Item {
/// #     fn into_tagged_box(self) -> TaggedBox<Self> {
/// #         match self {
/// #             Self::Integer(int) => TaggedBox::new(int, 0),
/// #             Self::Boolean(boolean) => TaggedBox::new(boolean, 1),
/// #             Self::String(string) => TaggedBox::new(string, 2),
/// #         }
/// #     }
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
/// #
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
/// #
/// use tagged_box::TaggableContainer;
///
/// let container = Container::from(String::from("Hello from tagged-box!"));
/// assert_eq!(container.into_inner(), Item::String(String::from("Hello from tagged-box!")));
/// ```
///
/// [`TaggedBox`]: crate::TaggedBox
/// [manually implementing a tagged enum]: crate::manually_impl_enum
/// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
/// [`into_inner`]: crate::TaggableContainer#into_inner
/// [`TaggableContainer`]: crate::TaggableContainer
#[macro_export]
macro_rules! tagged_box {
    (
        $( #[$meta:meta] )*
        $struct_vis:vis struct $struct:ident, $enum_vis:vis enum $enum:ident {
            $($variants:tt)+
        }
    ) => {
        $( #[$meta] )*
        #[repr(transparent)]
        $struct_vis struct $struct {
            value: $crate::TaggedBox<$enum>,
        }

        impl $crate::TaggableContainer for $struct {
            type Inner = $enum;

            fn into_inner(self) -> $enum {
                $crate::__counter_enum! {
                    $( $variants )+
                }

                // Safety: The generated discriminants and their associated variants should be valid, as
                // they are macro generated. As such, when calling `into_inner` the requested type should
                // be valid for the tagged pointer
                unsafe {
                    $crate::__boxed_into_inner!(self.value, $enum, __tagged_box_enum_counter, $( $variants )*)
                }
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

        $crate::__derive_from! {
            $struct, $enum, $( $variants )+
        }

        $crate::__make_enum! {
            $enum_vis, $enum
            [$( $meta )*]
            $( $variants )+
        }

        impl $crate::TaggableInner for $enum {
            fn into_tagged_box(self) -> $crate::TaggedBox<Self> {
                $crate::__counter_enum! {
                    $( $variants )+
                }

                $crate::__taggable_into_box!( self, $enum, __tagged_box_enum_counter, $( $variants )+)
            }

            fn from_tagged_box(tagged: $crate::TaggedBox<$enum>) -> Self {
                // Safety: The discriminants and the enum variants should be synced, as they are all
                // generated by a macro. Therefore, when `tagged`'s discriminant and the current discriminant
                // are the same, the variant should be valid for the data stored at `tagged`
                unsafe {
                    $crate::__counter_enum! {
                        $( $variants )+
                    }

                    const __TAGGED_BOX_TOTAL_VARIANTS: usize = $crate::__count_variants!($( $variants )+);
                    $crate::__from_tagged_box!(
                        tagged,
                        $enum,
                        __tagged_box_enum_counter,
                        __TAGGED_BOX_TOTAL_VARIANTS,
                        $( $variants )*
                    )
                }
            }

            unsafe fn ref_from_tagged_box<F>(tagged: &$crate::TaggedBox<$enum>, callback: F)
            where
                F: FnOnce(&$enum),
            {
                $crate::__counter_enum! {
                    $( $variants )+
                }

                const __TAGGED_BOX_TOTAL_VARIANTS: usize = $crate::__count_variants!($( $variants )+);
                $crate::__ref_from_tagged!(
                    tagged,
                    callback,
                    $enum,
                    __tagged_box_enum_counter,
                    __TAGGED_BOX_TOTAL_VARIANTS,
                    $( $variants )*
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn variants_compile() {
        tagged_box! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            struct Container, enum Item {
                Unit,
                Something(i32),
                EmptyTuple(),
                ManyThings(usize, bool, isize),
                OrphanStruct {
                    thing: usize,
                    other_thing: bool
                },
            }
        }
    }
}
