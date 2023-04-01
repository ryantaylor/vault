# vault

[![Documentation](https://img.shields.io/badge/View-Documentation-blue.svg)](http://ryantaylor.github.io/vault/v2.0.0/vault/index.html)

`vault` is a Company of Heroes replay parsing library written in [Rust](https://www.rust-lang.org/). It has been completely rewritten for Company of Heroes 3 to provide a more intuitive interface while simplifying the code and leveraging [nom](https://github.com/rust-bakery/nom)'s parser combinators to enable clean, fast parsing of Company of Heroes 3 replay files.

Note that this project is still under development, and as such not all information is currently being parsed from CoH3 replay files; namely, command parsing has not yet been implemented. Core player, map, and chat information is accessible however.

# Usage

## Rust

If you are writing a Rust application, you can use `vault` from [crates.io](https://crates.io/crates/vault):

`Cargo.toml`:

```toml
[dependencies]
vault = "2"
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
vault = { version = "2", features = ["magnus"] }
```

`src/lib.rs`:

```rust
use magnus::{class, define_module, exception, function, method, prelude::*, Error};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Vault")?;

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
replay = Vault::Replay.from_bytes(bytes)
puts replay.version
```

Note that all classes must be bound to the `Vault` namespace, with class names matching their Rust counterparts. For an example of this functionality in action, see [vault-rb](https://github.com/ryantaylor/vault-rb).

## Company of Heroes 2

`vault` has been rewritten from scratch to better support future development, which means Company of Heroes 2 parsing support has been deprecated. [The CoH2 parser and usage instructions can be found here](https://github.com/ryantaylor/vault/tree/v1.0.0). CoH2 replay parsing will continue to work with v1.0.0 of `vault`.

# Documentation

Documentation for `vault` [can be viewed online](http://ryantaylor.github.io/vault/v2.0.0/vault/index.html).

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
