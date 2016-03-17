use std::mem;
use std::ptr;
use libc::c_void;
use super::UnQlite;
use ffi::{unqlite, unqlite_kv_cursor, unqlite_kv_cursor_data, unqlite_kv_cursor_data_callback,
          unqlite_kv_cursor_delete_entry, unqlite_kv_cursor_first_entry, unqlite_kv_cursor_init,
          unqlite_kv_cursor_key, unqlite_kv_cursor_key_callback, unqlite_kv_cursor_last_entry,
          unqlite_kv_cursor_next_entry, unqlite_kv_cursor_prev_entry, unqlite_kv_cursor_release,
          unqlite_kv_cursor_reset, unqlite_kv_cursor_seek, unqlite_kv_cursor_valid_entry};
use ffi::constants::{UNQLITE_CURSOR_MATCH_EXACT, UNQLITE_CURSOR_MATCH_GE, UNQLITE_CURSOR_MATCH_LE};

impl<'kv_cursor> UnQlite {
    fn iter(&self) -> Cursor {
        unsafe {
            let mut cursor: *mut unqlite_kv_cursor = mem::uninitialized();
            error_or!(unqlite_kv_cursor_init(self.db, &mut cursor))
                .ok()
                .map(|_| {
                    Cursor {
                        db: self.db,
                        cursor: cursor,
                    }
                })
                .unwrap()
        }
    }

    fn first(&self) -> Option<Cursor> {
        self.iter().first_entry()
    }
}

trait CursorBase: Sized {
    fn unqlite(&self) -> *mut unqlite;
    fn cursor(&self) -> *mut unqlite_kv_cursor;
}

trait CursorReader: CursorBase {
    fn seek<K: AsRef<[u8]>>(self, key: K, pos: Direction) -> Option<Self>;
    /* {
        error_or!(unsafe {
            unqlite_kv_cursor_seek(self.cursor(),
                                   key.as_ref().as_ptr() as *const c_void,
                                   key.as_ref().len() as i32,
                                   pos as i32)
        })
            .ok()
            .and_then(|_| self.check())
    }*/

    fn first_entry(self) -> Option<Self>; /* {
        error_or!(unsafe { unqlite_kv_cursor_first_entry(self.cursor()) })
            .ok()
            .and_then(|_| self.check())
    }*/

    fn last_entry(self) -> Option<Self>; /* {
        error_or!(unsafe { unqlite_kv_cursor_last_entry(self.cursor()) })
            .ok()
            .and_then(|_| self.check())
    }*/

    fn next(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_next_entry(self.cursor()) })
            .ok()
            .and_then(|_| self.check())
    }

    fn prev_entry(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_prev_entry(self.cursor()) })
            .ok()
            .and_then(|_| self.check())
    }

    fn is_valid(&self) -> bool {
        match unsafe { unqlite_kv_cursor_valid_entry(self.cursor()) } {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    fn delete_entry(&mut self) -> () {
        error_or!(unsafe { unqlite_kv_cursor_delete_entry(self.cursor()) }).unwrap();
    }

    fn key_len(&self) -> Option<i32> {
        unsafe {
            let mut len = 0i32;
            error_or!(unqlite_kv_cursor_key(self.cursor(), ptr::null_mut() as *mut c_void, &mut len))
                .ok()
                .and_then(|_| Some(len))
        }
    }
    fn key(&self) -> Option<Vec<u8>> {
        self.key_len().and_then(|mut len| unsafe {
            let ptr = ::libc::malloc(len as ::libc::size_t);
            error_or!(unqlite_kv_cursor_key(self.cursor(), ptr, &mut len))
                .ok()
                .map(|_| Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize))
        })
    }

    fn data_len(&self) -> Option<i64> {
        unsafe {
            let mut len = 0i64;
            error_or!(unqlite_kv_cursor_data(self.cursor(), ptr::null_mut() as *mut c_void, &mut len))
                .ok()
                .and_then(|_| Some(len))
        }
    }

    fn data(&self) -> Option<Vec<u8>> {
        self.data_len().and_then(|mut len| unsafe {
            let ptr = ::libc::malloc(len as ::libc::size_t);
            error_or!(unqlite_kv_cursor_data(self.cursor(), ptr, &mut len))
                .ok()
                .map(|_| Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize))
        })
    }

    fn check(self) -> Option<Self> {
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cursor {
    db: *mut unqlite,
    cursor: *mut unqlite_kv_cursor,
}

pub struct Iter {
    cursor: Cursor,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        error_or!(unsafe { unqlite_kv_cursor_release(self.db, self.cursor) }).unwrap();
    }
}

