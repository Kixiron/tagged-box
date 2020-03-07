#![no_std]

//! Implementations of tagged pointers and boxes, the main usage of this crate is from the [`tagged_box`] macro.  
//!
//! This crate has a few features:
//! - `48bits` (On by default): 48 bits of reserved pointer, 16 bits for data
//! - `49bits`: 49 bits of reserved pointer, 15 bits for data
//! - `50bits`: 50 bits of reserved pointer, 14 bits for data
//! - `51bits`: 51 bits of reserved pointer, 13 bits for data
//! - `52bits`: 52 bits of reserved pointer, 12 bits for data
//! - `53bits`: 53 bits of reserved pointer, 11 bits for data
//! - `54bits`: 54 bits of reserved pointer, 10 bits for data
//! - `55bits`: 55 bits of reserved pointer, 9 bits for data
//! - `56bits`: 56 bits of reserved pointer, 8 bits for data
//! - `57bits`: 57 bits of reserved pointer, 7 bits for data
//!
//! However, only one of these may be active at a time, otherwise a `compile_error` will be emitted
//!
//! [`tagged_box`]: crate::tagged_box

// `no_std` is supported, but the `alloc` crate is required, as
// tagged values are allocated to the heap.
extern crate alloc;

// TODO: Much of this crate can be `const fn` once certain features from #57563 come through
// Needed features from #57563:
//    - Dereferencing raw pointers via #51911
//    - Bounds via rust-lang/rfcs#2632
//    - &mut T references and borrows via #57349
//    - Control flow via #49146
//    - Panics via #51999
//
// https://github.com/rust-lang/rust/issues/57563

// 32bit arches use all 32 bits of their pointers, so an extra 16bit tag will have to be added.
// Any benefit from tagging is kinda lost, but it's worth it to allow compatibility for this crate
// between 32bit and 64bit arches.
//
//
// Possible `TaggedPointer` structure on 32bit arches:
// ```rust
// pub struct TaggedPointer {
//     ptr: usize,
//     tag: u8,
// }
// ```
//
// Note: Tagging of the lower bits is also possible due to alignment, but that only allows for 8 variants at best.
// this is better than nothing though, and should be implemented
#[cfg(target_pointer_width = "32")]
compile_error!("Tagged 32 bit pointers are not currently supported");

// Only pointer widths of 64bits and 32bits will be supported, unless 128bits ever becomes mainstream,
// but we'll cross that bridge when we get to it.
#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("Only pointer widths of 64 and 32 will be supported");

/// Macro to create a compile error if none or more than one feature are enabled
/// TODO: This only detects all or none as of now
macro_rules! generate_compile_error {
    ($($feature:literal),+) => {
        #[cfg(any(
            all($( feature = $feature ),+),
            not(any($( feature = $feature ),+)),
        ))]
        compile_error!("Please choose one of the reserved bit width features");
    }
}

generate_compile_error!(
    "48bits", "49bits", "50bits", "51bits", "52bits", "53bits", "54bits", "55bits", "56bits",
    "57bits"
);

/// Implement various formatting traits on included structs
macro_rules! impl_fmt {
    ($ty:ty => $($fmt:ident),+) => {
        $(
            impl fmt::$fmt for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::$fmt::fmt(&self.as_usize(), f)
                }
            }
        )+
    };

    // Special case because I don't like rewriting stuff
    (impl[T: TaggableInner] $ty:ty => $($fmt:ident),+) => {
        $(
            impl<T: TaggableInner> fmt::$fmt for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::$fmt::fmt(&self.as_usize(), f)
                }
            }
        )+
    };

    // General-purpose arm, used as follows:
    // ```rust
    // impl_fmt! {
    //     impl[T: Debug] TaggedBox<T> => LowerHex,
    //     impl[T] TaggedBox<T> => UpperHex
    // }
    // ```
    ($(impl[$($generic:tt)*] $ty:ty => $fmt:ident),+) => {
        $(
            impl<$($generic)*> fmt::$fmt for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::$fmt::fmt(&self.as_usize(), f)
                }
            }
        )+
    };
}

pub mod discriminant;
pub mod manually_impl_enum;
mod taggable;
mod tagged_box;
#[macro_use]
mod tagged_box_macro;
mod tagged_pointer;

