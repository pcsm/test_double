test_double [![Crates.io](https://img.shields.io/crates/v/test_double.svg)](https://crates.io/crates/test_double) [![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
==================================================================

A procedural macro that can swap in mock objects, dummy objects, or other test doubles only when testing.

To use, add the following to your `Cargo.toml`:

```toml
[dependencies]
test_double = "0.2.1"
```

Note that this crate has not yet reached version 1.0, so the API may change between releases.

## Substituting One Type: `#[test_double]`

The substituted name defaults to the original name, postfixed with "Mock":

```rust
#[test_double]
use image::ImageManager;

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::ImageManagerMock as ImageManager;
```

If you'd like to provide an alternate substituted name, you can do so:

```rust
#[test_double(IMDummy)]
use image::ImageManager;

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::IMDummy as ImageManager;
```

### Limitations

`#[test_double]` can't be used with:

- Glob imports, like `use blah::*;`
- Grouped imports, like `use blah::{foo, bar};`, when providing an alternate substituted name

## Substituting Multiple Types: `test_doubles!`

If you'd like to substitute multiple types at once, you can use the function-like macro. Note that this does not support providing an alternate substituted name.

```rust
test_doubles! {
    use image::ImageManager;
    use texture::TextureManager;
}

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::ImageManagerMock as ImageManager;
#[cfg(not(test))]
use texture::TextureManager;
#[cfg(test)]
use texture::TextureManagerMock as TextureManager;
```

### Limitations

`test_doubles!` can't be used with:

- Glob imports, like `use blah::*;`

## Features


### prefix-mock

For crates such as [mockall](https://github.com/asomers/mockall) that prefix the "Mock" name
addition, there's the feature `prefix-mock`.

Just enable it in your `Cargo.toml` like so:

```toml
test_double = { version = "0.2.1", features = ["prefix-mock"] }
```