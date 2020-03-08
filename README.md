# Tagged-box

[![Crates.io](https://img.shields.io/crates/v/tagged-box)](https://crates.io/tagged-box)
[![Docs.rs](https://docs.rs/tagged-box/badge.svg)](https://docs.rs/tagged-box)
[![GitHub](https://img.shields.io/github/languages/top/Kixiron/tagged-box)](https://github.com/Kixiron/tagged-box)
[![LOC](https://tokei.rs/b1/github/Kixiron/tagged-box)](https://github.com/Kixiron/tagged-box)

A `no_std`, zero-dependency crate for the creation and management of NaN-boxed types with
[`Box`]-like semantics and tagged pointers, tagged pointers and a macro interface to safely
create NaN-boxed enums.

## Quickstart

First, add the crate to your `Cargo.lock` (Note: for variable reserved widths, see the [features] section)

```toml
tagged_box = "0.1.0"
```

Next, for using the macro, add the following to the top of your file

```rust
use tagged_box::{tagged_box, TaggableContainer, TaggableInner};
```

Then you can use the macro as follows

```rust
tagged_box! {
    #[derive(Debug, Clone, PartialEq)]
    struct Container, enum Item {
        Integer(i32),
        Boolean(bool),
        String(String),
    }
}

let container = Container::from(String::from("Hello from tagged-box!"));

assert_eq!(
    container.into_inner(),
    Item::String(String::from("Hello from tagged-box!"))
);
```

For working with NaN-boxes, simply add

```rust
use tagged_box::TaggedBox;
```

And for tagged pointers use

```rust
use tagged_box::TaggedPointer;
```

## What this crate does

This crate implements [NaN-Boxing] and [Tagged Pointers], which are a way to store extra data in the [unused bits of pointers].
While the two differ in implementation, they are semantically the same. In this crate, the `TaggedBox` type allows you to store
anywhere from 7 to 16 bits of arbitrary data in your pointer, depending on the [features] enabled. For explanation's sake,
I'll be using the `48bits` feature to explain, as it's the default and leads to the cleanest examples.  
The pointers this applies to are 64 bits long, looking something like this

```text
0000 0000 0000 0000
```

However, not all of these bits are used for the actual addressing of memory, so most pointers look like this

```text
0000 FFFF FFFF FFFF
^^^^
Free Data!
```

Those first 16 bits are free data, just begging to be used, and that's what `TaggedPointer` does. `TaggedPointer` simply
manages the pointer and the data (referred to as a 'discriminant' throughout this crate), making sure you get a pointer when you
need it and data when you need it, and not mixing those two up.  

`TaggedBox` goes one layer higher, storing an [enum discriminant] (Indicated by the type parameter) and directly storing the enum variant's inner value to the heap. In short, `TaggedBox` is like a `Box` and an enum rolled into one.  

Ramping the abstraction up one more notch, we have the `tagged_box!` macro, which creates a container-type struct and an associated `TaggedBox`-backed enum that can be seamlessly transferred between.

## Cargo Features

This crate has a few features that change the number of free and reserved bits:

- `48bits` (On by default): 48 bits of reserved pointer, 16 bits for data
- `49bits`: 49 bits of reserved pointer, 15 bits for data
- `50bits`: 50 bits of reserved pointer, 14 bits for data
- `51bits`: 51 bits of reserved pointer, 13 bits for data
- `52bits`: 52 bits of reserved pointer, 12 bits for data
- `53bits`: 53 bits of reserved pointer, 11 bits for data
- `54bits`: 54 bits of reserved pointer, 10 bits for data
- `55bits`: 55 bits of reserved pointer, 9 bits for data
- `56bits`: 56 bits of reserved pointer, 8 bits for data
- `57bits`: 57 bits of reserved pointer, 7 bits for data

However, only one of these may be active at a time, otherwise a `compile_error` will be emitted.

To select a feature, put the following in your `Cargo.toml`

```toml
[dependencies.tagged_box]
version = "0.1.0"
default-features = false
features = ["50bits"] # Select your feature here
```

[`Box`]: (https://doc.rust-lang.org/std/boxed/struct.Box.html)
[features]: #cargo-features
[NaN-Boxing]: https://wingolog.org/archives/2011/05/18/value-representation-in-javascript-implementations
[Tagged Pointers]: https://en.wikipedia.org/wiki/Tagged_pointer
[unused bits of pointers]: https://en.wikipedia.org/wiki/X86-64#Virtual_address_space_details
[enum discriminant]: https://doc.rust-lang.org/reference/items/enumerations.html
