vault
================

vault is a Company of Heroes 2 replay parsing library written in Rust. It has evolved from work done by a number of individuals in the Company of Heroes community, most notably Seb and pingtoft, and has been helped along by the assistance of Relic Entertainment. This particular project is a rewrite and extension of a parser written by Seb, which itself was an extension of my original PHP port of pingtoft's C# Company of Heroes parsing code. Its goal is to provide a robust, complete, and flexible interface into Company of Heroes 2 replay files through a general library that can be used in a variety of environments and languages. It is also written to be fast and to take advantage of Rust's thread and memory safety features.

# File & Version Support

Currently vault supports Company of Heroes 2 replays recorded on version 19545 or higher. This was the first version after the release of the British forces expansion.

vault can handle parsing individual files, archived files in the zip format (.zip only, not .7z currently), and directories of replay files. When parsing an archive, all .rec files inside the archive and its subfolders will be parsed. When parsing a directory, all .rec and .zip files at that directory level will be parsed. vault currently will not recursively traverse subdirectories when parsing directories.

# flank

flank is a very basic CLI parsing application for CoH2 replays written using this library as a proof of concept and reference point. It is currently powering COH2.ORG's replay section. If you would like to try it out or look through the code, it is available on GitHub.

Usage
================

## Rust

If you are writing a Rust application, you can use vault from Crates.io:

`Cargo.toml`:

```toml
[dependencies]
vault = "0.1"
```

`src/main.rs`:

```rust
extern crate vault;

use std::path::Path;
use vault::Vault;

fn main() {
    let path = Path::new("/path/to/replay.rec");
    let vault = Vault::parse(&path).unwrap();
    println!("{}", vault.to_json().unwrap());
}
```

## FFI

One of the great strengths of Rust is its ability to be called into from foreign code as easily as C. This can be used to parse replays with vault from a higher-level language such as Python or Javascript.

NOTE: vault's FFI interface currently takes advantage of Rust functions only available in the unstable nightly builds of the Rust compiler. Please make sure you have Rust nightly installed before you try to build vault for FFI.

First, build vault from source:

```bash
git clone https://github.com/ryantaylor/vault.git && cd vault
cargo build --release
```

The library you want to be using is `libvault.so`, which you can find at `vault/target/release`. This library exposes two external functions for use over FFI:

```rust
pub extern fn parse_to_cstring(path: *const c_char) -> *mut c_char {
    // ...
}
```
This function takes a string path to a replay, archive, or directory and parses the replay file(s) it finds. It then serializes the result and passes it back to the caller as a JSON string.

```rust
pub extern fn free_cstring(ptr: *mut c_char) {
    // ...
}
```

This function takes the pointer passed back from `parse_to_cstring` and deallocates its memory. Every call to `parse_to_cstring` must have a matching call to `free_cstring` in order to prevent memory leaks. `free_cstring` is the only way to safely deallocate the pointer returned by `parse_to_cstring`.

`node.js`

```javascript
var ffi = require('ffi');
var ref = require('ref');

var charPtr = ref.refType(ref.types.CString);

var lib = ffi.Library('/path/to/vault/target/release/libvault', {
    'parse_to_cstring': [charPtr, ['string']],
    'free_cstring': ['void', [charPtr]]
});

var path = '/path/to/replay.rec';
var ptr = lib.parse_to_cstring(path);
var str = ref.readCString(ptr, 0);
lib.free_cstring(ptr);

console.log(str);
```

IMPORTANT: Failing to call `free_cstring` on the pointer returned by `parse_to_cstring` will cause a memory leak, and calling `free_cstring` on another pointer will likely cause a segfault.