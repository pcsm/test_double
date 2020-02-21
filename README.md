test_double [![Crates.io](https://img.shields.io/crates/v/test_double.svg)](https://crates.io/crates/test_double) [![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-blue.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)
==================================================================

A set of procedural macros that can swap in mock objects, dummy objects, or other test doubles only when testing.

To use, add the following to your `Cargo.toml`:

```toml
[dependencies]
test_double = "0.2.0"
```

Note that this crate has not yet reached version 1.0, so the API may change between releases.

## Substituting A Single Type

`#[test_double]` can be used to substitute one type for another when testing:

```rust
#[test_double]
use image::ImageManager;

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::ImageManagerMock as ImageManager;
```

The substituted type name defaults to the original name, postfixed with `Mock`. You can also provide an alternate substituted name to be used instead:

```rust
#[test_double(IMDummy)]
use image::ImageManager;

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::IMDummy as ImageManager;
```

As a shortcut, if you would like the alternate substituted name to be the original name, _prefixed_ with `Mock`, you can use the `#[test_double_prefixed]` macro instead:

```rust
#[test_double_prefixed]
use image::ImageManager;

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::MockImageManager as ImageManager;
```

### Limitations

`#[test_double]` and `#[test_double_prefixed]` can't be used with:

- Glob imports, like `use blah::*;`
- Grouped imports, like `use blah::{foo, bar};`, when providing an alternate substituted name

## Substituting Multiple Types

If you'd like to substitute multiple types at once, you can use the `test_doubles!` macro. Note that this does not support providing an alternate substituted name.

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

Similar to `#[test_double_prefixed]`, there is a `test_doubles_prefixed!` macro that can prefix instead of postfixing:

```rust
test_doubles_prefixed! {
    use image::ImageManager;
    use texture::TextureManager;
}

// Expands to:

#[cfg(not(test))]
use image::ImageManager;
#[cfg(test)]
use image::MockImageManager as ImageManager;
#[cfg(not(test))]
use texture::TextureManager;
#[cfg(test)]
use texture::MockTextureManager as TextureManager;
```

### Limitations

`test_doubles!` can't be used with:

- Glob imports, like `use blah::*;`