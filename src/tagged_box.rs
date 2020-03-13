use crate::{discriminant::Discriminant, taggable::TaggableInner, tagged_pointer::TaggedPointer};
use alloc::boxed::Box;
use core::{
    alloc::Layout,
    cmp, fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::NonNull,
};

/// A tagged box, associated with a variable type (enum, integer, etc.) able to be extracted from
/// the underlying [`TaggedPointer`]
///
/// [`TaggedPointer`]: crate::tagged_pointer::TaggedPointer
#[repr(transparent)]
pub struct TaggedBox<T> {
    boxed: TaggedPointer,
    _type: PhantomData<T>,
}

impl<T> TaggedBox<T> {
    /// Creates a new `TaggedBox` from a value and its discriminant
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// #
    /// enum Communication {
    ///     Message(String),
    ///     Letter(Vec<Paper>),
    /// }
    ///
    /// let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// ```
    ///
    /// ## **Make sure to retrieve the correct type from the `TaggedBox`, or you will encounter undefined behavior!**  
    ///
    /// This is alright, because we stored a `String` onto the heap and we are retrieving a `String` with `into_inner`
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// # enum Communication {
    /// #     Message(String),
    /// #     Letter(Vec<Paper>),
    /// # }
    /// # let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// #
    /// unsafe {
    ///     let message = TaggedBox::into_inner::<String>(tagged_message);
    ///
    ///     assert_eq!(&message, "Foobar is a timeless classic");
    /// }
    /// ```
    ///
    /// This is UB, because we stored a `String` onto the heap and are retreving a `Vec<Paper`
    ///
    /// ```should_panic
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// # enum Communication {
    /// #     Message(String),
    /// #     Letter(Vec<Paper>),
    /// # }
    /// # let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// #
    /// unsafe {
    ///     let letter = TaggedBox::into_inner::<Vec<Paper>>(tagged_message); // UB!
    /// }
    /// # panic!("Undefined Behavior!");
    /// ```
    ///
    #[inline]
    pub fn new<U>(val: U, discriminant: Discriminant) -> Self {
        if mem::size_of::<U>() == 0 {
            Self::dangling::<U>(discriminant)
        } else {
            let layout = Layout::new::<U>();

            // Safety: The allocation should be properly handled by alloc + layout,
            // and writing should be properly aligned, as the pointer came from the
            // global allocator, plus the allocated type is not a ZST
            let ptr = unsafe {
                let ptr = alloc::alloc::alloc(layout) as *mut U;
                assert!(ptr as u64 != 0);
                ptr.write(val);

                ptr
            };

            Self {
                boxed: TaggedPointer::new(ptr as u64, discriminant),
                _type: PhantomData,
            }
        }
    }

    /// Creates a new `TaggedBox` from a value and its discriminant, without checking invariance
    ///
    /// # Safety
    ///
    /// `discriminant` must be <= [`MAX_DISCRIMINANT`] and `pointer` must be <=
    /// [`MAX_POINTER_VALUE`].  
    /// See [`TaggedPointer::new_unchecked`] for more
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// #
    /// enum Communication {
    ///     Message(String),
    ///     Letter(Vec<Paper>),
    /// }
    ///
    /// let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// ```
    ///
    /// ## **Make sure to retrieve the correct type from the `TaggedBox`, or you will encounter undefined behavior!**  
    ///
    /// This is alright, because we stored a `String` onto the heap and we are retrieving a `String` with `into_inner`
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// # enum Communication {
    /// #     Message(String),
    /// #     Letter(Vec<Paper>),
    /// # }
    /// # let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// #
    /// unsafe {
    ///     let message = TaggedBox::into_inner::<String>(tagged_message);
    ///
    ///     assert_eq!(&message, "Foobar is a timeless classic");
    /// }
    /// ```
    ///
    /// This is UB, because we stored a `String` onto the heap and are retreving a `Vec<Paper`
    ///
    /// ```should_panic
    /// # extern crate alloc;
    /// # use alloc::{vec::Vec, string::String};
    /// # use tagged_box::TaggedBox;
    /// # struct Paper;
    /// # enum Communication {
    /// #     Message(String),
    /// #     Letter(Vec<Paper>),
    /// # }
    /// # let tagged_message: TaggedBox<Communication> = TaggedBox::new(String::from("Foobar is a timeless classic"), 0);
    /// #
    /// unsafe {
    ///     let letter = TaggedBox::into_inner::<Vec<Paper>>(tagged_message); // UB!
    /// }
    /// # panic!("Undefined Behavior!");
    /// ```
    ///
    /// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
    /// [`MAX_POINTER_VALUE`]: crate::discriminant::MAX_POINTER_VALUE
    /// [`TaggedPointer::new_unchecked`]: crate::tagged_pointer::TaggedPointer#new_unchecked
    #[inline]
    pub unsafe fn new_unchecked<U>(val: U, discriminant: Discriminant) -> Self {
        if mem::size_of::<U>() == 0 {
            Self::dangling::<U>(discriminant)
        } else {
            let layout = Layout::new::<U>();

            // Safety: The allocation should be properly handled by alloc + layout,
            // and writing should be properly aligned, as the pointer came from the
            // global allocator
            let ptr = {
                let ptr = alloc::alloc::alloc(layout) as *mut U;
                assert!(ptr as u64 != 0);
                ptr.write(val);

                ptr
            };

            Self {
                boxed: TaggedPointer::new(ptr as u64, discriminant),
                _type: PhantomData,
            }
        }
    }

