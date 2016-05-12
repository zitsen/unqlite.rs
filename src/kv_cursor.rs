use std::mem;
use std::ptr::{self, Shared, Unique};

use libc::c_void;

use ffi::{unqlite, unqlite_kv_cursor, unqlite_kv_cursor_data, unqlite_kv_cursor_data_callback,
          unqlite_kv_cursor_delete_entry, unqlite_kv_cursor_first_entry, unqlite_kv_cursor_init,
          unqlite_kv_cursor_key, unqlite_kv_cursor_key_callback, unqlite_kv_cursor_last_entry,
          unqlite_kv_cursor_next_entry, unqlite_kv_cursor_prev_entry, unqlite_kv_cursor_release,
          unqlite_kv_cursor_reset, unqlite_kv_cursor_seek, unqlite_kv_cursor_valid_entry};
use ffi::constants::{UNQLITE_CURSOR_MATCH_EXACT, UNQLITE_CURSOR_MATCH_GE, UNQLITE_CURSOR_MATCH_LE};

use UnQlite;
use error::{Result, Wrap};

/// Cursor iterator interfaces.
///
/// Cursors provide a mechanism by which you can iterate over the records in a database. Using
/// cursors, you can seek, fetch, move, and delete database records.
///
/// To iterate over database records, from the first record to the last, simply call `first`
/// to get the first valid cursor and loop to the next:
///
/// ```
/// # extern crate unqlite;
/// #
/// use unqlite::{UnQlite, Cursor};
/// #
/// # #[cfg(feature = "enable-threads")]
/// # fn main() {
/// let unqlite = UnQlite::create_temp();
/// let mut entry = unqlite.first();
///
/// loop {
///     if entry.is_none() { break; }
///
///     let record = entry.expect("valid entry");
///     println!("{:?}", record.key_value());
///     entry = record.next();
/// }
/// # }
/// # #[cfg(not(feature = "enable-threads"))]
/// # fn main() { }
/// ```
///
/// To iterate over database records, from the last record to the first, just replace `first` as
/// `last`, call `prev` instead of `next()` on `entry`.
///
/// You can also use cursors to search for records and start the iteration process from there.
/// To do that, start from [`seek`](#tymethod.seek) method.
///
/// To retrieve record key/value from a valid cursor, just use like:
///
/// ```ignore
/// let entry = ...; // Get the cursor entry
/// let key = entry.key();                   // Key only
/// let value = entry.value();               // Value only
/// let (key, value) = entry.key_value();    // Key-Value pair
/// ```
///
/// To delete a record from the database using the cursor interface, simply point to the target
/// record using `seek` and call `delete` on the `Entry` object.
///
/// A rusty `Iterator` style would perform in a short time.
pub trait Cursor {
    /// Returns the first entry.
    fn first(&self) -> Option<Entry>;

    /// Retruns the last entry.
    fn last(&self) -> Option<Entry>;

    /// Seek an entry by `key`.
    ///
    /// The `pos` `Direction` options:
    ///
    ///   * **Exact**: If the record exists, the cursor is left pointing to it,
    /// otherwise return `None`.
    ///   * **Le**: The cursor is left pointing to the largest key in the database that is smaller
    /// than `key`, If the database contains no keys smaller than `key`, it returns `None`.
    ///   * **Ge**: Oppsite to **Le**, it returns the smallest `Entry` in the database that is
    ///   larger than `key`.If the database contains no keys smaller than `key`, return `None`.
    fn seek<K: AsRef<[u8]>>(&self, key: K, pos: Direction) -> Option<Entry>;
}

impl Cursor for UnQlite {
    fn first(&self) -> Option<Entry> {
        RawCursor::init(&self)
            .and_then(|cur| cur.first())
            .ok()
            .and_then(|cur| cur.valid())
            .map(|cur| Entry(cur))
    }
    fn last(&self) -> Option<Entry> {
        RawCursor::init(&self)
            .and_then(|cur| cur.last())
            .ok()
            .and_then(|cur| cur.valid())
            .map(|cur| Entry(cur))
    }
    fn seek<K: AsRef<[u8]>>(&self, key: K, pos: Direction) -> Option<Entry> {
        RawCursor::init(&self)
            .and_then(|cur| cur.seek(key, pos))
            .ok()
            .and_then(|cur| cur.valid())
            .map(|cur| Entry(cur))
    }
}

/// A valid cursor entry of record.
pub struct Entry(RawCursor);

