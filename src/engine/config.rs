use super::UnQlite;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::convert::From;
use ffi::unqlite_config;
use ffi::constants::{UNQLITE_CONFIG_DISABLE_AUTO_COMMIT, UNQLITE_CONFIG_ERR_LOG,
                     UNQLITE_CONFIG_GET_KV_NAME, UNQLITE_CONFIG_JX9_ERR_LOG,
                     UNQLITE_CONFIG_KV_ENGINE, UNQLITE_CONFIG_MAX_PAGE_CACHE};

/// This part of functions is about UnQlite's config options.
///
/// The list is token from `unqlite_config` function, see also [here]
/// (http://unqlite.org/c_api/unqlite_config.html).
///
/// # Usage
///
/// ```ignore
/// let unqlite = UnQlite::create("test.db")
///     .max_page_cache(u32)
///     .disable_auto_commit()
///     .kv_engine("lsm");
///
/// println!("KV engine name: {}", unqlite.kv_name());
/// ```
impl<'config> UnQlite {
    /// Maximum raw pages to cache in memory.
    ///
    /// This is a simple hint, UnQLite is not forced to honor it.
    pub fn max_page_cache(self, max: u32) -> Self {
        error_or!(unsafe { unqlite_config(self.db, UNQLITE_CONFIG_MAX_PAGE_CACHE, max) }).unwrap();
        self
    }

    /// To diable automatically commit action.
    ///
    /// >
    /// Normally, If `unqlite_close()` is invoked while a transaction is open, the transaction is
    /// automatically committed. But, if this option is set, then the transaction is automatically
    /// rolled back and you should call `unqlite_commit()` manually to commit all database changes.
    ///
    pub fn disable_auto_commit(self) -> Self {
        error_or!(unsafe { unqlite_config(self.db, UNQLITE_CONFIG_DISABLE_AUTO_COMMIT) }).unwrap();
        self
    }

    /// Switch to another Key/Value storage engine.
    ///
    /// *This option is reserved for future usage.*
    pub fn kv_engine(self, name: CString) -> Self {
        error_or!(unsafe { unqlite_config(self.db, UNQLITE_CONFIG_KV_ENGINE, name.into_raw()) })
            .unwrap();
        self
    }

    /// The database error log is stored in an internal buffer. When something goes wrong during a
    /// commit, rollback, store, append operation, a human-readable error message is generated to
    /// help clients diagnostic the problem. This option can be used to point to that buffer.
    pub fn err_log(&self) -> Option<String> {
        unsafe {
            let log: *mut ::libc::c_char = mem::uninitialized();
            let len: i32 = mem::uninitialized();
            error_or!(unqlite_config(self.db, UNQLITE_CONFIG_ERR_LOG, &log, &len)).unwrap();
            if len > 0 {
                Some(from_chars_to_string(log))
            } else {
                None
            }
        }
    }

    /// When something goes wrong during compilation of the target Jx9 script due to an erroneous
    /// Jx9 code, the compiler error log is redirected to an internal buffer. This option can be
    /// used to point to that buffer.
    pub fn jx9_err_log(&self) -> Option<String> {
        unsafe {
            let log: *mut ::libc::c_char = mem::uninitialized();
            let len: i32 = mem::uninitialized();
            error_or!(unqlite_config(self.db, UNQLITE_CONFIG_JX9_ERR_LOG, &log, &len)).unwrap();
            if len > 0 {
                Some(from_chars_to_string(log))
            } else {
                None
            }
        }
    }
    /// Extract the name of the underlying Key/Value storage engine.
    ///
    /// Here's some useful names to know: Hash, Mem, R+Tree, LSM, etc.
    pub fn kv_name(&self) -> String {
        unsafe {
            let kv_name: *mut ::libc::c_char = mem::uninitialized();
            error_or!(unqlite_config(self.db, UNQLITE_CONFIG_GET_KV_NAME, &kv_name)).unwrap();
            from_chars_to_string(kv_name)
        }
    }
}

fn from_chars_to_cstring(p: *mut ::libc::c_char) -> CString {
    unsafe {
        let len = ::libc::strlen(p);
        let (_, vec) = (0..len).fold((p, Vec::new()), |(p, mut vec), _| {
            let u: u8 = ptr::read(p) as u8;
            vec.push(u);
            let p = p.offset(1);
            (p, vec)
        });
        CString::from_vec_unchecked(vec)
    }
}

fn from_chars_to_string(p: *mut ::libc::c_char) -> String {
   from_chars_to_cstring(p).into_string().unwrap()
}


#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use super::super::UnQlite;

    #[test]
    fn test_config() {
        let unqlite = UnQlite::create_in_memory().max_page_cache(512000000);
        let kv_name = unqlite.kv_name();
        assert_eq!(kv_name, String::from("mem"));
        assert_eq!(unqlite.err_log(), None);
        let unqlite = UnQlite::create("/root/test.db");
        assert_eq!(unqlite.err_log(), None);
    }
}
