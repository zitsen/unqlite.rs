extern crate unqlite_sys as ffi;
extern crate libc;

use std::mem;
use std::ffi::CString;
use libc::c_void;
use ffi::constants::UNQLITE_OK;

use error::*;

pub use mode::*;
#[allow(dead_code, non_camel_case_types)]mod error;
#[allow(dead_code, non_camel_case_types)]mod mode;

pub struct Unqlite {
    db: *mut ffi::unqlite,
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! error_or {
    ($code: expr, $ok: expr) => {
        match $code {
            UNQLITE_OK => Ok($ok),
            code => Err(Error::from(code))
        }
    }
}
impl Default for Unqlite {
    fn default() -> Unqlite {
        Unqlite { db: unsafe { mem::uninitialized() } }
    }
}

impl Unqlite {
    pub fn open(filename: &str, mode: OpenMode) -> Result<Unqlite> {
        unsafe {
            let mut unqlite = Unqlite::default();
            let filename = try!(CString::new(filename));
            error_or!(ffi::unqlite_open(&mut unqlite.db, filename.as_ptr(), mode.into()),
                      unqlite)
        }
    }
    pub fn kv_store_nonchecked(&mut self, key: &[u8], value: &[u8]) -> i32 {
        unsafe {
            ffi::unqlite_kv_store(self.db,
                                  key.as_ptr() as *const c_void,
                                  key.len() as i32,
                                  value.as_ptr() as *const c_void,
                                  value.len() as i64)
        }
    }

    pub fn kv_store(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        error_or!(self.kv_store_nonchecked(key, value), ())
    }

    pub fn kv_append_nonchecked(&mut self, key: &[u8], value: &[u8]) -> i32 {
        unsafe {
            ffi::unqlite_kv_append(self.db,
                                   key.as_ptr() as *const c_void,
                                   key.len() as i32,
                                   value.as_ptr() as *const c_void,
                                   value.len() as i64)
        }
    }
    pub fn kv_append(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        error_or!(self.kv_append_nonchecked(key, value), ())
    }
    pub fn kv_delete_nonchecked(&mut self, key: &[u8]) -> i32 {
        unsafe { ffi::unqlite_kv_delete(self.db, key.as_ptr() as *const c_void, key.len() as i32) }
    }
    pub fn kv_delete(&mut self, key: &[u8]) -> Result<()> {
        error_or!(self.kv_delete_nonchecked(key), ())
    }

    pub fn kv_contains(&self, key: &[u8]) -> bool {
        self.kv_fetch_length(key).map(|_x| true).unwrap_or(false)
    }
    pub fn kv_fetch_length(&self, key: &[u8]) -> Result<usize> {
        let mut len = 0usize;
        let raw_mut = &mut len as *mut usize;
        unsafe {
            error_or!(self.kv_fetch_nonchecked(key, std::ptr::null_mut(), raw_mut),
                      *raw_mut)
        }
    }
    pub fn kv_fetch_nonchecked(&self, key: &[u8], buf: *mut c_void, buf_len: *mut usize) -> i32 {
        unsafe {
            ffi::unqlite_kv_fetch(self.db,
                                  key.as_ptr() as *const c_void,
                                  key.len() as i32,
                                  buf,
                                  buf_len as *mut i64)
        }
    }
    pub fn kv_fetch(&self, key: &[u8], buf: &mut [u8]) -> Result<()> {
        let mut len = buf.len();
        error_or!(self.kv_fetch_nonchecked(key, buf.as_ptr() as *mut c_void, &mut len),
                  ())
    }

    fn close(&mut self) -> Result<()> {
        unsafe { error_or!(ffi::unqlite_close(self.db), ()) }
    }
}

impl Drop for Unqlite {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kv() {
        let filename = "test.db";
        {
            let mut rc = Unqlite::open(filename, UNQLITE_OPEN_CREATE).unwrap();
            println!("kv store");
            rc.kv_store(b"msg", b"Hello, ").unwrap();
            let len = rc.kv_fetch_length(b"msg").unwrap();
            assert_eq!(len, 7);
            assert_eq!(rc.kv_contains(b"msg"), true);
            assert_eq!(rc.kv_contains(b"msg2"), false);
            println!("kv append");
            rc.kv_append(b"msg", b"world!").unwrap();
            println!("kv fetch value length");
            let len = rc.kv_fetch_length(b"msg").unwrap();
            println!("fetched length {}", len);
            assert_eq!(len, 13);
            let mut vec: Vec<u8> = Vec::with_capacity(len);
            unsafe { vec.set_len(len) }
            rc.kv_fetch(b"msg", &mut *vec).unwrap();
            assert_eq!(String::from_utf8(vec).unwrap(),
                       String::from("Hello, world!"));
        }
        ::std::fs::remove_file(filename).unwrap();
    }
}
