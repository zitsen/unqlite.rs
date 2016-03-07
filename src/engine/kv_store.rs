use super::UnQlite;
use libc::c_void;
use std::mem;
use std::ptr;
use ffi::{unqlite_kv_append,
          unqlite_kv_config,
          unqlite_kv_delete,
          // unqlite_kv_store_fmt,
          // unqlite_kv_append_fmt,
          unqlite_kv_fetch,
          unqlite_kv_fetch_callback,
          unqlite_kv_store};
use ffi::constants::{UNQLITE_KV_CONFIG_CMP_FUNC, UNQLITE_KV_CONFIG_HASH_FUNC};

/// Key-Value Store Interface
impl<'kv_store> UnQlite {
    /// Store records in the database.
    ///
    /// Write a new record into the database. If the record does not exists, it is created.
    /// Otherwise, it is replaced. That is, the new data overwrite the old data. You can switch to
    /// `kv_append()` for an append operation.
    pub fn kv_store(&mut self, key: &[u8], value: &[u8]) -> ::Result<()> {
        error_or!(unsafe {
            unqlite_kv_store(self.db,
                             key.as_ptr() as *const c_void,
                             key.len() as i32,
                             value.as_ptr() as *const c_void,
                             value.len() as i64)
        })
    }

    /// Append data to a database record.
    ///
    /// Write a new record into the database. If the record does not exists, it is created.
    /// Otherwise, the new data chunk is appended to the end of the old chunk. You can switch to
    /// `kv_store()` for an overwrite operation.
    pub fn kv_append(&mut self, key: &[u8], value: &[u8]) -> ::Result<()> {
        error_or!(unsafe {
            unqlite_kv_append(self.db,
                              key.as_ptr() as *const c_void,
                              key.len() as i32,
                              value.as_ptr() as *const c_void,
                              value.len() as i64)
        })
    }

    /// Remove a record from the database.
    ///
    /// To remove a particular record from the database, you can use this high-level thread-safe
    /// routine to perform the deletion. You can also delete records using cursors via
    /// unqlite_kv_cursor_delete_entry().
    pub fn kv_delete(&mut self, key: &[u8]) -> ::Result<()> {
        error_or!(unsafe {
            unqlite_kv_delete(self.db, key.as_ptr() as *const c_void, key.len() as i32)
        })
    }

    /// Check if `key` is contained in database.
    pub fn kv_contains(&self, key: &[u8]) -> bool {
        self.kv_fetch_length(key).map(|_x| true).unwrap_or(false)
    }

    /// Fetch a record from the database and returns the length only
    pub fn kv_fetch_length(&self, key: &[u8]) -> ::Result<isize> {
        let len: *mut isize = unsafe { mem::uninitialized() };
        error_or!(unsafe {
            unqlite_kv_fetch(self.db,
                             key.as_ptr() as *const c_void,
                             key.len() as i32,
                             ptr::null_mut(),
                             len as *mut i64)
        })
            .map(|_| unsafe { *len })
    }

    /// Fetch a record from the database.
    ///
    /// Fetch a record from the database and copy its content to a `Vec<u8>`.
    ///
    /// The recommended interface for extracting very large data from the database is
    /// kv_fetch_callback() where the user simply need to supply a consumer callback
    /// instead of a buffer which may be unacceptable when dealing with very large records.
    pub fn kv_fetch(&self, key: &[u8]) -> ::Result<Vec<u8>> {
        let len = try!(self.kv_fetch_length(key));
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
        error_or!(unsafe {
                      unqlite_kv_fetch(self.db,
                                       key.as_ptr() as *const c_void,
                                       key.len() as i32,
                                       buf.as_mut_ptr() as *mut c_void,
                                       len as *mut i64)
                  },
                  buf)
    }

    /// Fetch a record from the database and invoke the supplied callback to consume its data.
    pub fn kv_fetch_callback(&self,
                             key: &[u8],
                             consumer: extern "C" fn(data: *const c_void,
                                                     len: u32,
                                                     user_data: *mut c_void)
                                                     -> i32)
                             -> ::Result<()> {
        error_or!(unsafe {
            unqlite_kv_fetch_callback(self.db,
                                      key.as_ptr() as *const c_void,
                                      key.len() as i32,
                                      Some(consumer),
                                      ptr::null_mut())
        })
    }

    /// Configure the hash function of the underlying Key/Value (KV) storage engine.
    ///
    /// Specify a hash function to be used instead of the built-in hash function. This option
    /// accepts a single argument which is a pointer to the client hash function.
    /// Note that the built-in hash function (DJB) is recommended for most purposes.
    pub fn kv_config_hash(&mut self,
                          hash: extern "C" fn(key: *const c_void, len: u32) -> u32)
                          -> ::Result<()> {
        error_or!(unsafe { unqlite_kv_config(self.db, UNQLITE_KV_CONFIG_HASH_FUNC, hash) })
    }

    /// Configure the compare function of the underlying Key/Value (KV) storage engine.
    ///
    /// Specify a comparison function to be used instead of the built-in comparison function. This
    /// option accepts a single argument which is a pointer to the client comparison function.
    /// Note that the built-in comparison function (Tuned memcmp() implementation) is recommended
    /// for most purposes.
    pub fn kv_config_cmp(&mut self,
                         hash: extern "C" fn(key: *const c_void, len: u32) -> u32)
                         -> ::Result<()> {
        error_or!(unsafe { unqlite_kv_config(self.db, UNQLITE_KV_CONFIG_CMP_FUNC, hash) })
    }
}
