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
            SingleTuple(usize),
            ManyTuple(usize, usize, f32, usize),
            Unit,
            Orphan {
                int: u32,
                boolean: bool,
            },
        }
    }

    #[test]
    fn container_into_inner() {
        let int = Container::from(usize::max_value());
        assert_eq!(int.into_inner(), Item::SingleTuple(usize::max_value()));

        let boolean = Container::from((100usize, 200usize, 50.100005, 300usize));
        assert_eq!(
            boolean.into_inner(),
            Item::ManyTuple(100, 200, 50.100005, 300)
        );
    }

    #[test]
    fn inner_into_tagged_box() {
        assert_eq!(
            Item::SingleTuple(usize::max_value()),
            Container {
                value: Item::SingleTuple(usize::max_value()).into_tagged_box()
            }
            .into_inner()
        );

        assert_eq!(
            Item::ManyTuple(100, 200, 50.100005, 300),
            Container {
                value: Item::ManyTuple(100, 200, 50.100005, 300).into_tagged_box()
            }
            .into_inner()
        );

        assert_eq!(
            Item::Unit,
            Container {
                value: Item::Unit.into_tagged_box()
            }
            .into_inner()
        );

        assert_eq!(
            Item::Orphan {
                int: 10,
                boolean: false,
            },
            Container {
                value: Item::Orphan {
                    int: 10,
                    boolean: false,
                }
                .into_tagged_box()
            }
            .into_inner()
        );
    }

    #[test]
    fn inner_from_tagged_box() {
        assert_eq!(
            Item::SingleTuple(usize::max_value()),
            Item::from_tagged_box(Item::SingleTuple(usize::max_value()).into_tagged_box())
        );

        assert_eq!(
            Item::ManyTuple(12_200, 23_300, 500.100005, 34_400),
            Item::from_tagged_box(
                Item::ManyTuple(12_200, 23_300, 500.100005, 34_400).into_tagged_box()
            )
        );

        assert_eq!(
            Item::Unit,
            Item::from_tagged_box(Item::Unit.into_tagged_box())
        );

        assert_eq!(
            Item::Orphan {
                int: 10_000,
                boolean: true,
            },
            Item::from_tagged_box(
                Item::Orphan {
                    int: 10_000,
                    boolean: true,
                }
                .into_tagged_box()
            )
        );
    }

    #[test]
    fn inner_ref_from_tagged_box() {
        unsafe {
            let one = Item::SingleTuple(usize::max_value());
            let many = Item::ManyTuple(1200, 233, 500.100005, 34);
            let unit = Item::Unit;
            let orphan = Item::Orphan {
                int: 0,
                boolean: false,
            };

            Item::ref_from_tagged_box(
                &Item::SingleTuple(usize::max_value()).into_tagged_box(),
                |item| {
                    assert_eq!(item, &one);
                    assert_ne!(item, &many);
                    assert_ne!(item, &unit);
                    assert_ne!(item, &orphan);
                },
            );

            Item::ref_from_tagged_box(
                &Item::ManyTuple(1200, 233, 500.100005, 34).into_tagged_box(),
                |item| {
                    assert_eq!(item, &many);
                    assert_ne!(item, &one);
                    assert_ne!(item, &unit);
                    assert_ne!(item, &orphan);
                },
            );

            Item::ref_from_tagged_box(&Item::Unit.into_tagged_box(), |item| {
                assert_eq!(item, &unit);
                assert_ne!(item, &one);
                assert_ne!(item, &many);
                assert_ne!(item, &orphan);
            });

            Item::ref_from_tagged_box(
                &Item::Orphan {
                    int: 0,
                    boolean: false,
                }
                .into_tagged_box(),
                |item| {
                    assert_eq!(item, &orphan);
                    assert_ne!(item, &unit);
                    assert_ne!(item, &one);
                    assert_ne!(item, &many);
                },
            );
        }
    }

    #[test]
    fn wrapped_refs_from_tagged_box() {
        let big = Item::SingleTuple(10_000).into_tagged_box();
        let small = Item::SingleTuple(100).into_tagged_box();

        unsafe {
            Item::ref_from_tagged_box(&big, |big| {
                Item::ref_from_tagged_box(&small, |small| {
                    assert_ne!(big, small);
                    assert!(big > small);
                    assert!(small < big);
                });
            });
        }

        assert_eq!(Item::from_tagged_box(big), Item::SingleTuple(10_000));
        assert_eq!(Item::from_tagged_box(small), Item::SingleTuple(100));
    }
}
