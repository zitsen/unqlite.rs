use error::{Result, Wrap};
use ffi::{
    unqlite_kv_append,
    unqlite_kv_config,
    unqlite_kv_delete,
    // unqlite_kv_store_fmt,
    // unqlite_kv_append_fmt,
    unqlite_kv_fetch,
    unqlite_kv_fetch_callback,
    unqlite_kv_store,
};
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use vars::{UNQLITE_KV_CONFIG_CMP_FUNC, UNQLITE_KV_CONFIG_HASH_FUNC};
use UnQLite;

/// Key-Value Store Interface
pub trait KV {
    /// Store records in the database.
    ///
    /// Write a new record into the database. If the record does not exists, it is created.
    /// Otherwise, it is replaced. That is, the new data overwrite the old data. You can switch to
    /// `kv_append()` for an append operation.
    fn kv_store<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<()>;

    /// Append data to a database record.
    ///
    /// Write a new record into the database. If the record does not exists, it is created.
    /// Otherwise, the new data chunk is appended to the end of the old chunk. You can switch to
    /// `kv_store()` for an overwrite operation.
    fn kv_append<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<()>;

    /// Remove a record from the database.
    ///
    /// To remove a particular record from the database, you can use this high-level thread-safe
    /// routine to perform the deletion. You can also delete records using cursors via
    /// unqlite_kv_cursor_delete_entry().
    fn kv_delete<K: AsRef<[u8]>>(&self, key: K) -> Result<()>;

    /// Check if `key` is contained in database.
    fn kv_contains<K: AsRef<[u8]>>(&self, key: K) -> bool;

    /// Fetch a record from the database and returns the length only
    fn kv_fetch_length<K: AsRef<[u8]>>(&self, key: K) -> Result<i64>;

    /// Fetch a record from the database.
    ///
    /// Fetch a record from the database and copy its content to a `Vec<u8>`.
    ///
    /// The recommended interface for extracting very large data from the database is
    /// kv_fetch_callback() where the user simply need to supply a consumer callback
    /// instead of a buffer which may be unacceptable when dealing with very large records.
    fn kv_fetch<K: AsRef<[u8]>>(&self, key: K) -> Result<Vec<u8>>;

    /// Fetch a record from the database and invoke the supplied callback to consume its data.
    fn kv_fetch_callback<K: AsRef<[u8]>>(
        &self,
        key: K,
        consumer: extern "C" fn(data: *const c_void, len: u32, user_data: *mut c_void) -> i32,
    ) -> Result<()>;

    /// Configure the hash function of the underlying Key/Value (KV) storage engine.
    ///
    /// Specify a hash function to be used instead of the built-in hash function. This option
    /// accepts a single argument which is a pointer to the client hash function.
    /// Note that the built-in hash function (DJB) is recommended for most purposes.
    fn kv_config_hash(
        &self,
        hash: extern "C" fn(key: *const c_void, len: u32) -> u32,
    ) -> Result<()>;

    /// Configure the compare function of the underlying Key/Value (KV) storage engine.
    ///
    /// Specify a comparison function to be used instead of the built-in comparison function. This
    /// option accepts a single argument which is a pointer to the client comparison function.
    /// Note that the built-in comparison function (Tuned memcmp() implementation) is recommended
    /// for most purposes.
    fn kv_config_cmp(&self, hash: extern "C" fn(key: *const c_void, len: u32) -> u32)
        -> Result<()>;
}

/// Key-Value Store Interface
impl KV for UnQLite {
    fn kv_store<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<()> {
        let key = key.as_ref();
        let value = value.as_ref();
        wrap!(
            kv_store,
            self.as_raw_mut_ptr(),
            key.as_ptr() as _,
            key.len() as _,
            value.as_ptr() as _,
            value.len() as _
        )
    }

    fn kv_append<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<()> {
        let key = key.as_ref();
        let value = value.as_ref();
        wrap!(
            kv_append,
            self.as_raw_mut_ptr(),
            key.as_ptr() as _,
            key.len() as _,
            value.as_ptr() as _,
            value.len() as _
        )
    }

