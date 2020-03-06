#![no_std]

//! Tagged boxes and pointers, main usage of this crate is from the [`tagged_box`] macro
//!
//! [`tagged_box`]: crate::tagged_box

// `no_std` is supported, but the `alloc` crate is required, as
// tagged values are allocated to the heap.
extern crate alloc;

use core::{
    alloc::Layout,
    cmp, fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ptr,
};

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
// Possible `TaggedPointer` structure on 32bit arches:
// ```rust
// pub struct TaggedPointer {
//     ptr: usize,
//     tag: u8,
// }
// ```
#[cfg(target_pointer_width = "32")]
compile_error!("Tagged 32 bit pointers are not currently supported");

// Only pointer widths of 64bits and 32bits will be supported, unless 128bits ever becomes mainstream,
// but we'll cross that bridge when we get to it.
#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
compile_error!("Only pointer widths of 64 and 32 will be supported");

// Pointers may either use 48bits or 57bits of a 64bit integer
#[cfg(any(
    all(feature = "48bits", feature = "57bits"),
    not(any(feature = "48bits", feature = "57bits"))
))]
compile_error!("Please choose either `48bits` or `57bits` as a feature");

#[cfg(feature = "57bits")]
mod discriminant {
    /// A discriminant stored in a [`TaggedPointer`], will hold an integer <= `128`  
    /// The width of a pointer that is reserved can be changed using the `48bits` and `57bits` features
    ///
    /// [`TaggedPointer`]: crate::TaggedPointer
    pub type Discriminant = u8;

    /// The maximum size that a discriminant can be, `128`
    pub(crate) const MAX_DISCRIMINANT: Discriminant = (2 * 2 * 2 * 2 * 2 * 2 * 2) - 1;
    /// The width of a pointer that is reserved, 57bits
    pub(crate) const POINTER_WIDTH: usize = 57;
    /// Masks the upper 7 bits of a pointer to remove the discriminant
    pub(crate) const DISCRIMINANT_MASK: usize = usize::max_value() >> 7;
}

#[cfg(feature = "48bits")]
mod discriminant {
    /// A discriminant stored in a [`TaggedPointer`], will hold an integer <= `65536`  
    /// The width of a pointer that is reserved can be changed using the `48bits` and `57bits` features
    ///
    /// [`TaggedPointer`]: crate::TaggedPointer
    pub type Discriminant = u16;

    #[allow(const_err)]
    /// The maximum size that a discriminant can be, `65536`
    pub(crate) const MAX_DISCRIMINANT: Discriminant = !0;
    /// The width of a pointer that is reserved, 48bits
    pub(crate) const POINTER_WIDTH: usize = 48;
    /// Masks the upper 16 bits of a pointer to remove the discriminant
    pub(crate) const DISCRIMINANT_MASK: usize = usize::max_value() >> 16;
}

pub use discriminant::Discriminant;
use discriminant::*;

/// A pointer that holds a data pointer plus additional data
///
/// Depending on which feature you have active, one of the following is true:
///
/// #### With the `48bits` feature
/// The upper 16 bits of the pointer will be used to store a `u16` of data
///
/// #### With the `57bits` feature
/// The upper 7 bits of the pointer will be used to store `2 ^ 7` of data
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TaggedPointer {
    /// The tagged pointer, the upper bits are used to store arbitrary data  
    /// With the `48bits` feature the upper 16 bits are usable, while with the
    /// `57bits` feature only the upper 7 bits are usable
    tagged_ptr: usize,
}

impl TaggedPointer {
    /// Create a new tagged pointer from a pointer and a discriminant
    ///
    /// # Panics
    /// Panics if `discriminant` is greater than `MAX_DISCRIMINANT`  
    /// `MAX_DISCRIMINANT` is `2 ^ 16` with the `48bits` feature and `2 ^ 7` with the `57bits` feature
    #[inline]
    pub fn new(ptr: usize, discriminant: Discriminant) -> Self {
        assert!(
            discriminant <= MAX_DISCRIMINANT,
            "Attempted to store a discriminant of {} while the max value is {}",
            discriminant,
            MAX_DISCRIMINANT,
        );

        Self {
            tagged_ptr: Self::store_discriminant(ptr, discriminant),
        }
    }

