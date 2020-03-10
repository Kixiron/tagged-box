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
///         Boolean(bool),
///         String(String),
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

#[doc(hidden)]
#[macro_export]
macro_rules! __make_enum {
    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*] $ident:ident($($ty:ty),*) $($tt:tt)*) => {
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident($($ty),*),] $($tt)*);
    };

    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*] $ident:ident { $($var:ident: $ty:ty),* } $($tt:tt)*) => {
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident { $($var: $ty),* },] $($tt)*);
    };

    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*] $ident:ident $($tt:tt)*) => {
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident,] $($tt)*);
    };

    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*]) => {
        $( #[$meta] )*
        #[allow(unused_parens)]
        $vis enum $enum {
            $( $finished )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __taggable_into_box {
    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $ident:ident ($ty:ty) $($rest:tt)* ) => {
        $crate::__taggable_into_box!($self, $enum, $counter,
            [
                $($tt)*
                $enum::$ident(var) => {
                    $crate::TaggedBox::new(var, $counter::$ident as _)
                },

            ] $($rest)*
        )
    };

    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $ident:ident ($($ty:ty),*) $($rest:tt)* ) => {
        $crate::__taggable_into_box!($self, $enum, $counter,
            [
                $($tt)*
                $enum::$ident(tuple) => {
                    $crate::TaggedBox::new($crate::__expand_tuple!(@paren tuple, $($ty),*), $counter::$ident as _)
                },

            ] $($rest)*
        )
    };

    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $ident:ident { $($member:ident: $ty:ty),* } $($rest:tt)* ) => {
        $crate::__taggable_into_box!($self, $enum, $counter,
            [
                $($tt)*
                $enum::$variant { $($member),* } => $crate::TaggedBox::new({
                    #[repr(C)]
                    struct $variant {
                        $( $member: $ty ),*
                    }
                    $variant { $( $member ),* }
                }, $counter::$variant as _),
            ] $($rest)*
        )
    };

    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $ident:ident ($($ty:ty),*) $($rest:tt)* ) => {
        $crate::__taggable_into_box!($self, $enum, $counter,
            [
                $($tt)*
                $enum::$variant => $crate::TaggedBox::dangling($counter::$variant as _),
            ] $($rest)*
        )
    };

    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*]) => {
        match $self {
            $( $tt )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __expand_tuple {
    (@paren $tuple:expr, $a:ty) => {
        ($tuple.0,)
    };
    (@paren $tuple:expr, $a:ty, $b:ty) => {
        ($tuple.0, $tuple.1)
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty) => {
        ($tuple.0, $tuple.1, $tuple.2)
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty) => {
        ($tuple.0, $tuple.1, $tuple.2, $tuple.3)
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        ($tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4)
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        ($tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5)
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty) => {
        (
            $tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5, $tuple.6,
        )
    };
    (@paren $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty) => {
        (
            $tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5, $tuple.6, $tuple.7,
        )
    };

    (@raw $variant:path, $tuple:expr, $a:ty) => {
        $variant($tuple.0)
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty) => {
        $variant($tuple.0, $tuple.1)
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty, $c:ty) => {
        $variant($tuple.0, $tuple.1, $tuple.2)
    };
    (@raw $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty) => {
        $variant($tuple.0, $tuple.1, $tuple.2, $tuple.3)
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        $variant($tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4)
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        $variant($tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5)
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty) => {
        $variant(
            $tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5, $tuple.6,
        )
    };
    (@raw $variant:path, $tuple:expr, $a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty) => {
        $variant(
            $tuple.0, $tuple.1, $tuple.2, $tuple.3, $tuple.4, $tuple.5, $tuple.6, $tuple.7,
        )
    };
}

/*
TODO: Make this work
macro_rules! generate_tuple_expansion {
    (@expand $($fields:literal,)*) => {
        generate_tuple_expansion!(($) [] [$sign tuple:expr,] []
            $( $fields, )*
        );
    };

    (($sign:tt) [$($rest:tt)*] [$($match:tt)*] [$($access:tt)*] $field:tt, $($fields:tt,)*) => {
        generate_tuple_expansion!(($sign) [
                $( $rest )*
                ($( $match, )* $sign $field:ty) => {{
                    $( $access, )* $sign tuple.$field
                }};
            ]
            [$( $match, )* $sign $field:ty]
            [$( $access, )* $sign tuple.$field]
            $( $fields ),*
        );
    };

    (($sign:tt) [$($rest:tt)*] [$($match:tt)*] [$($access:tt)*]) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_tuple {
            $( $rest )*
        }
    };
}

trace_macros!(true);
generate_tuple_expansion!(
    @expand
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
);
trace_macros!(false);
*/

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_from {
    ($struct:ident, $enum:ident, $variant:ident, [($ty:ty)]) => {
        impl From<$ty> for $struct {
            #[inline]
            fn from(val: $ty) -> Self {
                use $crate::TaggableInner;

                Self {
                    value: $enum::$variant(val).into_tagged_box(),
                }
            }
        }
    };

    ($struct:ident, $enum:ident, $variant:ident, [($($ty:ty),*)]) => {
        impl From<($( $ty, )*)> for $struct {
            #[inline]
            fn from(tuple: ($( $ty, )*)) -> Self {
                use $crate::TaggableInner;

                let variant = $crate::__expand_tuple!(@raw $enum::$variant, tuple, $($ty),*);
                Self {
                    value: variant.into_tagged_box(),
                }
            }
        }
    };

    ($struct:ident, $enum:ident, $variant:ident, [{ $($ident:ident: $ty:ty),* }]) => { };

    ($struct:ident, $enum:ident, $variant:ident, []) => { };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ptr_read_box {
    ($enum:ident, $variant:ident, $tagged:expr, [($ty:ty)]) => {
        $enum::$variant($crate::TaggedBox::into_inner::<$ty>($tagged).read())
    };

    ($enum:ident, $variant:ident, $tagged:expr, [($($ty:ty),*)]) => {{
        let tuple = $crate::TaggedBox::into_inner::<($( $ty, )*)>($tagged).read();
        $crate::__expand_tuple!(@raw $enum::$variant, tuple, $($ty),*)
    }};

    ($enum:ident, $variant:ident, $tagged:expr, [{ $($ident:ident: $ty:ty),* }]) => {
        $enum::$variant({
            #[repr(C)]
            struct $variant {
                $( $ident: $ty ),*
            }
            let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged).read();
            $enum::$variant { $( $ident ),* }
        })
    };

    ($enum:ident, $variant:ident, $tagged:expr, []) => {
        $enum::$variant
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __boxed_into_inner {
    ($enum:ident, $tagged:expr, $counter:ident, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, $tagged, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                $enum::$variant($crate::TaggedBox::into_inner::<$ty>($tagged))
            },
        ] $( $rest )*);
    };

    ($enum:ident, $tagged:expr, $counter:ident, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, $tagged, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => {
                let tuple = $crate::TaggedBox::into_inner::<($( $ty, )*)>($tagged);
                $crate::__expand_tuple!(@raw $enum::$variant, tuple, $($ty),*)
            },
        ] $( $rest )*);
    };

    ($enum:ident, $tagged:expr, $counter:ident, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, $tagged, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => $enum::$variant({
                #[repr(C)]
                struct $variant {
                    $( $ident: $ty ),*
                }
                let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                $enum::$variant { $( $ident ),* }
            }),
        ] $( $rest )*);
    };

    ($enum:ident, $tagged:expr, $counter:ident, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, $tagged, $counter, [
            $( $tt )*
            discrim if discrim == $counter::$variant as _ => $enum::$variant,
        ] $( $rest )*);
    };

    ($enum:ident, $tagged:expr, $counter:ident, [$($tt:tt)*]) => {
        #[allow(unused_parens)]
        match $tagged.discriminant() {
            $( $tt )*
            _ => panic!("Attempted to create an enum variant from a discriminant that doesn't exist!"),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_tagged_box {
    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__from_tagged_box!(
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

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let tuple = $crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged);
                    $crate::__expand_tuple!(@raw $enum::$variant, tuple, $( $ty ),*)
                },
            ] $($rest)*
        )
    };

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => $enum::$variant({
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                    $enum::$variant { $( $ident ),* }
                }),
            ] $($rest)*
        )
    };

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
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

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*]) => {
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
    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($ty:ty), $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = $crate::__ptr_read_box!($enum, $variant, $tagged, [$( $tt )*]);
                    (callback)(&variant);
                },
            ] $($rest)*
        )
    };

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let tuple = $crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged);
                    $crate::__expand_tuple!(@raw $enum::$variant, tuple, $( $ty ),*)
                },
            ] $($rest)*
        )
    };

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = $crate::__ptr_read_box!($enum, $variant, $tagged, [$( $tt )*]);
                    (callback)(&variant);
                },
            ] $($rest)*
        )
    };

    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__ref_from_tagged!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = $crate::__ptr_read_box!($enum, $variant, $tagged, [$( $tt )*]);
                    (callback)(&variant);
                },
            ] $($rest)*
        )
    };

    ($enum:ident, $tagged:expr, $counter:ident, $total_variants:expr, [$($tt:tt)*]) => {
        #[allow(unused_parens)]
        match $tagged.discriminant() {
            $( $tt )*

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

#[macro_export]
macro_rules! tagged_box {
    (
        $( #[$meta:meta] )*
        $struct_vis:vis struct $struct:ident, $enum_vis:vis enum $enum:ident {
            $( $variant:ident[$($tt:tt)*], )+
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
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                enum __tagged_box_enum_counter {
                    $( $variant ),+
                }

                // Safety: The generated discriminants and their associated variants should be valid, as
                // they are macro generated. As such, when calling `into_inner` the requested type should
                // be valid for the tagged pointer
                unsafe {
                    $crate::__boxed_into_inner!($enum, self.value, __tagged_box_enum_counter, [] $($variant $($tt)*,)*)
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

        $(
            $crate::__derive_from!($struct, $enum, $variant, [$( $tt )*]);
        )+

        $crate::__make_enum!(
            [
                [$($meta),*]
                $enum_vis, $enum
            ] [] $( $variant$($tt)* )*
        );

        impl $crate::TaggableInner for $enum {
            fn into_tagged_box(self) -> $crate::TaggedBox<Self> {
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                enum __tagged_box_enum_counter {
                    $( $variant ),+
                }

                $crate::__taggable_into_box!(
                    self,
                    Self,
                    __tagged_box_enum_counter,
                    [] $($variant $($tt)*)*
                )
            }

            fn from_tagged_box(tagged: $crate::TaggedBox<$enum>) -> Self {
                // Safety: The discriminants and the enum variants should be synced, as they are all
                // generated by a macro. Therefore, when `tagged`'s discriminant and the current discriminant
                // are the same, the variant should be valid for the data stored at `tagged`
                unsafe {
                    #[doc(hidden)]
                    #[allow(non_camel_case_types)]
                    enum __tagged_box_enum_counter {
                        $( $variant ),+
                    }

                    const __tagged_box_total_variants: usize = [$( stringify!($variant) ),+].len();
                    $crate::__from_tagged_box!(
                        tagged,
                        Self,
                        __tagged_box_enum_counter,
                        __tagged_box_total_variants,
                        [] $($variant $($tt)*),*
                    )
                }
            }

            unsafe fn ref_from_tagged_box<F>(tagged: &$crate::TaggedBox<$enum>, callback: F)
            where
                F: FnOnce(&$enum),
            {
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                enum __tagged_box_enum_counter {
                    $( $variant ),+
                }

                const __tagged_box_total_variants: usize = [$( stringify!($variant) ),+].len();
                $crate::__ref_from_tagged!(
                    $enum,
                    tagged,
                    __tagged_box_enum_counter,
                    __tagged_box_total_variants,
                    [] $( $variant $($tt)*,)*
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tuple_variants() {
        tagged_box! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            struct Container, enum Item {
                Unit[],
                Something[(i32)],
                ManyThings[(usize, bool, isize)],
                OrphanStruct [{
                    thing: usize,
                    other_thing: bool
                }],
            }
        }
    }
}
