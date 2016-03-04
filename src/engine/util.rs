use super::UnQlite;
use ffi::{unqlite_util_random_num, unqlite_util_random_string};

impl <'util> UnQlite {
    /// Generate random string using the UnQLite PRNG.
    ///
    /// It will generate a english alphabet based string of length buf_size (last argument).
    fn random_string(&self, buf_size: u32) -> Vec<u8> {
        unsafe {
            let mut vec: Vec<u8> = Vec::with_capacity(buf_size as usize);
            error_or!(unqlite_util_random_string(self.db, vec.as_mut_ptr() as *mut i8, buf_size)).unwrap();
            vec
        }
    }

    /// Generate random number using the UnQLite PRNG.
    ///
    /// It will return a 32-bit unsigned integer between 0 and 0xFFFFFFFF.
    fn random_num(&self) -> u32 {
        unsafe {
            unqlite_util_random_num(self.db)
        }
    }
}