    /// Fetches the discriminant of the tagged pointer
    #[inline]
    pub const fn discriminant(&self) -> Discriminant {
        Self::fetch_discriminant(self.tagged_ptr)
    }

    /// Gains a reference to the inner value of the pointer
    ///
    /// # Safety
    /// The pointer given to [`TaggedPointer::new`] must be valid
    ///
    /// [`TaggedPointer::new`]: crate::TaggedPointer#new
    #[inline]
    pub unsafe fn as_ref<T>(&self) -> &T {
        &*(Self::strip_discriminant(self.tagged_ptr) as *const T)
    }

    /// Gains a mutable reference to the inner value of the pointer
    ///
    /// # Safety
    /// The pointer given to [`TaggedPointer::new`] must be valid
    ///
    /// [`TaggedPointer::new`]: crate::TaggedPointer#new
    #[inline]
    pub unsafe fn as_mut_ref<T>(&mut self) -> &mut T {
        &mut *(Self::strip_discriminant(self.tagged_ptr) as *mut T)
    }

    /// Returns the pointer as a usize, removing the discriminant
    #[inline]
    pub const fn as_usize(&self) -> usize {
        Self::strip_discriminant(self.tagged_ptr)
    }

    /// Returns the raw tagged pointer, without removing the discriminant
    ///
    /// # Warning
    ///
    /// Attempting to dereference this usize will not point to valid memory!
    #[inline]
    pub const fn as_raw_usize(&self) -> usize {
        self.tagged_ptr
    }

    /// Converts a tagged pointer into a raw pointer, removing the discriminant
    #[inline]
    pub const fn as_ptr<T>(&self) -> *const T {
        Self::strip_discriminant(self.tagged_ptr) as *const T
    }

    /// Converts a tagged pointer into a raw pointer, removing the discriminant
    #[inline]
    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        Self::strip_discriminant(self.tagged_ptr) as *mut T
    }

    const MASK: usize = !(1 << POINTER_WIDTH);

    /// Store a [`Discriminant`] into a tagged pointer
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub fn store_discriminant(pointer: usize, discriminant: Discriminant) -> usize {
        assert!(discriminant <= MAX_DISCRIMINANT);

        (pointer & TaggedPointer::MASK) | ((discriminant as usize) << POINTER_WIDTH)
    }

    /// Store a [`Discriminant`] into a tagged pointer    
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub const fn fetch_discriminant(pointer: usize) -> Discriminant {
        (pointer >> POINTER_WIDTH) as Discriminant
    }

    /// Strip the [`Discriminant`] of a tagged pointer, returning only the valid pointer as a usize
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub const fn strip_discriminant(pointer: usize) -> usize {
        pointer & DISCRIMINANT_MASK
    }
}

impl fmt::Debug for TaggedPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaggedPointer")
            .field("raw", &(self.as_raw_usize() as *const ()))
            .field("ptr", &self.as_ptr::<()>())
            .field("discriminant", &self.discriminant())
            .finish()
    }
}

impl fmt::Pointer for TaggedPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr::<()>(), f)
    }
}

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
    //     impl[T] TaggedBox<T> => LowerHex,
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

impl_fmt!(TaggedPointer => LowerHex, UpperHex, Binary, Octal);

/// A tagged box,
#[repr(transparent)]
pub struct TaggedBox<T: TaggableInner> {
    boxed: TaggedPointer,
    _type: PhantomData<T>,
}

impl<T: TaggableInner> TaggedBox<T> {
    #[inline]
    pub fn new<U>(val: U, discriminant: Discriminant) -> Self {
        let ptr = if mem::size_of::<U>() == 0 {
            ptr::NonNull::dangling().as_ptr()
        } else {
            let layout = Layout::new::<U>();

            // Safety: The allocation should be properly handled by alloc + layout,
            // and writing should be properly aligned
            unsafe {
                let ptr = alloc::alloc::alloc(layout) as *mut U;
                assert!(ptr as usize != 0);
                ptr.write(val);

                ptr
            }
        };

        Self {
            boxed: TaggedPointer::new(ptr as usize, discriminant),
            _type: PhantomData,
        }
    }