impl Entry {
    /// Returns the key of record
    pub fn key(&self) -> Vec<u8> {
        self.0.key().unwrap()
    }
    /// Returns the value
    pub fn value(&self) -> Vec<u8> {
        self.0.value().unwrap()
    }
    /// Returns the key-value pair
    pub fn key_value(&self) -> (Vec<u8>, Vec<u8>) {
        self.0.key_value().unwrap()
    }

    /// Use mangle function for callback of key.
    ///
    /// The callback function should define as this:
    ///
    /// ```ignore
    /// #[no_mangle]
    /// pub extern fn print_data(ptr: *const c_void, len: u32, _data: *mut c_void) -> i32 {
    ///     // Do stuff with (ptr, len)
    ///     println!("Key/Value length is {}", len);
    ///     0
    /// }
    /// ```
    pub fn key_callback(&self,
                        func: extern "C" fn(*const c_void, u32, *mut c_void) -> i32,
                        data: *mut c_void) {
        self.0.key_callback(func, data)
    }

    /// Use mangle function for callback of value
    pub fn value_callback(&self,
                          func: extern "C" fn(*const c_void, u32, *mut c_void) -> i32,
                          data: *mut c_void) {
        self.0.value_callback(func, data)
    }


    /// Goto next entry.
    ///
    /// Returns `None` if there's no valid cursors.
    pub fn next(self) -> Option<Self> {
        self.0
            .next()
            .ok()
            .and_then(|raw| raw.valid())
            .map(|raw| Entry(raw))
    }

    /// Goto previous entry.
    ///
    /// Returns `None` if no valid cursors.
    pub fn prev(self) -> Option<Self> {
        self.0
            .prev()
            .ok()
            .and_then(|raw| raw.valid())
            .map(|raw| Entry(raw))
    }

    /// Delete the pointed record.
    pub fn delete(self) -> Option<Self> {
        self.0.delete().map(|raw| Entry(raw)).ok()
    }
}

pub enum Direction {
    /// Seek the cursor exactly
    Exact = UNQLITE_CURSOR_MATCH_EXACT as isize,
    Le = UNQLITE_CURSOR_MATCH_LE as isize,
    Ge = UNQLITE_CURSOR_MATCH_GE as isize,
}

struct RawCursor {
    engine: Shared<unqlite>,
    cursor: Unique<unqlite_kv_cursor>,
}

macro_rules! eval {
    ($i: ident, $($e: expr),*) => (
        unsafe {
            concat_idents!(unqlite_kv_cursor_, $i)($($e),*)
        }
    );
}

macro_rules! wrap {
    ($i: ident, $($e: expr),*) => (eval!($i, $($e),*).wrap());
}

macro_rules! wrap_in_place {
    ($self_:ident, $i: ident) => (
        wrap!($i, $self_.cursor()).map(|_| $self_)
    );
    ($self_:ident, $i: ident, $($e: expr),+) => (
        wrap!($i, $self_.cursor(), $($e),+).map(|_| $self_)
    );
}

impl RawCursor {
    /// Opening Database Cursors
    pub fn init(unqlite: &UnQlite) -> Result<Self> {
        let mut cursor: *mut unqlite_kv_cursor = unsafe { mem::uninitialized() };
        wrap!(init, unqlite.as_raw_mut_ptr(), &mut cursor).map(|_| {
            RawCursor {
                engine: unqlite.engine.clone(),
                cursor: unsafe { Unique::new(cursor) },
            }
        })
    }

    #[allow(dead_code)]
    pub fn reset(self) -> Result<Self> {
        wrap_in_place!(self, reset)
    }

    pub fn release(&self) -> Result<()> {
        wrap!(release, self.engine(), self.cursor())
    }

    /// # Positioning Database Cursors
    ///
    /// * seek
    /// * first
    /// * last
    /// * next
    /// * prev
    ///
    pub fn seek<Key: AsRef<[u8]>>(self, key: Key, pos: Direction) -> Result<Self> {
        wrap_in_place!(self,
                       seek,
                       key.as_ref().as_ptr() as _,
                       key.as_ref().len() as _,
                       pos as _)
    }
    pub fn first(self) -> Result<Self> {
        wrap_in_place!(self, first_entry)
    }
    pub fn last(self) -> Result<Self> {
        wrap_in_place!(self, last_entry)
    }
    pub fn next(self) -> Result<Self> {
        wrap_in_place!(self, next_entry)
    }
    pub fn prev(self) -> Result<Self> {
        wrap_in_place!(self, prev_entry)
    }

