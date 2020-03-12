use crate::discriminant::{
    Discriminant, DISCRIMINANT_MASK, MAX_DISCRIMINANT, MAX_POINTER_VALUE, POINTER_WIDTH,
};
use core::fmt;

/// A pointer that holds a data pointer plus additional data stored as a [`Discriminant`]  
/// Note: The discriminant must be <= [`MAX_DISCRIMINANT`], which is feature-dependent  
///
/// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TaggedPointer {
    /// The tagged pointer, the upper bits are used to store arbitrary data
    tagged_ptr: u64,
}

impl TaggedPointer {
    /// Create a new tagged pointer from a pointer and a discriminant
    ///
    /// # Panics
    ///
    /// Panics if `discriminant` is greater than [`MAX_DISCRIMINANT`] or if
    /// `ptr` is greater than [`MAX_POINTER_VALUE`]
    ///
    /// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
    /// [`MAX_POINTER_VALUE`]: crate::discriminant::MAX_POINTER_VALUE
    #[inline]
    #[allow(clippy::absurd_extreme_comparisons)]
    pub fn new(ptr: u64, discriminant: Discriminant) -> Self {
        assert!(
            discriminant <= MAX_DISCRIMINANT,
            "Attempted to store a discriminant of {} while the max value is {}",
            discriminant,
            MAX_DISCRIMINANT,
        );
        assert!(
            ptr <= MAX_POINTER_VALUE,
            "If you are receiving this error, then your hardware uses more than {} bits of a pointer to store addresses. \
            It is recommended that you use a different feature for the `tagged-box` crate using `features = [\"{}bits\"]` or above.
            ",
            POINTER_WIDTH,
            POINTER_WIDTH + 1,
        );

        // Safety: The check that discriminant <= MAX_DISCRIMINANT has already been preformed
        let tagged_ptr = unsafe { Self::store_discriminant_unchecked(ptr, discriminant) };

        Self { tagged_ptr }
    }

    /// Create a new tagged pointer from a pointer and a discriminant without
    /// checking invariance
    ///
    /// # Safety
    ///
    /// `discriminant` must be <= [`MAX_DISCRIMINANT`] and `pointer` must be <=
    /// [`MAX_POINTER_VALUE`]
    ///
    /// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
    /// [`MAX_POINTER_VALUE`]: crate::discriminant::MAX_POINTER_VALUE
    #[inline]
    pub unsafe fn new_unchecked(ptr: u64, discriminant: Discriminant) -> Self {
        let tagged_ptr = Self::store_discriminant_unchecked(ptr, discriminant);

        Self { tagged_ptr }
    }

    /// Fetches the [`Discriminant`] of a tagged pointer
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub const fn discriminant(self) -> Discriminant {
        Self::fetch_discriminant(self.tagged_ptr)
    }

    /// Gains a reference to the inner value of the pointer
    ///
    /// # Safety
    ///
    /// The pointer given to [`TaggedPointer::new`] must be properly aligned and non-null
    ///
    /// [`TaggedPointer::new`]: crate::TaggedPointer#new
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub unsafe fn as_ref<T>(&self) -> &T {
        &*(Self::strip_discriminant(self.tagged_ptr) as *const T)
    }

    /// Gains a mutable reference to the inner value of the pointer
    ///
    /// # Safety
    ///
    /// The pointer given to [`TaggedPointer::new`] must be properly aligned and non-null
    ///
    /// [`TaggedPointer::new`]: crate::TaggedPointer#new
    #[inline]
    pub unsafe fn as_mut_ref<T>(&mut self) -> &mut T {
        &mut *(Self::strip_discriminant(self.tagged_ptr) as *mut T)
    }

    /// Returns the pointer as a u64, removing the discriminant
    #[inline]
    pub const fn as_u64(self) -> u64 {
        Self::strip_discriminant(self.tagged_ptr)
    }

    /// Returns the raw tagged pointer, without removing the discriminant
    ///
    /// # Warning
    ///
    /// Attempting to dereference this u64 will not point to valid memory!
    ///
    #[inline]
    pub const fn as_raw_u64(self) -> u64 {
        self.tagged_ptr
    }