pub use crate::tagged_box::TaggedBox;
pub use discriminant::Discriminant;
pub use taggable::{TaggableContainer, TaggableInner};
pub use tagged_pointer::TaggedPointer;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};
    use core::mem::ManuallyDrop;

    #[test]
    fn tagged_pointer() {
        let integer = 100i32;
        let discriminant = 10;

        let ptr = TaggedPointer::new(&integer as *const i32 as usize, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_usize(), &integer as *const i32 as usize);
    }

    #[test]
    fn max_tagged_pointer() {
        let integer = 100usize;
        let discriminant = discriminant::MAX_DISCRIMINANT;

        let ptr = TaggedPointer::new(&integer as *const usize as usize, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_usize(), &integer as *const usize as usize);
    }

    #[test]
    fn inner_mutability_compat() {
        use core::cell::Cell;

        tagged_box! {
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
            struct Container, enum Item {
                Cell(Cell<usize>),
            }
        }

        let min = Container::from(Cell::new(0));
        let max = Container::from(Cell::new(usize::max_value()));

        assert!(min < max);
        assert!(min.clone() < max.clone());

        assert!(min <= max);
        assert!(min.clone() <= max.clone());
    }

    #[test]
    fn tagged_box() {
        #[derive(Clone)]
        struct Container {
            tagged: TaggedBox<Value>,
        }

        impl TaggableContainer for Container {
            type Inner = Value;

            fn into_inner(self) -> Self::Inner {
                unsafe {
                    match self.tagged.discriminant() {
                        0 => Value::I32(TaggedBox::into_inner(self.tagged)),
                        1 => Value::Bool(TaggedBox::into_inner(self.tagged)),
                        _ => unreachable!(),
                    }
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum Value {
            I32(i32),
            Bool(bool),
        }

        impl TaggableInner for Value {
            fn into_tagged_box(self) -> TaggedBox<Self> {
                match self {
                    Self::I32(int) => TaggedBox::new(int, 0),
                    Self::Bool(boolean) => TaggedBox::new(boolean, 1),
                }
            }

            fn from_tagged_box(tagged: TaggedBox<Self>) -> Self {
                unsafe {
                    match tagged.discriminant() {
                        0 => Value::I32(TaggedBox::into_inner(tagged)),
                        1 => Value::Bool(TaggedBox::into_inner(tagged)),
                        _ => unreachable!(),
                    }
                }
            }

            unsafe fn ref_from_tagged_box<F>(tagged: &TaggedBox<Self>, callback: F)
            where
                F: FnOnce(&Self),
            {
                let value = ManuallyDrop::new(match tagged.discriminant() {
                    0 => Value::I32(*tagged.as_ptr::<i32>()),
                    1 => Value::Bool(*tagged.as_ptr::<bool>()),
                    _ => unreachable!(),
                });

                (callback)(&value);
            }
        }

        let int_container = Container {
            tagged: TaggedBox::new::<i32>(110, 0),
        };
        let bool_container = Container {
            tagged: TaggedBox::new::<bool>(true, 1),
        };

        assert_eq!(int_container.tagged.discriminant(), 0);
        assert_eq!(bool_container.tagged.discriminant(), 1);

        assert_eq!(unsafe { int_container.tagged.as_ref::<i32>() }, &110i32);
        assert_eq!(unsafe { bool_container.tagged.as_ref::<bool>() }, &true);

        assert_eq!(int_container.clone().into_inner(), Value::I32(110));
        assert_eq!(bool_container.clone().into_inner(), Value::Bool(true));

        assert_eq!(int_container.into_inner(), Value::I32(110));
        assert_eq!(bool_container.into_inner(), Value::Bool(true));
    }

    #[test]
    fn storage() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct CustomStruct {
            int: usize,
            boolean: bool,
        }

        tagged_box! {
            #[derive(Debug, Clone, PartialEq)]
            struct Outer, enum Inner {
                Float(f32),
                Int(i32),
                Byte(u8),
                Unit(()),
                Bool(bool),
                Array([u8; 8]),
                Vector(Vec<u8>),
                CustomStruct(CustomStruct),
            }
        }

        assert_eq!(Outer::from(10.0f32).into_inner(), Inner::Float(10.0));
        assert_eq!(Outer::from(100i32).into_inner(), Inner::Int(100));
        assert_eq!(Outer::from(10u8).into_inner(), Inner::Byte(10));
        assert_eq!(Outer::from(()).into_inner(), Inner::Unit(()));
        assert_eq!(Outer::from(true).into_inner(), Inner::Bool(true));
        assert_eq!(Outer::from([100; 8]).into_inner(), Inner::Array([100; 8]));
        assert_eq!(
            Outer::from(vec![100; 10]).into_inner(),
            Inner::Vector(vec![100; 10])
        );
        assert_eq!(
            Outer::from(CustomStruct {
                int: 100_000,
                boolean: false
            })
            .into_inner(),
            Inner::CustomStruct(CustomStruct {
                int: 100_000,
                boolean: false
            })
        );
    }
}