    /// Check if the cursor reperesent a valid entry
    pub fn is_valid(&self) -> bool {
        match eval!(valid_entry, self.cursor()) {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    pub fn valid(self) -> Option<Self> {
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }

    /// Extracting Data from Database Cursors
    pub fn key(&self) -> Result<Vec<u8>> {
        debug_assert!(self.is_valid());

        self.key_len().and_then(|mut len| {
            let ptr = unsafe { ::libc::malloc(len as ::libc::size_t) };
            wrap!(key, self.cursor(), ptr, &mut len)
                .map(|_| unsafe { Vec::from_raw_parts(ptr as _, len as _, len as _) })
        })
    }

    pub fn key_callback(&self,
                        func: extern "C" fn(*const c_void, u32, *mut c_void) -> i32,
                        data: *mut c_void) {
        eval!(key_callback, self.cursor(), Some(func), data);
    }

    pub fn value(&self) -> Result<Vec<u8>> {
        debug_assert!(self.is_valid());

        self.value_len().and_then(|mut len| {
            let ptr = unsafe { ::libc::malloc(len as _) };
            wrap!(data, self.cursor(), ptr, &mut len)
                .map(|_| unsafe { Vec::from_raw_parts(ptr as _, len as _, len as _) })
        })
    }

    pub fn value_callback(&self,
                          func: extern "C" fn(*const c_void, u32, *mut c_void) -> i32,
                          data: *mut c_void) {
        eval!(data_callback, self.cursor(), Some(func), data);
    }

    pub fn key_value(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        self.key().and_then(|key| self.value().map(|value| (key, value)))
    }


    /// Deleting Records using Database Cursors
    pub fn delete(self) -> Result<Self> {
        wrap_in_place!(self, delete_entry)
    }

    pub fn key_len(&self) -> Result<i32> {
        let mut len = 0i32;
        wrap!(key, self.cursor(), ptr::null_mut() as _, &mut len).map(|_| len)
    }
    pub fn value_len(&self) -> Result<i64> {
        let mut len = 0i64;
        wrap!(data, self.cursor(), ptr::null_mut() as _, &mut len).map(|_| len)
    }

    unsafe fn cursor(&self) -> *mut unqlite_kv_cursor {
        *self.cursor
    }
    unsafe fn engine(&self) -> *mut unqlite {
        *self.engine
    }
}

impl Drop for RawCursor {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use {KV, UnQlite};
    use std::ptr;
    use libc::c_void;
    use super::*;

    macro_rules! _test_assert_eq {
        ($lhs:expr, ($rhs_0:expr, $rhs_1:expr)) => {
            {
                let kv = $lhs;
                assert_eq!(
                    (String::from_utf8(kv.0).unwrap(), String::from_utf8(kv.1).unwrap()),
                    ($rhs_0.to_string(), $rhs_1.to_string()))
            }
        };
        ($lhs:expr, $rhs:expr) => (
            assert_eq!(String::from_utf8($lhs).unwrap(), $rhs.to_string())
        );
    }

    #[no_mangle]
    pub extern fn print_data(ptr: *const c_void, len: u32, _data: *mut c_void) -> i32 {
        println!("Key callback: {:?}", ptr);
        0
    }

    #[test]
    fn test_kv_cursor() {
        let mut unqlite = UnQlite::create_in_memory();
        unqlite.kv_store("abc", "1").unwrap();
        unqlite.kv_store("cde", "3").unwrap();
        unqlite.kv_store("bcd", "2").unwrap();

        let entry = unqlite.first().unwrap();
        _test_assert_eq!(entry.key(), "abc");
        _test_assert_eq!(entry.value(), "1");
        _test_assert_eq!(entry.key_value(), ("abc", "1"));
        let entry = entry.next().unwrap();
        _test_assert_eq!(entry.key(), "cde");
        _test_assert_eq!(entry.value(), "3");
        _test_assert_eq!(entry.key_value(), ("cde", "3"));
        let entry = entry.next().unwrap();
        entry.key_callback(print_data, ptr::null_mut());
        _test_assert_eq!(entry.key(), "bcd");
        _test_assert_eq!(entry.value(), "2");
        _test_assert_eq!(entry.key_value(), ("bcd", "2"));
        let entry = entry.next(); // Now reach the end
        assert!(entry.is_none());

        let mut entry = unqlite.last();
        loop {
            if entry.is_none() {
                break;
            }

            let current = entry.expect("valid entry");
            println!("{:?}", current.key_value());
            entry = current.prev();
        }
    }
}
