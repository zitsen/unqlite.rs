//! A high-level UnQLite database engine wrapper.
//!
//! [![travis-badge][]][travis] [![release-badge][]][cargo] [![downloads]][cargo]
//! [![docs-badge][]][docs] [![license-badge][]][cargo]
//!
//! NOTE: Some of the documents is stolen from [UnQLite Official Website][unqlite].
//!
//! # What is UnQLite?
//!
//! >
//! UnQLite is a software library which implements a *self-contained*, *serverless*,
//! zero-configuration, transactional NoSQL database engine. UnQLite is a document store database
//! similar to [MongoDB], [Redis], [CouchDB] etc. as well a standard Key/Value store similar to
//! [BerkeleyDB], [LevelDB], etc.
//! >
//! UnQLite is an embedded NoSQL (Key/Value store and Document-store) database engine. Unlike most
//! other NoSQL databases, UnQLite does not have a separate server process. UnQLite reads and
//! writes directly to ordinary disk files. A complete database with multiple collections, is
//! contained in **a single disk file**. The database file format is cross-platform, you can freely
//! copy a database between 32-bit and 64-bit systems or between big-endian and little-endian
//! architectures.
//!
//! # Port to Rust
//!
//! This crate is high-level UnQLite database wrapper for Rust. A low-level bindings wrapper
//! is avaliable as a seperated crate: [unqlite-sys](https://crates.io/crates/unqlite-sys).
//!
//! # Usage
//!
//! You can start with `UnQLite` constructors:
//!
//! ```
//! extern crate unqlite;
//!
//! use unqlite::{UnQLite, Config, KV, Cursor};
//!
//! # #[cfg(feature = "enable-threads")]
//! fn main() {
//!     // The database memory is not handled by Rust, and the database is on-disk,
//!     // so `mut` is not neccessary.
//!     let unqlite = UnQLite::create_temp();
//!     // Use any type that can use as `[u8]`
//!     unqlite.kv_store("key", "a long length value").unwrap();
//!     unqlite.kv_store("abc", [1,2,3]).unwrap();
//!
//!     let mut entry = unqlite.first();
//!     // Iterate records
//!     loop {
//!         if entry.is_none() { break; }
//!
//!         let record = entry.expect("valid entry");
//!         let (key, value) = record.key_value();
//!         println!("* Go through {:?} --> {:?}", key, value);
//!
//!         if value.len() > 10 {
//!             println!("** Delete key {:?} by value length", key);
//!             entry = record.delete();
//!         } else {
//!             entry = record.next();
//!         }
//!     }
//!     //panic!("for test");
//! }
//! # #[cfg(not(feature = "enable-threads"))]
//! # fn main() { }
//! ```
//!
//! [unqlite]: https://unqlite.org/index.html
//! [travis-badge]: https://img.shields.io/travis/zitsen/unqlite.rs.svg?style=flat-square
//! [travis]: https://travis-ci.org/zitsen/unqlite.rs
//! [release-badge]: https://img.shields.io/crates/v/unqlite.svg?style=flat-square
//! [downloads]: https://img.shields.io/crates/d/unqlite.svg?style=flat-square
//! [cargo]: https://crates.io/crates/unqlite
//! [docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
//! [docs]: https://zitsen.github.io/unqlite.rs
//! [license-badge]: https://img.shields.io/crates/l/unqlite.svg?style=flat-square

extern crate libc;

#[cfg(test)]
extern crate tempfile;
extern crate paste;

pub use error::{Error, Result};
use error::Wrap;

use ffi::{unqlite_close, unqlite_open};
use std::ffi::CString;
use std::mem;
use std::ptr::NonNull;

/// UnQLite database entry point.
///
/// UnQLite support both in-memory and on-disk database.
/// There's several constructors:
///
/// Constructor | Meaning
/// --- | ---
/// [`create_in_memory`](#method.create_in_memory) | Create a private, in-memory database.
/// [`create_temp`](#method.create_temp) | Create a private, temporary on-disk database.
/// [`create`](#method.create) | Create if not exists, otherwise, open as read-write.
/// [`open_mmap`](#method.open_mmap) | Obtain a read-only memory view of the whole database.
/// [`open_readonly`](#method.open_readonly) | Open the database in a read-only mode.
///
pub struct UnQLite {
    engine: NonNull<::ffi::unqlite>,
}

macro_rules! eval {
    ($i: ident, $($e: expr),*) => (
        loop {
            match unsafe {
                paste::expr! { [<unqlite_ $i>]($($e),*) }
            } {
                crate::vars::UNQLITE_LOCKED => {},
                state => break state
            }
        }
    );
}

macro_rules! wrap {
    ($i: ident, $($e: expr),*) => (eval!($i, $($e),*).wrap());
}

macro_rules! wrap_raw {
    ($self_:ident, $i: ident) => (
        wrap!($i, $self_.as_raw_mut_ptr())
    );
    ($self_:ident, $i: ident, $($e: expr),+) => (
        wrap!($i, $self_.as_raw_mut_ptr(), $($e),+)
    );
}