    /// Creates a dangling tagged box, see [`NonNull::dangling`] for more information
    ///
    /// [`NonNull::dangling`]: https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.dangling
    #[inline]
    pub const fn dangling<U>(discriminant: Discriminant) -> Self {
        Self {
            boxed: TaggedPointer::dangling::<U>(discriminant),
            _type: PhantomData,
        }
    }

    /// Returns an immutable reference to the value stored on the heap
    ///
    /// # Safety
    ///
    /// The type provided as `U` must be the same type as allocated by `new`,
    /// and any actions done with the value must not move it
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// enum Bricks {
    ///     Red(usize),
    /// }
    ///
    /// let red_brick: TaggedBox<Bricks> = TaggedBox::new(100_usize, 0);
    ///
    /// unsafe {
    ///     // We allocated a usize, so we can retrieve one
    ///     assert_eq!(red_brick.as_ref::<usize>(), &100);
    /// }
    /// ```
    ///
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub unsafe fn as_ref<U>(&self) -> &U {
        self.boxed.as_ref()
    }

    /// Returns an immutable reference to the value stored on the heap
    ///
    /// # Safety
    ///
    /// The type provided as `U` must be the same type as allocated by `new`,
    /// and any actions done with the value must not move it
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// enum Bricks {
    ///     Red(usize),
    /// }
    ///
    /// let mut red_brick: TaggedBox<Bricks> = TaggedBox::new(100_usize, 0);
    ///
    /// unsafe {
    ///     assert_eq!(red_brick.as_ref::<usize>(), &100);
    ///
    ///     // We allocated a usize, so we can retrieve one and change it
    ///     *red_brick.as_mut_ref::<usize>() = 300;
    ///
    ///     assert_eq!(red_brick.as_ref::<usize>(), &300);
    /// }
    /// ```
    ///
    #[inline]
    pub unsafe fn as_mut_ref<U>(&mut self) -> &mut U {
        self.boxed.as_mut_ref()
    }

    /// Return the boxed value contained in the `TaggedPointer`
    ///
    /// # Safety
    ///
    /// The type provided as `U` must be the same type as allocated by `new`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// enum Bricks {
    ///     Red(usize),
    /// }
    ///
    /// let red_brick: TaggedBox<Bricks> = TaggedBox::new(100_usize, 0);
    ///
    /// unsafe {
    ///     // We allocated a usize, so we can retrieve one
    ///     let mut number_bricks: usize = TaggedBox::into_inner(red_brick);
    ///
    ///     assert_eq!(number_bricks, 100);
    /// }
    /// ```
    ///
    #[inline]
    #[must_use]
    pub unsafe fn into_inner<U>(tagged: Self) -> U {
        let mut tagged = ManuallyDrop::new(tagged);
        tagged.as_mut_ptr::<U>().read()
    }