    /// Return a reference to the boxed value
    ///
    /// # Safety
    /// The type provided as `U` must be the same type as allocated by `new`
    #[inline]
    pub unsafe fn as_ref<U>(&self) -> &U {
        self.boxed.as_ref()
    }

    /// Return a mutable reference to the boxed value
    ///
    /// # Safety
    /// The type provided as `U` must be the same type as allocated by `new`
    #[inline]
    pub unsafe fn as_mut_ref<U>(&mut self) -> &mut U {
        self.boxed.as_mut_ref()
    }

    /// Return the boxed value
    ///
    /// # Safety
    /// The type provided as `U` must be the same type as allocated by `new`
    #[inline]
    pub unsafe fn into_inner<U>(tagged: Self) -> U {
        let mut tagged = ManuallyDrop::new(tagged);
        tagged.as_mut_ptr::<U>().read()
    }

    /// Consumes the `TaggedBox`, returning a wrapped pointer.
    ///
    /// The pointer will be properly aligned and non-null, and the caller is responsible for managing the memory
    /// allocated by `TaggedBox`.
    #[inline]
    pub fn into_raw<U>(self) -> *mut U {
        let mut this = ManuallyDrop::new(self);
        this.boxed.as_mut_ptr()
    }

    /// Constructs a `TaggedBox` from a raw pointer and a discriminant.
    ///
    /// Trusts that the provided pointer is valid and non-null, as well as that the memory
    /// allocated is the same as allocated by `TaggedBox`
    ///
    /// # Safety
    /// This function is unsafe because improper use may lead to memory problems.
    /// For example, a double-free may occur if the function is called twice on the same raw pointer.
    #[inline]
    pub unsafe fn from_raw<U>(raw: *mut U, discriminant: Discriminant) -> Self {
        Self {
            boxed: TaggedPointer::new(raw as usize, discriminant),
            _type: PhantomData,
        }
    }

    /// Fetches the discriminant of a `TaggedBox`
    #[inline]
    pub fn discriminant(&self) -> Discriminant {
        self.boxed.discriminant()
    }

    /// Retrieves a raw pointer to the data owned by `TaggedBox`, see [`TaggedPointer::as_ptr`]
    ///
    /// [`TaggedPointer::as_ptr`]: crate::TaggedPointer#as_ptr
    #[inline]
    pub fn as_ptr<U>(&self) -> *const U {
        self.boxed.as_ptr() as *const U
    }

    /// Retrieves a raw pointer to the data owned by `TaggedBox`, see [`TaggedPointer::as_mut_ptr`]
    ///
    /// [`TaggedPointer::as_mut_ptr`]: crate::TaggedPointer#as_mut_ptr
    #[inline]
    pub fn as_mut_ptr<U>(&mut self) -> *mut U {
        self.boxed.as_mut_ptr() as *mut U
    }

    /// Retrieves a usize pointing to the data owned by `TaggedBox`, see [`TaggedPointer::as_usize`]
    ///
    /// [`TaggedPointer::as_usize`]: crate::TaggedPointer#as_usize
    #[inline]
    pub(crate) fn as_usize(&self) -> usize {
        self.boxed.as_usize()
    }
}

impl_fmt!(impl[T: TaggableInner] TaggedBox<T> => LowerHex, UpperHex, Binary, Octal);

impl<T> fmt::Debug for TaggedBox<T>
where
    T: TaggableInner + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", T::from_tagged_box(self.clone()))
    }
}

impl<T> fmt::Display for TaggedBox<T>
where
    T: TaggableInner + fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", T::from_tagged_box(self.clone()))
    }
}

