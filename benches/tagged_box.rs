use criterion::{black_box, criterion_group, criterion_main, Criterion};
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

fn default_impl(c: &mut Criterion) {
    c.bench_function("Unit into_tagged_box", |b| {
        b.iter(|| black_box(Item::Unit).into_tagged_box());
    })
    .bench_function("Single element Tuple into_tagged_box", |b| {
        b.iter(|| black_box(Item::SingleTuple(usize::max_value())).into_tagged_box());
    })
    .bench_function("Multiple element Tuple into_tagged_box", |b| {
        b.iter(|| black_box(Item::ManyTuple(100, 200, 50.100005, 300)).into_tagged_box());
    })
    .bench_function("Orphan Struct into_tagged_box", |b| {
        b.iter(|| {
            black_box(Item::Orphan {
                int: 10,
                boolean: false,
            })
            .into_tagged_box()
        });
    });
}

criterion_group!(benches, default_impl);
criterion_main!(benches);
