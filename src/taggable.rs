use crate::tagged_box::TaggedBox;

/// A helper trait for containers that hold a [`TaggedBox`] associated with a specific enum.  
///
/// Using this directly is not recommended, as [`tagged_box!`] should be used instead.
/// If you want to implement this yourself, see [`manually implementing a tagged enum`].
///
/// [`TaggedBox`]: crate::TaggedBox
/// [`tagged_box!`]: macro.tagged_box.html
/// [`manually implementing a tagged enum`]: crate::manually_impl_enum
pub trait TaggableContainer {
    type Inner;

    /// Takes an instance of a `TaggableContainer` and converts it into the enum variant stored within
    fn into_inner(self) -> Self::Inner;
}

/// Represents a value able to be stored in a [`TaggedBox`].  
///
/// Using this directly is not recommended, as [`tagged_box!`] should be used instead.
///  If you want to implement this yourself, see [`manually implementing a tagged enum`].
///
/// [`TaggedBox`]: crate::TaggedBox
/// [`tagged_box!`]: macro.tagged_box.html
/// [`manually implementing a tagged enum`]: crate::manually_impl_enum
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
