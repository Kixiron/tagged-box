use tagged_box::{tagged_box, TaggableContainer, TaggableInner};

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
