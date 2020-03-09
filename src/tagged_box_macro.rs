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
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident($($ty),*)] $($tt)*);
    };

    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*] $ident:ident { $($var:ident: $ty:ty),* } $($tt:tt)*) => {
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident { $($var: $ty),* }] $($tt)*);
    };

    ([[$($meta:meta),*] $vis:vis, $enum:ident] [$($finished:tt)*] $ident:ident $($tt:tt)*) => {
        $crate::__make_enum!([[$($meta)*] $vis, $enum] [$($finished)* $ident] $($tt)*);
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
macro_rules! __identify {
    ([$($prev:tt)*] $next:ty, $($rest:tt)*) => {
        $crate::__identify!([$( $prev )*, x] $( $rest )*)
    };

    ([$($prev:tt)*]) => {
        $( $prev )*
    };

    (@tuple [$($prev:tt)*] $next:ty, $($rest:tt)*) => {
        $crate::__identify!(@tuple [$( $prev )* x] $( $rest )*)
    };

    (@tuple [$($prev:tt)*]) => {
        ($( $prev )*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __taggable_into_box {
    ($self:expr, $enum:ident, $counter:ident, [$($tt:tt)*] $ident:ident ($($ty:ty),*) $($rest:tt)* ) => {
        $crate::__taggable_into_box!($self, $enum, $counter,
            [
                $($tt)*
                $enum::$ident($crate::__identify!([] $( $ty, )*)) =>
                    $crate::TaggedBox::new(($crate::__identify!([] $( $ty, )*)), $counter::$ident as _),
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
macro_rules! __derive_from {
    ($struct:ident, $enum:ident, $variant:ident, [($($ty:ty),*)]) => {
        #[allow(unused_parens)]
        impl From<($( $ty ),*)> for $struct {
            #[inline]
            fn from(value: ($( $ty ),*)) -> Self {
                use $crate::TaggableInner;
                let $crate::__identify!(@tuple [] $( $ty, )*) = value;
                let variant = $enum::$variant($crate::__identify!([] $( $ty, )*));

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
    ($self:ident, $variant:ident, $tagged:expr, [($($ty:ty),*)]) => {
        $self::$variant($crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged).read())
    };

    ($self:ident, $variant:ident, $tagged:expr, [{ $($ident:ident: $ty:ty),* }]) => {
        $self::$variant({
            #[repr(C)]
            struct $variant {
                $( $ident: $ty ),*
            }
            let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged).read();
            $self::$variant { $( $ident ),* }
        })
    };

    ($self:ident, $variant:ident, $tagged:expr, []) => {
        $self::$variant
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __boxed_into_inner {
    ($enum:ident, $self:expr, $counter:ident, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, self, [
            $($tt)*
            discrim if discrim == $counter::$variant as _ => {
                $self::$variant($crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged))
            },
        ] $($rest)*);
    };

    ($enum:ident, $self:expr, $counter:ident, [$($tt:tt)*] $variant:ident { $($ident:ident: $ty:ty),* }, $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, self, [
            $($tt)*
            discrim if discrim == $counter::$variant as _ => self::$variant({
                #[repr(C)]
                struct $variant {
                    $( $ident: $ty ),*
                }
                let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                $self::$variant { $( $ident ),* }
            }),
        ] $($rest)*);
    };

    ($enum:ident, $self:expr, $counter:ident, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__boxed_into_inner!($enum, self, [
            $($tt)*
            discrim if discrim == $counter::$variant as _ => $self::$variant,
        ] $($rest)*);
    };

    ($enum:ident, $self:expr, $counter:ident, [$($tt:tt)*] ) => {
        #[allow(unused_parens)]
        match $self.value.discriminant() {
            $( $tt )*
            _ => panic!("Attempted to create an enum variant from a discriminant that doesn't exist!"),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __from_tagged_box {
    ($tagged:expr, $enum:ident, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident($($ty:ty),*), $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            $tagged,
            $enum,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    $self::$variant($crate::TaggedBox::into_inner::<($( $ty ),*)>($tagged))
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
                discrim if discrim == $counter::$variant as _ => self::$variant({
                    #[repr(C)]
                    struct $variant {
                        $( $ident: $ty ),*
                    }
                    let $variant { $( $ident ),* } = $crate::TaggedBox::into_inner::<$variant>($tagged);
                    $self::$variant { $( $ident ),* }
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
                discrim if discrim == $counter::$variant as _ => $self::$variant,
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
    ($enum:ident, $tagged:expr, $counter:ident, $total_variants:expr, [$($tt:tt)*] $variant:ident, $($rest:tt)*) => {
        $crate::__from_tagged_box!(
            $enum,
            $tagged,
            $counter,
            $total_variants,
            [
                $($tt)*
                discrim if discrim == $counter::$variant as _ => {
                    let variant = $crate::__ptr_read_box!(Self, $variant, tagged, [$( $tt )*]);
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
                    $crate::__boxed_into_inner!($enum, self, __tagged_box_enum_counter, $($variant $($tt)*)*)
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
                        [] $($variant $($tt)*)*
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
                    [] $( $variant $($tt)*),*
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
                OrphanStruct[{ thing: usize, other_thing: bool }],
            }
        }
    }
}