    /// Consumes the `TaggedBox`, returning a raw pointer.
    ///
    /// The pointer will be properly aligned and non-null, and the caller is responsible for managing the memory
    /// allocated by `TaggedBox`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// # enum InnerValue {}
    /// #
    /// let tagged_box: TaggedBox<InnerValue> = TaggedBox::new([10u8; 10], 8);
    ///
    /// // Get the raw pointer to the heap-allocated value
    /// let raw: *mut [u8; 10] = TaggedBox::into_raw(tagged_box);
    ///
    /// unsafe {
    ///     assert_eq!(*raw, [10; 10]);
    /// }
    /// ```
    ///
    #[inline]
    pub fn into_raw<U>(tagged: Self) -> *mut U {
        ManuallyDrop::new(tagged).boxed.as_mut_ptr()
    }

    /// Creates a [`Box`] from the provided `TaggedBox`  
    /// Trusts that the type provided as `U` is valid for the allocated layout.
    ///
    /// # Safety
    ///
    /// The type provided as `U` must be the same type that the instance of `TaggedBox`
    /// was initialized with.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use alloc::boxed::Box;
    /// # use tagged_box::TaggedBox;
    /// # enum InnerValue {}
    /// #
    /// let tagged_box: TaggedBox<InnerValue> = TaggedBox::new([10u8; 10], 8);
    ///
    /// unsafe {
    ///     // Get the Boxed value
    ///     let boxed: Box<[u8; 10]> = TaggedBox::into_box(tagged_box);
    /// }
    /// ```
    ///
    /// [`Box`]: https://doc.rust-lang.org/alloc/boxed/struct.Box.html
    #[inline]
    pub unsafe fn into_box<U>(tagged: Self) -> alloc::boxed::Box<U> {
        use alloc::boxed::Box;

        let raw = Self::into_raw(tagged);
        Box::from_raw(raw)
    }

    // TODO: from_box

    /// Constructs a `TaggedBox` from a raw pointer and a discriminant.
    ///
    /// Trusts that the provided pointer is valid and non-null, as well as that the memory
    /// allocated is the method same as allocated by `TaggedBox`. `TaggedBox` uses the standard
    /// allocator
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory problems.
    /// For example, a double-free may occur if the function is called twice on the same raw pointer.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate alloc;
    /// # use tagged_box::TaggedBox;
    /// # use alloc::{vec, vec::Vec};
    /// # enum InnerValue {}
    /// #
    /// let tagged_box: TaggedBox<InnerValue> = TaggedBox::new(vec![100_usize, 200, 300], 10);
    ///
    /// // Turn the tagged box into a raw pointer and its discriminant
    /// let discriminant = tagged_box.discriminant();
    /// let raw_box: *mut Vec<usize> = TaggedBox::into_raw(tagged_box);
    ///
    /// unsafe {
    ///     assert_eq!(*raw_box, vec![100, 200, 300]);
    ///
    ///     let tagged_box: TaggedBox<InnerValue> = TaggedBox::from_raw(raw_box, discriminant);
    ///     
    ///     assert_eq!(TaggedBox::into_inner::<Vec<usize>>(tagged_box), vec![100, 200, 300]);
    /// }
    /// ```
    ///
    #[inline]
    pub unsafe fn from_raw<U>(raw: *mut U, discriminant: Discriminant) -> Self {
        Self {
            boxed: TaggedPointer::new(raw as u64, discriminant),
            _type: PhantomData,
        }
    }

    /// Fetches the discriminant of a `TaggedBox`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// # enum InnerValue {}
    /// #
    /// let tagged_box: TaggedBox<InnerValue> = TaggedBox::new(0x00, 11);
    ///
    /// assert_eq!(tagged_box.discriminant(), 11);
    /// ```
    ///
    #[inline]
    pub const fn discriminant(&self) -> Discriminant {
        self.boxed.discriminant()
    }

