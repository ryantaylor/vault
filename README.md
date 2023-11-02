# vault

[![crates.io](https://img.shields.io/crates/v/vault.svg)](https://crates.io/crates/vault) [![Documentation](https://img.shields.io/badge/View-Documentation-blue.svg)](https://docs.rs/vault/5.0.0/vault/)

`vault` is a Company of Heroes replay parsing library written in [Rust](https://www.rust-lang.org/). It has been completely rewritten for Company of Heroes 3 to provide a more intuitive interface while simplifying the code and leveraging [nom](https://github.com/rust-bakery/nom)'s parser combinators to enable clean, fast parsing of Company of Heroes 3 replay files.

Note that this project is still under development, and as such not all information is currently being parsed from CoH3 replay files; namely, command parsing has not yet been implemented. Core player, map, and chat information is accessible however.

# Usage

## Rust

If you are writing a Rust application, you can use `vault` from [crates.io](https://crates.io/crates/vault):

`Cargo.toml`:

```toml
[dependencies]
vault = "5"
```

`src/main.rs`:

```rust
fn main() {
    let data = include_bytes!("/path/to/replay.rec");
    let replay = vault::Replay::from_bytes(data);
    assert!(replay.is_ok())
}
```

## Ruby

`vault` ships with Ruby bindings via [magnus](https://github.com/matsadler/magnus), which allows you to call into `vault` from Ruby code directly. This can be enabled with the `magnus` feature:

`Cargo.toml`:

```toml
[dependencies]
vault = { version = "4", features = ["magnus"] }
```

`src/lib.rs`:

```rust
use magnus::{class, define_module, exception, function, method, prelude::*, Error};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VaultCoh")?;

    let replay = module.define_class("Replay", class::object())?;
    replay.define_singleton_method("from_bytes", function!(from_bytes, 1))?;
    replay.define_method("version", method!(vault::Replay::version, 0))?;

    Ok(())
}

fn from_bytes(input: Vec<u8>) -> Result<vault::Replay, Error> {
    vault::Replay::from_bytes(&input)
        .map_err(|err| Error::new(exception::runtime_error(), err.to_string()))
}
```

`irb`:

```ruby
require 'vault'

bytes = File.open('/path/to/replay.rec').read.unpack('C*')
replay = VaultCoh::Replay.from_bytes(bytes)
puts replay.version
```

Note that all classes must be bound to the `VaultCoh` namespace, with class names matching their Rust counterparts. For an example of this functionality in action, see [vault-rb](https://github.com/ryantaylor/vault-rb).

## Serde

`vault` implements [serde](https://serde.rs/)'s `Serialize` and `Deserialize` traits for all data structures that make up a parsed replay. These can be accessed via the `serde` feature:

`Cargo.toml`:

```toml
[dependencies]
vault = { version = "4", features = ["serde"] }
```

`src/main.rs`:

```rust
fn main() {
    let data = include_bytes!("/path/to/replay.rec");
    let replay = vault::Replay::from_bytes(data).unwrap();

    // Convert the Replay to a JSON string.
    let serialized = serde_json::to_string(&replay).unwrap();

    // Convert the JSON string back to a Replay.
    let deserialized: Replay = serde_json::from_str(&serialized).unwrap();
}
```

## Company of Heroes 2

`vault` has been rewritten from scratch to better support future development, which means Company of Heroes 2 parsing support has been deprecated. [The CoH2 parser and usage instructions can be found here](https://github.com/ryantaylor/vault/tree/v1.0.0). CoH2 replay parsing will continue to work with v1.0.0 of `vault`.

# Compatibility

Official minimum supported Rust version is 1.65.0, because this is the version magnus requires. However, building without Ruby bindings should be fine on any compiler version that supports Rust 2021, though this isn't officially supported.

Ruby bindings have some additional compatibility requirements, such as libclang and minimum Ruby version requirements. For more information see [magnus compatibility](https://github.com/matsadler/magnus#compatibility).

# Documentation

Documentation for `vault` [can be viewed online](https://docs.rs/vault/5.0.0/vault/).

Alternatively, you can easily build an offline copy of the documentation for yourself with `cargo`:

```
$ cargo doc
```

For documentation that includes the magnus Ruby bindings, run:

```
$ cargo doc --features=magnus
```

The resulting documentation can then be found at `vault/target/doc`.

# License

MIT
