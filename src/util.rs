use std::path::Path;
use std::ffi::CString;
use std::mem;
use libc::c_void;
use super::UnQlite;
use ffi::{unqlite_util_load_mmaped_file, unqlite_util_random_num, unqlite_util_random_string,
          unqlite_util_release_mmaped_file};

pub trait Util {
    fn random_string(&self, buf_size: u32) -> Vec<u8>;

    fn random_num(&self) -> u32;
}

impl Util for UnQlite {
    /// Generate random string using the UnQLite PRNG.
    ///
    /// It will generate a english alphabet based string of length buf_size (last argument).
    fn random_string(&self, buf_size: u32) -> Vec<u8> {
        unsafe {
            let mut vec: Vec<u8> = Vec::with_capacity(buf_size as usize);
            error_or!(unqlite_util_random_string(self.as_raw_mut_ptr(),
                                                 vec.as_mut_ptr() as *mut i8,
                                                 buf_size))
                .unwrap();
            vec
        }
    }

    /// Generate random number using the UnQLite PRNG.
    ///
    /// It will return a 32-bit unsigned integer between 0 and 0xFFFFFFFF.
    fn random_num(&self) -> u32 {
        unsafe { unqlite_util_random_num(self.as_raw_mut_ptr()) }
    }
}
/// Memory-mapped file
pub fn load_mmaped_file<P: AsRef<Path>>(path: P) -> ::Result<Mmap> {
    unsafe {
        let path = path.as_ref();
        let mut ptr: *mut c_void = mem::uninitialized();
        let mut size: i64 = mem::uninitialized();
        let cpath = try!(CString::new(path.to_str().expect("cannot convert the path to str")));
        error_or!(unqlite_util_load_mmaped_file(cpath.as_ptr(), &mut ptr, &mut size)).map(|_| {
            Mmap {
                ptr: ptr,
                size: size,
            }
        })
    }
}

pub struct Mmap {
    pub ptr: *mut c_void,
    pub size: i64,
}

impl Drop for Mmap {
    fn drop(&mut self) {
        unsafe {
            error_or!(unqlite_util_release_mmaped_file(self.ptr, self.size)).unwrap();
        }
    }
}

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use super::*;
    use super::super::UnQlite;
    use tempfile::NamedTempFile;

    #[test]
    fn test_random_string() {
        let unqlite = UnQlite::create_in_memory();
        let _ = unqlite.random_string(32);
    }

    #[test]
    fn test_random_num() {
        let _ = UnQlite::create_in_memory().random_num();
    }

    #[test]
    fn test_mmap() {
        let tempfile = NamedTempFile::new().unwrap();
        load_mmaped_file(tempfile.path()).unwrap();
    }
}