    /// Retrieves a raw pointer to the data owned by `TaggedBox`, see [`TaggedPointer::as_ptr`]  
    /// The caller must ensure that the returned pointer is never written to. If you need to
    /// mutate the contents of the tagged pointer, use [`as_mut_ptr`].
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// enum Bricks {
    ///     Red(usize),
    /// }
    ///
    /// let red_brick: TaggedBox<Bricks> = TaggedBox::new(100_usize, 0);
    ///
    /// unsafe {
    ///     assert_eq!(*red_brick.as_ptr::<usize>(), 100);
    /// }
    /// ```
    ///
    /// [`TaggedPointer::as_ptr`]: crate::TaggedPointer#as_ptr
    /// [`as_mut_ptr`]: crate::TaggedBox::as_ptr
    #[inline]
    pub const fn as_ptr<U>(&self) -> *const U {
        self.boxed.as_ptr() as *const U
    }

    /// Retrieves a raw pointer to the data owned by `TaggedBox`, see [`TaggedPointer::as_mut_ptr`]  
    /// It is your responsibility to make sure that the string slice only gets modified in a way
    /// that it remains valid
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tagged_box::TaggedBox;
    /// enum Bricks {
    ///     Red(usize),
    /// }
    ///
    /// let mut red_brick: TaggedBox<Bricks> = TaggedBox::new(100_usize, 0);
    ///
    /// unsafe {
    ///     *red_brick.as_mut_ptr::<usize>() = 100_000;
    ///
    ///     assert_eq!(TaggedBox::into_inner::<usize>(red_brick), 100_000);
    /// }
    /// ```
    ///
    /// [`TaggedPointer::as_mut_ptr`]: crate::TaggedPointer#as_mut_ptr
    #[inline]
    pub fn as_mut_ptr<U>(&mut self) -> *mut U {
        self.boxed.as_mut_ptr() as *mut U
    }

    /// Retrieves a u64 pointing to the data owned by `TaggedBox`, see [`TaggedPointer::as_usize`]
    ///
    /// [`TaggedPointer::as_u64`]: crate::TaggedPointer#as_u64
    #[inline]
    pub(crate) const fn as_u64(&self) -> u64 {
        self.boxed.as_u64()
    }
}

impl<T> fmt::Debug for TaggedBox<T>
where
    T: TaggableInner + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Ok(());
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                result = write!(f, "{:?}", this);
            });
        }

        result
    }
}

impl<T> fmt::Display for TaggedBox<T>
where
    T: TaggableInner + fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = Ok(());
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                result = write!(f, "{}", this);
            });
        }

        result
    }
}

impl<T> Clone for TaggedBox<T>
where
    T: TaggableInner + Clone,
{
    fn clone(&self) -> Self {
        let mut output = None;
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                output = Some(this.clone());
            });
        }

        output
            .expect("The inner value could not be fetched")
            .into_tagged_box()
    }
}

impl<T> Copy for TaggedBox<T> where T: TaggableInner + Copy {}

impl<T> PartialEq for TaggedBox<T>
where
    T: TaggableInner + PartialEq<T>,
{
    fn eq(&self, other: &TaggedBox<T>) -> bool {
        let mut eq = false;
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                T::ref_from_tagged_box(other, |other| {
                    eq = this == other;
                });
            });
        }

        eq
    }
}

impl<T> Eq for TaggedBox<T> where T: TaggableInner + Eq {}

impl<T> PartialOrd for TaggedBox<T>
where
    T: TaggableInner + PartialOrd<T>,
{
    fn partial_cmp(&self, other: &TaggedBox<T>) -> Option<cmp::Ordering> {
        let mut cmp = None;
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                T::ref_from_tagged_box(other, |other| {
                    cmp = this.partial_cmp(other);
                });
            });
        }

        cmp
    }
}

impl<T> Ord for TaggedBox<T>
where
    T: TaggableInner + Ord,
{
    fn cmp(&self, other: &TaggedBox<T>) -> cmp::Ordering {
        let mut cmp = cmp::Ordering::Equal;
        unsafe {
            T::ref_from_tagged_box(self, |this| {
                T::ref_from_tagged_box(other, |other| {
                    cmp = this.cmp(other);
                });
            });
        }

        cmp
    }
}

