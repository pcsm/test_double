test_double [![Crates.io](https://img.shields.io/crates/v/test_double.svg)](https://crates.io/crates/test_double) [![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
==================================================================

A set of procedural macros that can swap in mock objects, dummy objects, or other test doubles only when testing.

This doesn't solve the problem of mocking in the first place, [which is worth looking into](https://asomers.github.io/mock_shootout/), but it does allow you to import mock objects into your tests easily.

To use, add the following to your `Cargo.toml`:

```toml
[dependencies]
test_double = "0.2.0"
```

Note that this crate has not yet reached version 1.0, so the API may change between releases.

## Substituting A Single Use Statement

`#[test_double]` can be used to substitute one type for another when testing:

```rust
#[test_double]
use db::Database;

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::DatabaseMock as Database;
```

The substituted type name defaults to the original name, with `Mock` appended to it.

### Alternate Name

You can also provide an alternate name to be substituted instead:

```rust
#[test_double(DummyDB)]
use db::Database;

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::DummyDB as Database;
```

Note that this is only supported when substituting a single type name.

### Prefixed

As a shortcut, if you would like to use the original type name, _prefixed_ with `Mock` instead of appended, you can use the `#[test_double_prefixed]` macro:

```rust
#[test_double_prefixed]
use db::Database;

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::MockDatabase as Database;
```

### Grouped Imports

It's worth noting that these macros also support grouped imports:


```rust
#[test_double]
use db::{
    Database,
    Connection
};

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::DatabaseMock as Database;
#[cfg(not(test))]
use db::Connection;
#[cfg(test)]
use db::ConnectionMock as Connection;
```

### Limitations 

For `#[test_doubles]` and `#[test_doubles_prefixed]`:

- Glob imports, such as `use blah::*;`, are not supported
- When providing an alternate substituted name, grouped imports, such as `use blah::{foo, bar};`, are not supported

## Substituting Multiple Use Statements

If you'd like to substitute multiple use statements at once, you can use the `test_doubles!` macro. Note that this macro does not support using an alternate substituted name, but it does support grouped imports, such as `use blah::{foo, bar};`.

```rust
test_doubles! {
    use db::{
        Database, 
        Connection
    };
    use image::ImageCache;
}

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::DatabaseMock as Database;
#[cfg(not(test))]
use db::Connection;
#[cfg(test)]
use db::ConnectionMock as Connection;
#[cfg(not(test))]
use image::ImageCache;
#[cfg(test)]
use image::ImageCacheMock as ImageCache;
```

### Prefixed

Similar to the single-use-statement macros, there is a `test_doubles_prefixed!` macro that can prefix instead of appending:

```rust
test_doubles_prefixed! {
    use db::Database;
    use image::ImageCache;
}

// Expands to:

#[cfg(not(test))]
use db::Database;
#[cfg(test)]
use db::MockDatabase as Database;
#[cfg(not(test))]
use image::ImageCache;
#[cfg(test)]
use image::MockImageCache as ImageCache;
```

### Limitations

For `test_doubles!` and `test_doubles_prefixed!`:

- Glob imports, such as `use blah::*;`, are not supported