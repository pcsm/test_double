test_double [![Crates.io](https://img.shields.io/crates/v/test_double.svg)](https://crates.io/crates/test_double) [![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
==================================================================

A procedural macro that can swap in mock objects, dummy objects, or other test doubles only when testing.

There are many limitations at present:

- Does not support grouped imports, like `use blah::{foo, bar};`
- Does not support nested paths, like `use blah::{*, something::{foo, bar}};`
- The substituted type can't be changed when using the function-like macro

To use, add the following to your `Cargo.toml`:

```toml
[dependencies]
test_double = "0.1.1"
```

Note that this crate has not yet reached version 1.0, so the API may change drastically between releases.

## Substituting One Type

The substituted name defaults to the original name, postfixed with "Mock":

```rust
#[test_double]
use ::image::ImageManager;

// Expands to:

#[cfg(not(test))]
use ::image::ImageManager;
#[cfg(test)]
use ::image::ImageManagerMock as ImageManager;
```

If you'd like to provide an alternate subsituted name, you can do so:

```rust
#[test_double(IMDummy)]
use ::image::ImageManager;

// Expands to:

#[cfg(not(test))]
use ::image::ImageManager;
#[cfg(test)]
use ::image::IMDummy as ImageManager;
```


## Substituting Multiple Types

If you'd like to substitute multiple types at once, you can use the function-like macro. Note that this does not support providing an alternate substituted name.

```rust
test_doubles! {
    use ::image::ImageManager;
    use ::texture::TextureManager;
}

// Expands to:

#[cfg(not(test))]
use ::image::ImageManager;
#[cfg(test)]
use ::image::ImageManagerMock as ImageManager;
#[cfg(not(test))]
use ::texture::TextureManager;
#[cfg(test)]
use ::texture::TextureManagerMock as TextureManager;
```