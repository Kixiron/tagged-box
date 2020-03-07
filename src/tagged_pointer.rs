use crate::discriminant::{Discriminant, DISCRIMINANT_MASK, MAX_DISCRIMINANT, POINTER_WIDTH};
use core::fmt;

/// A pointer that holds a data pointer plus additional data stored as a [`Discriminant`]
///
/// Depending on which feature you have active, one of the following is true:
///
/// #### With the `48bits` feature
/// The upper 16 bits of the pointer will be used to store a `u16` of data
///
/// #### With the `57bits` feature
/// The upper 7 bits of the pointer will be used to store `2 ^ 7` of data
///
/// [`Discriminant`]: crate::discriminant::Discriminant
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

        // Safety: The check that discriminant <= MAX_DISCRIMINANT has already been preformed
        let tagged_ptr = unsafe { Self::store_discriminant_unchecked(ptr, discriminant) };

        Self { tagged_ptr }
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

    /// Store a [`Discriminant`] into a tagged pointer without any checks
    ///
    /// # Safety
    /// `discriminant` must be less than or equal to MAX_DISCRIMINANT
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub unsafe fn store_discriminant_unchecked(
        pointer: usize,
        discriminant: Discriminant,
    ) -> usize {
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

impl_fmt!(TaggedPointer => LowerHex, UpperHex, Binary, Octal);