impl_fmt!(impl[T: TaggableInner] TaggedBox<T> => LowerHex, UpperHex, Binary, Octal);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tagged_box, TaggableContainer, TaggableInner};
    use alloc::{string::String, vec, vec::Vec};
    use core::mem::ManuallyDrop;

    #[test]
    fn new() {
        enum Test {}
        #[derive(Debug, PartialEq)]
        struct Custom {
            a: u32,
            b: bool,
            c: [usize; 10],
        }

        let integer: TaggedBox<Test> =
            TaggedBox::new(0xF00D_BEEF_usize, crate::discriminant::MAX_DISCRIMINANT / 2);
        let custom: TaggedBox<Test> = TaggedBox::new(
            Custom {
                a: 10000,
                b: false,
                c: [(usize::max_value() << 32) >> 32; 10],
            },
            crate::discriminant::MAX_DISCRIMINANT / 4,
        );
        let string: TaggedBox<Test> = TaggedBox::new(String::from("Hello world!"), 0);
        let vec: TaggedBox<Test> = TaggedBox::new(vec![1i32, 2, 3, 4, 5, 6], 10);

        unsafe {
            assert_eq!(0xF00D_BEEF_usize, TaggedBox::into_inner(integer));
            assert_eq!(
                Custom {
                    a: 10000,
                    b: false,
                    c: [(usize::max_value() << 32) >> 32; 10],
                },
                TaggedBox::into_inner(custom)
            );
            assert_eq!(
                String::from("Hello world!"),
                TaggedBox::into_inner::<String>(string)
            );
            assert_eq!(
                vec![1, 2, 3, 4, 5, 6],
                TaggedBox::into_inner::<Vec<i32>>(vec),
            );
        }
    }

    #[test]
    fn new_zst() {
        enum Test {}
        #[derive(Debug, PartialEq)]
        struct Zst;

        let unit: TaggedBox<Test> = TaggedBox::new((), 0);
        let zst: TaggedBox<Test> = TaggedBox::new(Zst, crate::discriminant::MAX_DISCRIMINANT);

        unsafe {
            assert_eq!((), TaggedBox::into_inner(unit));
            assert_eq!(Zst, TaggedBox::into_inner(zst));
        }
    }

    #[test]
    fn clone() {
        tagged_box! {
            #[derive(Clone, Debug, PartialEq)]
            struct Container, enum Enum {
                Usize(usize),
                Str(String),
            }
        }

        let original_usize = Container::from(100usize);

        let mut cloned_usize = original_usize.clone().into_inner().into_tagged_box();
        unsafe {
            *cloned_usize.as_mut_ref::<usize>() *= 100;
        }
        let cloned_usize = Enum::from_tagged_box(cloned_usize);
        let original_usize = original_usize.into_inner();

        assert_ne!(&original_usize, &cloned_usize);
        assert_eq!(&Enum::Usize(100 * 100), &cloned_usize);
        assert_eq!(Enum::Usize(100), original_usize);

        let original_string = Container::from(String::from("Hello"));

        let mut cloned_string = original_string.clone().into_inner().into_tagged_box();
        unsafe {
            cloned_string.as_mut_ref::<String>().push_str(", World!");
        }
        let cloned_string = Enum::from_tagged_box(cloned_string);
        let original_string = original_string.into_inner();

        assert_ne!(&original_string, &cloned_string);
        assert_eq!(&Enum::Str(String::from("Hello, World!")), &cloned_string);
        assert_eq!(Enum::Str(String::from("Hello")), original_string);
        assert_ne!(cloned_string, cloned_usize);
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
                Unit,
                Bool(bool),
                Array([u8; 8]),
                Vector(Vec<u8>),
                CustomStruct(CustomStruct),
                OrphanStructTrailingComma {
                    int: i32,
                    trailing_comma: f32,
                },
                OrphanStructNoTrailingComma {
                    int: i32,
                    no_trailing_comma: f32
                },
            }
        }

        assert_eq!(Outer::from(10.0f32).into_inner(), Inner::Float(10.0));
        assert_eq!(Outer::from(100i32).into_inner(), Inner::Int(100));
        assert_eq!(Outer::from(10u8).into_inner(), Inner::Byte(10));
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
