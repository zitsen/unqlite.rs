use UnQlite;
use error::Wrap;
use std::ffi::CString;
use std::ptr;
use std::mem;
use ffi::unqlite_config;
use ffi::constants::{UNQLITE_CONFIG_DISABLE_AUTO_COMMIT, UNQLITE_CONFIG_ERR_LOG,
                     UNQLITE_CONFIG_GET_KV_NAME, UNQLITE_CONFIG_JX9_ERR_LOG,
                     UNQLITE_CONFIG_KV_ENGINE, UNQLITE_CONFIG_MAX_PAGE_CACHE};

/// A `Trait` for configuration.
///
/// This part of functions is about UnQlite's config options.
///
/// The list is token from `unqlite_config` function, see also [here]
/// (http://unqlite.org/c_api/unqlite_config.html).
///
/// # Usage
///
/// ```
/// # extern crate unqlite;
/// #
/// # use unqlite::{UnQlite, Config};
/// #
/// # #[cfg(feature = "enable-threads")]
/// # fn main() {
/// let unqlite = UnQlite::create_temp()
///     .max_page_cache(4096u32)
///     .disable_auto_commit();
/// println!("KV engine name: {}", unqlite.kv_name());
///
/// // Extract error logs
/// unqlite.err_log().map(|log| panic!(log));
///
/// # }
/// # #[cfg(not(feature = "enable-threads"))]
/// # fn main() { }
/// ```
pub trait Config {
    /// Maximum raw pages to cache in memory.
    ///
    /// This is a simple hint, UnQLite is not forced to honor it.
    fn max_page_cache(self, max: u32) -> Self;

    /// To diable automatically commit action.
    ///
    /// >
    /// Normally, If `unqlite_close()` is invoked while a transaction is open, the transaction is
    /// automatically committed. But, if this option is set, then the transaction is automatically
    /// rolled back and you should call `unqlite_commit()` manually to commit all database changes.
    ///
    fn disable_auto_commit(self) -> Self;

    /// Switch to another Key/Value storage engine.
    ///
    /// Offical document says: *This option is reserved for future usage.*
    ///
    /// There is some unknown bug in setting this, **DO NOT** use this option currently.
    fn kv_engine<S: Into<Vec<u8>>>(self, name: S) -> Self;

    /// The database error log is stored in an internal buffer. When something goes wrong during a
    /// commit, rollback, store, append operation, a human-readable error message is generated to
    /// help clients diagnostic the problem. This option can be used to point to that buffer.
    fn err_log(&self) -> Option<String>;

    /// When something goes wrong during compilation of the target Jx9 script due to an erroneous
    /// Jx9 code, the compiler error log is redirected to an internal buffer. This option can be
    /// used to point to that buffer.
    fn jx9_err_log(&self) -> Option<String>;

    /// Extract the name of the underlying Key/Value storage engine.
    ///
    /// Here's some useful names to know: Hash, Mem, R+Tree, LSM, etc.
    fn kv_name(&self) -> String;
}

impl Config for UnQlite {
    fn max_page_cache(self, max: u32) -> Self {
        wrap_raw!(self, config, UNQLITE_CONFIG_MAX_PAGE_CACHE, max)
            .expect("set max page cache error");
        self
    }

    fn disable_auto_commit(self) -> Self {
        wrap_raw!(self, config, UNQLITE_CONFIG_DISABLE_AUTO_COMMIT).expect("disable auto commit");
        self
    }

    fn kv_engine<S: Into<Vec<u8>>>(self, name: S) -> Self {
        wrap_raw!(self,
                  config,
                  UNQLITE_CONFIG_KV_ENGINE,
                  CString::new(name).expect("KV engine error").into_raw())
            .expect("config KV engine");
        self
    }

    fn err_log(&self) -> Option<String> {
        let log: *mut ::libc::c_char = unsafe { mem::uninitialized() };
        let len: i32 = unsafe { mem::uninitialized() };

        wrap_raw!(self, config, UNQLITE_CONFIG_ERR_LOG, &log, &len)
            .ok()
            .and_then(|_| {
                if len > 0 {
                    Some(from_chars_to_string(log))
                } else {
                    None
                }
            })
    }

    fn jx9_err_log(&self) -> Option<String> {
        let log: *mut ::libc::c_char = unsafe { mem::uninitialized() };
        let len: i32 = unsafe { mem::uninitialized() };
        wrap_raw!(self, config, UNQLITE_CONFIG_JX9_ERR_LOG, &log, &len)
            .ok()
            .and_then(|_| {
                if len > 0 {
                    Some(from_chars_to_string(log))
                } else {
                    None
                }
            })
    }

    fn kv_name(&self) -> String {
        let kv_name: *mut ::libc::c_char = unsafe { mem::uninitialized() };

        wrap_raw!(self, config, UNQLITE_CONFIG_GET_KV_NAME, &kv_name).unwrap();
        from_chars_to_string(kv_name)
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
    use UnQlite;
    use super::Config;
    #[test]
    fn max_page_cache() {
        let unqlite = UnQlite::create_temp().max_page_cache(512000000);
        let kv_name = unqlite.kv_name();
        assert_eq!(kv_name, String::from("mem"));
        assert_eq!(unqlite.err_log(), None);
    }
    #[test]
    fn disable_auto_commit() {
        let unqlite = UnQlite::create_temp()
                          .max_page_cache(4096u32)
                          .disable_auto_commit();
    }
    #[test]
    #[should_panic]
    fn kv_engine_panic() {
        let _ = UnQlite::create_temp().kv_engine("hash");
    }
}