    fn kv_delete<K: AsRef<[u8]>>(&self, key: K) -> Result<()> {
        wrap!(
            kv_delete,
            self.as_raw_mut_ptr(),
            key.as_ref().as_ptr() as _,
            key.as_ref().len() as _
        )
    }

    fn kv_contains<K: AsRef<[u8]>>(&self, key: K) -> bool {
        self.kv_fetch_length(key).map(|_x| true).unwrap_or(false)
    }

    fn kv_fetch_length<K: AsRef<[u8]>>(&self, key: K) -> Result<i64> {
        let key = key.as_ref();
        let mut len = 0i64;
        wrap!(
            kv_fetch,
            self.as_raw_mut_ptr(),
            key.as_ptr() as _,
            key.len() as _,
            ptr::null_mut(),
            &mut len
        )
        .map(|_| len)
    }

    fn kv_fetch<K: AsRef<[u8]>>(&self, key: K) -> Result<Vec<u8>> {
        let key = key.as_ref();
        let mut len = try!(self.kv_fetch_length(key));
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
        let cap = buf.capacity();
        let ptr = buf.as_mut_ptr();
        mem::forget(buf);
        wrap!(
            kv_fetch,
            self.as_raw_mut_ptr(),
            key.as_ptr() as _,
            key.len() as _,
            ptr as _,
            &mut len
        )
        .map(|_| unsafe { Vec::from_raw_parts(ptr, len as usize, cap) })
    }

    fn kv_fetch_callback<K: AsRef<[u8]>>(
        &self,
        key: K,
        consumer: extern "C" fn(data: *const c_void, len: u32, user_data: *mut c_void) -> i32,
    ) -> Result<()> {
        let key = key.as_ref();
        wrap!(
            kv_fetch_callback,
            self.as_raw_mut_ptr(),
            key.as_ptr() as _,
            key.len() as i32,
            Some(consumer),
            ptr::null_mut()
        )
    }

    fn kv_config_hash(
        &self,
        hash: extern "C" fn(key: *const c_void, len: u32) -> u32,
    ) -> Result<()> {
        wrap!(
            kv_config,
            self.as_raw_mut_ptr(),
            UNQLITE_KV_CONFIG_HASH_FUNC,
            hash
        )
    }

    fn kv_config_cmp(&self, cmp: extern "C" fn(key: *const c_void, len: u32) -> u32) -> Result<()> {
        wrap!(
            kv_config,
            self.as_raw_mut_ptr(),
            UNQLITE_KV_CONFIG_CMP_FUNC,
            cmp
        )
    }
}

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use super::KV;
    use UnQLite;

    #[test]
    fn test_kv_store() {
        let unqlite = UnQLite::create_temp();
        let _ = unqlite.kv_store("abc", "123").unwrap();
        let vec = [1u8, 2u8, 3u8];
        let _ = unqlite.kv_store(&vec, "123").unwrap();
        let _ = unqlite
            .kv_store(&vec![4, 5, 6], &String::from("哈哈"))
            .unwrap();

        let value = unqlite.kv_fetch_length("abc");
        assert!(value.is_ok());
        assert!(value.unwrap() == 3);
        assert!(unqlite.kv_contains(&vec![1, 2, 3]));

        let value = unqlite.kv_fetch(&vec![1, 2, 3]).unwrap();
        assert!(value.len() == 3);
        assert_eq!(value, [49, 50, 51]);

        let value = unqlite.kv_fetch(&vec![4, 5, 6]).unwrap();
        assert_eq!(
            unsafe { String::from_utf8_unchecked(value) },
            String::from("哈哈")
        );

        unqlite.kv_delete("abc").unwrap();
        assert!(!unqlite.kv_contains("abc"));

        unqlite.kv_append(&vec, "456").unwrap();
        assert!(unqlite.kv_fetch_length(&vec).unwrap() == 6);
    }

    #[test]
    #[should_panic]
    fn panic_kv_fetch_not_found() {
        let unqlite = UnQLite::create_in_memory();
        unqlite.kv_fetch(&vec![4, 5, 6]).unwrap();
    }
}