impl UnQLite {
    /// Create UnQLite database at specific path.
    ///
    /// ```ignore
    /// let _ = UnQLite::open("str");
    /// let _ = UnQLite::open(String::new());
    /// ```
    #[inline]
    fn open<P: AsRef<str>>(filename: P, mode: OpenMode) -> Result<UnQLite> {
        let mut db: *mut ::ffi::unqlite = unsafe { mem::uninitialized() };
        let filename = filename.as_ref();
        let filename = try!(CString::new(filename));
        wrap!(open, &mut db, filename.as_ptr(), mode.into()).map(|_| UnQLite {
            engine: unsafe { NonNull::new_unchecked(db) },
        })
    }

    /// Create UnQLite database as `filename`.
    ///
    /// By default, the database is created in read-write mode.
    ///
    /// ## Panics
    ///
    /// Will panic if failed in creating.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// let _ = UnQLite::create("test.db");
    /// ```
    ///
    /// ## C
    ///
    /// ```c
    /// unqlite *pDb;
    ///
    /// // on-disk database
    /// rc = unqlite_open(&pDb,"test.db",UNQLITE_OPEN_CREATE);
    ///
    /// // in-memory database
    /// rc = unqlite_open(&pDb, ":mem:", UNQLITE_OPEN_MEM);
    /// ```
    #[inline]
    pub fn create<P: AsRef<str>>(filename: P) -> UnQLite {
        Self::open(filename, OpenMode::Create).unwrap()
    }

    /// Create database in memory.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// let _ = UnQLite::create(":mem:");
    /// ```
    /// ## Panics
    ///
    /// Will panic if failed in creating.
    ///
    #[inline]
    pub fn create_in_memory() -> UnQLite {
        Self::create(":mem:")
    }

    /// A private, temporary on-disk database will be created.
    ///
    /// This private database will be automatically deleted as soon as
    /// the database connection is closed.
    ///
    /// ## Panics
    ///
    /// Will panic if failed in creating.
    ///
    /// ## C
    ///
    /// ```c
    /// int rc = unqlite_open("test.db", UNQLITE_OPEN_TEMP_DB);
    /// ```
    #[inline]
    pub fn create_temp() -> UnQLite {
        Self::open("", OpenMode::TempDB).unwrap()
    }

    /// Obtain a read-only memory view of the whole database.
    ///
    /// You will get significant performance improvements with this combination but your database
    /// is still read-only.
    ///
    /// ## Panics
    ///
    /// Panic if open failed.
    ///
    /// ## C
    ///
    /// ```c
    /// unqlite_open(&pDb, "test.db", UNQLITE_OPEN_MMAP | UNQLITE_OPEN_READONLY);
    /// ```
    #[inline]
    pub fn open_mmap<P: AsRef<str>>(filename: P) -> UnQLite {
        Self::open(filename, OpenMode::MMap).unwrap()
    }

    /// Open the database in a read-only mode.
    ///
    /// That is, you cannot perform a store, append, commit or rollback operations with this
    /// control flag.
    ///
    /// Always prefer to use `open_mmap` for readonly in disk database.
    ///
    /// ## Panics
    ///
    /// Panic too.
    ///
    /// ## C
    /// ```c
    /// unqlite_open(&pDb, "test.db", UNQLITE_OPEN_READONLY);
    /// ```
    #[inline]
    pub fn open_readonly<P: AsRef<str>>(filename: P) -> UnQLite {
        Self::open(filename, OpenMode::ReadOnly).unwrap()
    }

    fn close(&self) -> Result<()> {
        wrap!(close, self.as_raw_mut_ptr())
    }

    unsafe fn as_raw_mut_ptr(&self) -> *mut ::ffi::unqlite {
        self.engine.as_ptr()
    }
}

unsafe impl Send for UnQLite {}
unsafe impl Sync for UnQLite {}

impl Drop for UnQLite {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

#[allow(dead_code, non_snake_case, non_camel_case_types)]
pub mod ffi;
#[allow(dead_code)]
pub mod vars;

mod error;
mod openmode;
mod config;
mod util;
mod transaction;
mod kv_store;
mod kv_cursor;
pub mod document;

pub use self::config::Config;
pub use self::kv_cursor::*;
pub use self::kv_store::*;
use self::openmode::OpenMode;
pub use self::transaction::Transaction;
pub use self::util::*;

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests_threadsafe {
    use super::UnQLite;

    #[test]
    fn create_temp() {
        let _ = UnQLite::create_temp();
    }

    #[test]
    fn create_in_memory() {
        let _ = UnQLite::create_in_memory();
    }

    #[test]
    fn from_readonly_memory() {
        let _ = UnQLite::open_readonly(":mem:");
    }
}

#[cfg(test)]
mod tests {
    use super::UnQLite;

    #[test]
    fn open() {
        let _ = UnQLite::create_temp();
        let _ = UnQLite::create_in_memory();
        let _ = UnQLite::open_readonly(":mem:");
    }
}