pub enum Direction {
    Exact = UNQLITE_CURSOR_MATCH_EXACT as isize,
    Le = UNQLITE_CURSOR_MATCH_LE as isize,
    Ge = UNQLITE_CURSOR_MATCH_GE as isize,
}

pub use self::Direction::*;

impl Cursor {
    fn seek<K: AsRef<[u8]>>(self, key: K, pos: Direction) -> Option<Self> {
        error_or!(unsafe {
            unqlite_kv_cursor_seek(self.cursor,
                                   key.as_ref().as_ptr() as *const c_void,
                                   key.as_ref().len() as i32,
                                   pos as i32)
        })
            .ok()
            .and_then(|_| self.check())
    }

    fn first_entry(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_first_entry(self.cursor) })
            .ok()
            .and_then(|_| self.check())
    }

    fn last_entry(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_last_entry(self.cursor) })
            .ok()
            .and_then(|_| self.check())
    }

    fn next(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_next_entry(self.cursor) })
            .ok()
            .and_then(|_| self.check())
    }

    fn prev_entry(self) -> Option<Self> {
        error_or!(unsafe { unqlite_kv_cursor_prev_entry(self.cursor) })
            .ok()
            .and_then(|_| self.check())
    }

    fn is_valid(&self) -> bool {
        match unsafe { unqlite_kv_cursor_valid_entry(self.cursor) } {
            1 => true,
            0 => false,
            _ => unreachable!(),
        }
    }

    fn delete_entry(&mut self) -> () {
        error_or!(unsafe { unqlite_kv_cursor_delete_entry(self.cursor) }).unwrap();
    }

    fn key_len(&self) -> Option<i32> {
        unsafe {
            let mut len = 0i32;
            error_or!(unqlite_kv_cursor_key(self.cursor, ptr::null_mut() as *mut c_void, &mut len))
                .ok()
                .and_then(|_| Some(len))
        }
    }
    pub fn key(&self) -> Option<Vec<u8>> {
        self.key_len().and_then(|mut len| unsafe {
            let ptr = ::libc::malloc(len as ::libc::size_t);
            error_or!(unqlite_kv_cursor_key(self.cursor, ptr, &mut len))
                .ok()
                .map(|_| Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize))
        })
    }

    fn data_len(&self) -> Option<i64> {
        unsafe {
            let mut len = 0i64;
            error_or!(unqlite_kv_cursor_data(self.cursor, ptr::null_mut() as *mut c_void, &mut len))
                .ok()
                .and_then(|_| Some(len))
        }
    }

    pub fn data(&self) -> Option<Vec<u8>> {
        self.data_len().and_then(|mut len| unsafe {
            let ptr = ::libc::malloc(len as ::libc::size_t);
            error_or!(unqlite_kv_cursor_data(self.cursor, ptr, &mut len))
                .ok()
                .map(|_| Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize))
        })
    }

    fn check(self) -> Option<Self> {
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }
}

impl Iterator for Cursor {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        self.clone().next()
    }
}

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use super::super::UnQlite;

    #[test]
    fn test_kv_cursor() {
        let mut unqlite = UnQlite::create_in_memory();
        unqlite.kv_store("abc", "1");
        unqlite.kv_store("cde", "3");
        unqlite.kv_store("bcd", "2");

        for cursor in unqlite.iter() {
            let key = cursor.key().unwrap();
            let data = cursor.data().unwrap();
            println!("{:?}: {:?}, {:?}", cursor, String::from_utf8(key), String::from_utf8(data));
        }
    }
}
