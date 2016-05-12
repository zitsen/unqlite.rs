//! A high-level UnQlite database engine wrapper.
//!
//! NOTE: Some of the documents is stolen from [UnQlite Offical Website][unqlite].
//!
//! # What is UnQlite?
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
//! This crate is high-level UnQlite database wrapper for Rust. A low-level bindings wrapper
//! is avaliable as a seperated crate: [unqlite-sys](https://crates.io/crates/unqlite-sys).
//!
//! # Usage
//!
//! You can start with `UnQlite` constructors:
//!
//! ```
//! extern crate unqlite;
//!
//! use unqlite::{UnQlite, Config, KV, Cursor};
//!
//! # #[cfg(feature = "enable-threads")]
//! fn main() {
//!     // The database memory is not handled by Rust, and the database is on-disk,
//!     // so `mut` is not neccessary.
//!     let unqlite = UnQlite::create_temp();
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

#![feature(shared)]
#![feature(unique)]
#![feature(concat_idents)]

extern crate unqlite_sys as ffi;
extern crate libc;

#[cfg(test)]
extern crate tempfile;

use std::ffi::CString;
use std::mem;
use std::ptr::Shared;

use ffi::{unqlite_close, unqlite_open};

use error::Wrap;
pub use error::{Error, Result};

/// UnQlite database entry point.
///
/// UnQlite support both in-memory and on-disk database.
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
pub struct UnQlite {
    engine: Shared<::ffi::unqlite>,
}

macro_rules! eval {
    ($i: ident, $($e: expr),*) => (
        unsafe {
            concat_idents!(unqlite_, $i)($($e),*)
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

impl UnQlite {
    /// Create UnQlite database at specific path.
    ///
    /// ```ignore
    /// let _ = UnQlite::open("str");
    /// let _ = UnQlite::open(String::new());
    /// ```
    #[inline]
    fn open<P: AsRef<str>>(filename: P, mode: OpenMode) -> Result<UnQlite> {
        let mut db: *mut ::ffi::unqlite = unsafe { mem::uninitialized() };
        let filename = filename.as_ref();
        let filename = try!(CString::new(filename));
        wrap!(open, &mut db, filename.as_ptr(), mode.into())
            .map(|_| UnQlite { engine: unsafe { Shared::new(db) } })
    }

    /// Create UnQlite database as `filename`.
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
    /// let _ = UnQlite::create("test.db");
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
    pub fn create<P: AsRef<str>>(filename: P) -> UnQlite {
        Self::open(filename, OpenMode::Create).unwrap()
    }

    /// Create database in memory.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// let _ = UnQlite::create(":mem:");
    /// ```
    /// ## Panics
    ///
    /// Will panic if failed in creating.
    ///
    #[inline]
    pub fn create_in_memory() -> UnQlite {
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
    pub fn create_temp() -> UnQlite {
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
    pub fn open_mmap<P: AsRef<str>>(filename: P) -> UnQlite {
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
    pub fn open_readonly<P: AsRef<str>>(filename: P) -> UnQlite {
        Self::open(filename, OpenMode::ReadOnly).unwrap()
    }

    fn close(&self) -> Result<()> {
        wrap!(close, self.as_raw_mut_ptr())
    }

    unsafe fn as_raw_mut_ptr(&self) -> *mut ::ffi::unqlite {
        *self.engine
    }
}

unsafe impl Send for UnQlite {}
unsafe impl Sync for UnQlite {}

impl Drop for UnQlite {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

mod error;
mod openmode;
mod config;
mod util;
mod transaction;
mod kv_store;
mod kv_cursor;

pub use self::openmode::OpenMode;
pub use self::config::Config;
pub use self::transaction::Transaction;
pub use self::util::*;
pub use self::kv_store::*;
pub use self::kv_cursor::*;

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests_threadsafe {
    use super::UnQlite;

    #[test]
    fn create_temp() {
        let _ = UnQlite::create_temp();
    }

    #[test]
    fn create_in_memory() {
        let _ = UnQlite::create_in_memory();
    }

    #[test]
    fn from_readonly_memory() {
        let _ = UnQlite::open_readonly(":mem:");
    }
}

#[cfg(test)]
mod tests {
    use super::UnQlite;

    #[test]
    fn open() {
        let _ = UnQlite::create_temp();
        let _ = UnQlite::create_in_memory();
        let _ = UnQlite::open_readonly(":mem:");
    }
}