    /// Converts a tagged pointer into a raw pointer, removing the discriminant
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        Self::strip_discriminant(self.tagged_ptr) as *const T
    }

    /// Converts a tagged pointer into a raw pointer, removing the discriminant
    #[inline]
    pub fn as_mut_ptr<T>(self) -> *mut T {
        Self::strip_discriminant(self.tagged_ptr) as *mut T
    }

    /// Store a [`Discriminant`] into a tagged pointer
    ///
    /// # Panics
    ///
    /// Panics if `discriminant` is greater than [`MAX_DISCRIMINANT`] or if
    /// `ptr` is greater than [`MAX_POINTER_VALUE`]
    ///
    /// [`Discriminant`]: crate::Discriminant
    /// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
    /// [`MAX_POINTER_VALUE`]: crate::discriminant::MAX_POINTER_VALUE
    #[inline]
    #[allow(clippy::absurd_extreme_comparisons)]
    pub fn store_discriminant(pointer: u64, discriminant: Discriminant) -> u64 {
        assert!(
            discriminant <= MAX_DISCRIMINANT,
            "Attempted to store a discriminant of {} while the max value is {}",
            discriminant,
            MAX_DISCRIMINANT,
        );
        assert!(
            pointer <= MAX_POINTER_VALUE,
            "If you are receiving this error, then your hardware uses more than {} bits of a pointer to store addresses. \
            It is recommended that you use a different feature for the `tagged-box` crate using `features = [\"{}bits\"]` or above.
            ",
            POINTER_WIDTH,
            POINTER_WIDTH + 1,
        );

        pointer | ((discriminant as u64) << POINTER_WIDTH)
    }

    /// Store a [`Discriminant`] into a tagged pointer without any checks
    ///
    /// # Safety
    ///
    /// `discriminant` must be <= [`MAX_DISCRIMINANT`] and `pointer` must be <=
    /// [`MAX_POINTER_VALUE`]
    ///
    /// [`Discriminant`]: crate::Discriminant
    /// [`MAX_DISCRIMINANT`]: crate::discriminant::MAX_DISCRIMINANT
    /// [`MAX_POINTER_VALUE`]: crate::discriminant::MAX_POINTER_VALUE
    #[inline]
    pub unsafe fn store_discriminant_unchecked(pointer: u64, discriminant: Discriminant) -> u64 {
        pointer | ((discriminant as u64) << POINTER_WIDTH)
    }

    /// Fetch a [`Discriminant`] from a tagged pointer    
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn fetch_discriminant(pointer: u64) -> Discriminant {
        (pointer >> POINTER_WIDTH) as Discriminant
    }

    /// Strip the [`Discriminant`] from a tagged pointer, returning only the valid pointer as a usize
    ///
    /// [`Discriminant`]: crate::Discriminant
    #[inline]
    pub const fn strip_discriminant(pointer: u64) -> u64 {
        pointer & DISCRIMINANT_MASK
    }
}

impl fmt::Debug for TaggedPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaggedPointer")
            .field("raw", &(self.as_raw_u64() as *const ()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discriminant;
    use alloc::string::String;
    use core::slice;

    #[test]
    fn utility_functions() {
        let ptr = 0xF00D_BEEF;
        let discrim = discriminant::MAX_DISCRIMINANT / 2;
        let stored = TaggedPointer::new(ptr, discrim).as_raw_u64();

        assert_eq!(TaggedPointer::strip_discriminant(stored), ptr);
        assert_eq!(TaggedPointer::fetch_discriminant(stored), discrim);
        assert_eq!(TaggedPointer::store_discriminant(ptr, discrim), stored);
    }

    #[test]
    fn tagged_pointer() {
        let integer = 100i32;
        let int_ptr = &integer as *const _ as u64;
        let discriminant = 10;

        let ptr = TaggedPointer::new(int_ptr, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_u64(), int_ptr);

        unsafe {
            assert_eq!(ptr.as_ref::<i32>(), &integer);
        }
    }

    #[test]
    fn max_pointer() {
        let ptr = discriminant::MAX_POINTER_VALUE;
        let discriminant = discriminant::MAX_DISCRIMINANT;

        let tagged = TaggedPointer::new(ptr, discriminant);

        assert_eq!(tagged.discriminant(), discriminant);
        assert_eq!(tagged.as_u64(), ptr);
    }

    #[test]
    fn min_pointer() {
        let ptr: u64 = 0;
        let discriminant = discriminant::MAX_DISCRIMINANT;

        let tagged = TaggedPointer::new(ptr, discriminant);

        assert_eq!(tagged.discriminant(), discriminant);
        assert_eq!(tagged.as_u64(), ptr);
    }

    #[test]
    fn max_discriminant() {
        let integer = 100usize;
        let int_ptr = &integer as *const _ as u64;
        let discriminant = discriminant::MAX_DISCRIMINANT;

        let ptr = TaggedPointer::new(int_ptr, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_u64(), int_ptr);

        unsafe {
            assert_eq!(ptr.as_ref::<usize>(), &integer);
        }
    }

    #[test]
    fn min_discriminant() {
        let integer = 100usize;
        let int_ptr = &integer as *const _ as u64;
        let discriminant = 0;

        let ptr = TaggedPointer::new(int_ptr, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_u64(), int_ptr);

        unsafe {
            assert_eq!(ptr.as_ref::<usize>(), &integer);
        }
    }

    #[test]
    fn string_pointer() {
        let string = String::from("Hello world!");
        let str_ptr = string.as_ptr() as u64;
        let discriminant = discriminant::MAX_DISCRIMINANT;

        let ptr = TaggedPointer::new(str_ptr, discriminant);

        assert_eq!(ptr.discriminant(), discriminant);
        assert_eq!(ptr.as_u64(), str_ptr);

        unsafe {
            let temp_str = slice::from_raw_parts(ptr.as_ptr::<u8>(), string.len());
            assert_eq!(core::str::from_utf8(temp_str).unwrap(), &string);
        }
    }

    #[test]
    #[should_panic]
    #[cfg_attr(miri, ignore)]
    fn oversized_discriminant() {
        let pointer = 0xF00D_BEEF;
        let discriminant = if let Some(discrim) = discriminant::MAX_DISCRIMINANT.checked_add(1) {
            discrim
        } else {
            panic!("Adding one to discriminant would overflow type, aborting test");
        };

        TaggedPointer::new(pointer, discriminant);
    }
}
