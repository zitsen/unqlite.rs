# unqlite

A high-level UnQLite database engine wrapper.

[![travis-badge][]][travis] [![release-badge][]][cargo] [![downloads]][cargo]
[![docs-badge][]][docs] [![license-badge][]][cargo]

NOTE: Some of the documents is stolen from [UnQLite Official Website][unqlite].

## What is UnQLite?

>
UnQLite is a software library which implements a *self-contained*, *serverless*,
zero-configuration, transactional NoSQL database engine. UnQLite is a document store database
similar to [MongoDB], [Redis], [CouchDB] etc. as well a standard Key/Value store similar to
[BerkeleyDB], [LevelDB], etc.
>
UnQLite is an embedded NoSQL (Key/Value store and Document-store) database engine. Unlike most
other NoSQL databases, UnQLite does not have a separate server process. UnQLite reads and
writes directly to ordinary disk files. A complete database with multiple collections, is
contained in **a single disk file**. The database file format is cross-platform, you can freely
copy a database between 32-bit and 64-bit systems or between big-endian and little-endian
architectures.

## Port to Rust

This crate is high-level UnQLite database wrapper for Rust. A low-level bindings wrapper
is avaliable as a seperated crate: [unqlite-sys](https://crates.io/crates/unqlite-sys).

## Usage

You can start with `UnQLite` constructors:

```rust
extern crate unqlite;

use unqlite::{UnQLite, Config, KV, Cursor};

fn main() {
    // The database memory is not handled by Rust, and the database is on-disk,
    // so `mut` is not neccessary.
    let unqlite = UnQLite::create_temp();
    // Use any type that can use as `[u8]`
    unqlite.kv_store("key", "a long length value").unwrap();
    unqlite.kv_store("abc", [1,2,3]).unwrap();

    let mut entry = unqlite.first();
    // Iterate records
    loop {
        if entry.is_none() { break; }

        let record = entry.expect("valid entry");
        let (key, value) = record.key_value();
        println!("* Go through {:?} --> {:?}", key, value);

        if value.len() > 10 {
            println!("** Delete key {:?} by value length", key);
            entry = record.delete();
        } else {
            entry = record.next();
        }
    }
    //panic!("for test");
}
```

## Contributors

- **@bemyak**
- **@chritchens**
- **@wolandr**
- **@timlyo**

[unqlite]: https://unqlite.org/index.html
[travis-badge]: https://img.shields.io/travis/zitsen/unqlite.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zitsen/unqlite.rs
[release-badge]: https://img.shields.io/crates/v/unqlite.svg?style=flat-square
[downloads]: https://img.shields.io/crates/d/unqlite.svg?style=flat-square
[cargo]: https://crates.io/crates/unqlite
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://zitsen.github.io/unqlite.rs
[license-badge]: https://img.shields.io/crates/l/unqlite.svg?style=flat-square