impl<T> Clone for TaggedBox<T>
where
    T: TaggableInner + Clone,
{
    fn clone(&self) -> Self {
        T::from_tagged_box(TaggedBox {
            boxed: self.boxed,
            _type: PhantomData,
        })
        .clone()
        .into_tagged_box()
    }
}

impl<T> Copy for TaggedBox<T> where T: TaggableInner + Copy {}

impl<T> PartialEq for TaggedBox<T>
where
    T: TaggableInner + PartialEq<T> + Clone,
{
    fn eq(&self, other: &TaggedBox<T>) -> bool {
        T::from_tagged_box(self.clone()) == T::from_tagged_box(other.clone())
    }
}

impl<T> Eq for TaggedBox<T> where T: TaggableInner + Eq + Clone {}

impl<T> PartialOrd for TaggedBox<T>
where
    T: TaggableInner + PartialOrd<T> + Clone,
{
    fn partial_cmp(&self, other: &TaggedBox<T>) -> Option<cmp::Ordering> {
        T::from_tagged_box(self.clone()).partial_cmp(&T::from_tagged_box(other.clone()))
    }
}

impl<T> Ord for TaggedBox<T>
where
    T: TaggableInner + Ord + Clone,
{
    fn cmp(&self, other: &TaggedBox<T>) -> cmp::Ordering {
        let mut cmp = cmp::Ordering::Equal;
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                T::ref_from_tagged_box(other, |other| {
                    cmp = this.cmp(other);
                })
            });
        }

        cmp
    }
}

/// A helper trait for containers that hold a [`TaggedBox`] associated with a specific enum
///
/// [`TaggedBox`]: crate::TaggedBox
pub trait TaggableContainer {
    type Inner;

    /// Takes an instance of a `TaggableContainer` and converts it into the enum variant stored within
    fn into_inner(self) -> Self::Inner;
}

/// Represents a value able to be stored in a [`TaggedBox`]
///
/// [`TaggedBox`]: crate::TaggedBox
pub trait TaggableInner: Sized {
    /// Creates a [`TaggedBox`] from `self`, storing it on the heap and keeping it's discriminant
    /// in the pointer.  
    /// See [`TaggedPointer`] for more
    ///
    /// [`TaggedBox`]: crate::TaggedBox
    /// [`TaggedPointer`]: crate::TaggedPointer
    fn into_tagged_box(self) -> TaggedBox<Self>;

    /// Creates an instance of `Self` from a [`TaggedBox`], taking ownership of the value
    ///
    /// [`TaggedBox`]: crate::TaggedBox
    fn from_tagged_box(tagged: TaggedBox<Self>) -> Self;

    /// Run a closure on a reference to the value contained in `tagged`
    ///
    /// # Safety
    ///
    /// The closure supplied to `callback` must not move the referenced value
    unsafe fn ref_from_tagged_box<F>(tagged: &TaggedBox<Self>, callback: F)
    where
        F: FnOnce(&Self);
}

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
        $( #[$meta] )*
        #[repr(transparent)]
        $struct_vis struct $struct {
            value: $crate::TaggedBox<$enum>,
        }

        impl $crate::TaggableContainer for $struct {
            type Inner = $enum;

            fn into_inner(self) -> $enum {
                let mut discriminant = 0;

                #[allow(unused_assignments)]
                unsafe {
                    // TODO: Find a way to use a match statement
                    $(
                        if discriminant == self.value.discriminant() {
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
            #[allow(unused_assignments, irrefutable_let_patterns)]
            fn into_tagged_box(self) -> $crate::TaggedBox<Self> {
                let mut discriminant = 0;

                // TODO: Find a way to use a match statement
                $(
                    if let Self::$variant(value) = self {
                        return $crate::TaggedBox::new(value, discriminant);
                    } else {
                        discriminant += 1;
                    }
                )+

                unreachable!("All variants of the enum should have been matched on");
            }

            fn from_tagged_box(tagged: $crate::TaggedBox<$enum>) -> Self {
                let mut discriminant = 0;

                #[allow(unused_assignments)]
                unsafe {
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};

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
        let discriminant = Discriminant::max_value();

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
