use crate::{discriminant::Discriminant, taggable::TaggableInner, tagged_pointer::TaggedPointer};
use core::{
    alloc::Layout,
    cmp, fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ptr,
};

/// A tagged box, associated with a variable type (enum, integer, etc.) able to be extracted from
/// the underlying [`TaggedPointer`]
///
/// [`TaggedPointer`]: crate::tagged_pointer::TaggedPointer
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
