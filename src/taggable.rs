#![allow(clippy::module_name_repetitions)]

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
    /// The type of the enum that the container can be safely made into
    type Inner;

    /// Takes an instance of a `TaggableContainer` and converts it into the enum variant stored within
    fn into_inner(self) -> Self::Inner;
}

/// Represents a value able to be stored in a [`TaggedBox`].  
///
/// Using this directly is not recommended, as [`tagged_box!`] should be used instead.  
/// If you want to implement this yourself, see [`manually implementing a tagged enum`].
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
    ///
    unsafe fn ref_from_tagged_box<F>(tagged: &TaggedBox<Self>, callback: F)
    where
        F: FnOnce(&Self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tagged_box;

    tagged_box! {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        struct Container, enum Item {
            Int[(usize)],
            Bool[(bool)],
            Float[(f32)],
        }
    }

    #[test]
    fn container_into_inner() {
        let int = Container::from(usize::max_value());
        assert_eq!(int.into_inner(), Item::Int(usize::max_value()));

        let boolean = Container::from(true);
        assert_eq!(boolean.into_inner(), Item::Bool(true));

        let float = Container::from(core::f32::MAX);
        assert_eq!(float.into_inner(), Item::Float(core::f32::MAX));
    }

    #[test]
    fn inner_into_tagged_box() {
        assert_eq!(
            Item::Int(usize::max_value()),
            Container {
                value: Item::Int(usize::max_value()).into_tagged_box()
            }
            .into_inner()
        );

        assert_eq!(
            Item::Bool(false),
            Container {
                value: Item::Bool(false).into_tagged_box()
            }
            .into_inner()
        );

        assert_eq!(
            Item::Float(core::f32::MIN),
            Container {
                value: Item::Float(core::f32::MIN).into_tagged_box()
            }
            .into_inner()
        );
    }

    #[test]
    fn inner_from_tagged_box() {
        assert_eq!(
            Item::Int(usize::max_value()),
            Item::from_tagged_box(Item::Int(usize::max_value()).into_tagged_box())
        );

        assert_eq!(
            Item::Bool(true),
            Item::from_tagged_box(Item::Bool(true).into_tagged_box())
        );

        assert_eq!(
            Item::Float(core::f32::MAX),
            Item::from_tagged_box(Item::Float(core::f32::MAX).into_tagged_box())
        );
    }

    #[test]
    fn inner_ref_from_tagged_box() {
        unsafe {
            let int = Item::Int(usize::max_value());
            let boolean = Item::Bool(false);
            let float = Item::Float(core::f32::MIN);

            Item::ref_from_tagged_box(&Item::Int(usize::max_value()).into_tagged_box(), |item| {
                assert_eq!(item, &int);
                assert_ne!(item, &boolean);
                assert_ne!(item, &float);
            });

            Item::ref_from_tagged_box(&Item::Bool(false).into_tagged_box(), |item| {
                assert_eq!(item, &boolean);
                assert_ne!(item, &int);
                assert_ne!(item, &float);
            });

            Item::ref_from_tagged_box(&Item::Float(core::f32::MIN).into_tagged_box(), |item| {
                assert_eq!(item, &float);
                assert_ne!(item, &int);
                assert_ne!(item, &boolean);
            });
        }
    }

    #[test]
    fn wrapped_refs_from_tagged_box() {
        let big = Item::Int(10_000).into_tagged_box();
        let small = Item::Int(100).into_tagged_box();

        unsafe {
            Item::ref_from_tagged_box(&big, |big| {
                Item::ref_from_tagged_box(&small, |small| {
                    assert_ne!(big, small);
                    assert!(big > small);
                    assert!(small < big);
                });
            });
        }

        assert_eq!(Item::from_tagged_box(big), Item::Int(10_000));
        assert_eq!(Item::from_tagged_box(small), Item::Int(100));
    }
}
