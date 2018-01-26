/* automatically generated by rust-bindgen */

pub const UNQLITE_VERSION: &'static [u8; 6usize] = b"1.1.8\0";
pub const UNQLITE_VERSION_NUMBER: ::std::os::raw::c_int = 1001008;
pub const UNQLITE_SIG: &'static [u8; 14usize] = b"unqlite/1.1.8\0";
pub const UNQLITE_IDENT: &'static [u8; 49usize] =
    b"unqlite:b172a1e2c3f62fb35c8e1fb2795121f82356cad6\0";
pub const UNQLITE_COPYRIGHT : & 'static [ u8 ; 108usize ] = b"Copyright (C) Symisc Systems, S.U.A.R.L [Mrad Chems Eddine <chm@symisc.net>] 2012-2018, http://unqlite.org/\0" ;
pub const SXRET_OK: ::std::os::raw::c_int = 0;
pub const SXERR_MEM: ::std::os::raw::c_int = -1;
pub const SXERR_IO: ::std::os::raw::c_int = -2;
pub const SXERR_EMPTY: ::std::os::raw::c_int = -3;
pub const SXERR_LOCKED: ::std::os::raw::c_int = -4;
pub const SXERR_ORANGE: ::std::os::raw::c_int = -5;
pub const SXERR_NOTFOUND: ::std::os::raw::c_int = -6;
pub const SXERR_LIMIT: ::std::os::raw::c_int = -7;
pub const SXERR_MORE: ::std::os::raw::c_int = -8;
pub const SXERR_INVALID: ::std::os::raw::c_int = -9;
pub const SXERR_ABORT: ::std::os::raw::c_int = -10;
pub const SXERR_EXISTS: ::std::os::raw::c_int = -11;
pub const SXERR_SYNTAX: ::std::os::raw::c_int = -12;
pub const SXERR_UNKNOWN: ::std::os::raw::c_int = -13;
pub const SXERR_BUSY: ::std::os::raw::c_int = -14;
pub const SXERR_OVERFLOW: ::std::os::raw::c_int = -15;
pub const SXERR_WILLBLOCK: ::std::os::raw::c_int = -16;
pub const SXERR_NOTIMPLEMENTED: ::std::os::raw::c_int = -17;
pub const SXERR_EOF: ::std::os::raw::c_int = -18;
pub const SXERR_PERM: ::std::os::raw::c_int = -19;
pub const SXERR_NOOP: ::std::os::raw::c_int = -20;
pub const SXERR_FORMAT: ::std::os::raw::c_int = -21;
pub const SXERR_NEXT: ::std::os::raw::c_int = -22;
pub const SXERR_OS: ::std::os::raw::c_int = -23;
pub const SXERR_CORRUPT: ::std::os::raw::c_int = -24;
pub const SXERR_CONTINUE: ::std::os::raw::c_int = -25;
pub const SXERR_NOMATCH: ::std::os::raw::c_int = -26;
pub const SXERR_RESET: ::std::os::raw::c_int = -27;
pub const SXERR_DONE: ::std::os::raw::c_int = -28;
pub const SXERR_SHORT: ::std::os::raw::c_int = -29;
pub const SXERR_PATH: ::std::os::raw::c_int = -30;
pub const SXERR_TIMEOUT: ::std::os::raw::c_int = -31;
pub const SXERR_BIG: ::std::os::raw::c_int = -32;
pub const SXERR_RETRY: ::std::os::raw::c_int = -33;
pub const SXERR_IGNORE: ::std::os::raw::c_int = -63;
pub const UNQLITE_OK: ::std::os::raw::c_int = 0;
pub const UNQLITE_NOMEM: ::std::os::raw::c_int = -1;
pub const UNQLITE_ABORT: ::std::os::raw::c_int = -10;
pub const UNQLITE_IOERR: ::std::os::raw::c_int = -2;
pub const UNQLITE_CORRUPT: ::std::os::raw::c_int = -24;
pub const UNQLITE_LOCKED: ::std::os::raw::c_int = -4;
pub const UNQLITE_BUSY: ::std::os::raw::c_int = -14;
pub const UNQLITE_DONE: ::std::os::raw::c_int = -28;
pub const UNQLITE_PERM: ::std::os::raw::c_int = -19;
pub const UNQLITE_NOTIMPLEMENTED: ::std::os::raw::c_int = -17;
pub const UNQLITE_NOTFOUND: ::std::os::raw::c_int = -6;
pub const UNQLITE_NOOP: ::std::os::raw::c_int = -20;
pub const UNQLITE_INVALID: ::std::os::raw::c_int = -9;
pub const UNQLITE_EOF: ::std::os::raw::c_int = -18;
pub const UNQLITE_UNKNOWN: ::std::os::raw::c_int = -13;
pub const UNQLITE_LIMIT: ::std::os::raw::c_int = -7;
pub const UNQLITE_EXISTS: ::std::os::raw::c_int = -11;
pub const UNQLITE_EMPTY: ::std::os::raw::c_int = -3;
pub const UNQLITE_COMPILE_ERR: ::std::os::raw::c_int = -70;
pub const UNQLITE_VM_ERR: ::std::os::raw::c_int = -71;
pub const UNQLITE_FULL: ::std::os::raw::c_int = -73;
pub const UNQLITE_CANTOPEN: ::std::os::raw::c_int = -74;
pub const UNQLITE_READ_ONLY: ::std::os::raw::c_int = -75;
pub const UNQLITE_LOCKERR: ::std::os::raw::c_int = -76;
pub const UNQLITE_CONFIG_JX9_ERR_LOG: ::std::os::raw::c_int = 1;
pub const UNQLITE_CONFIG_MAX_PAGE_CACHE: ::std::os::raw::c_int = 2;
pub const UNQLITE_CONFIG_ERR_LOG: ::std::os::raw::c_int = 3;
pub const UNQLITE_CONFIG_KV_ENGINE: ::std::os::raw::c_int = 4;
pub const UNQLITE_CONFIG_DISABLE_AUTO_COMMIT: ::std::os::raw::c_int = 5;
pub const UNQLITE_CONFIG_GET_KV_NAME: ::std::os::raw::c_int = 6;
pub const UNQLITE_VM_CONFIG_OUTPUT: ::std::os::raw::c_int = 1;
pub const UNQLITE_VM_CONFIG_IMPORT_PATH: ::std::os::raw::c_int = 2;
pub const UNQLITE_VM_CONFIG_ERR_REPORT: ::std::os::raw::c_int = 3;
pub const UNQLITE_VM_CONFIG_RECURSION_DEPTH: ::std::os::raw::c_int = 4;
pub const UNQLITE_VM_OUTPUT_LENGTH: ::std::os::raw::c_int = 5;
pub const UNQLITE_VM_CONFIG_CREATE_VAR: ::std::os::raw::c_int = 6;
pub const UNQLITE_VM_CONFIG_HTTP_REQUEST: ::std::os::raw::c_int = 7;
pub const UNQLITE_VM_CONFIG_SERVER_ATTR: ::std::os::raw::c_int = 8;
pub const UNQLITE_VM_CONFIG_ENV_ATTR: ::std::os::raw::c_int = 9;
pub const UNQLITE_VM_CONFIG_EXEC_VALUE: ::std::os::raw::c_int = 10;
pub const UNQLITE_VM_CONFIG_IO_STREAM: ::std::os::raw::c_int = 11;
pub const UNQLITE_VM_CONFIG_ARGV_ENTRY: ::std::os::raw::c_int = 12;
pub const UNQLITE_VM_CONFIG_EXTRACT_OUTPUT: ::std::os::raw::c_int = 13;
pub const UNQLITE_KV_CONFIG_HASH_FUNC: ::std::os::raw::c_int = 1;
pub const UNQLITE_KV_CONFIG_CMP_FUNC: ::std::os::raw::c_int = 2;
pub const UNQLITE_LIB_CONFIG_USER_MALLOC: ::std::os::raw::c_int = 1;
pub const UNQLITE_LIB_CONFIG_MEM_ERR_CALLBACK: ::std::os::raw::c_int = 2;
pub const UNQLITE_LIB_CONFIG_USER_MUTEX: ::std::os::raw::c_int = 3;
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE: ::std::os::raw::c_int = 4;
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI: ::std::os::raw::c_int = 5;
pub const UNQLITE_LIB_CONFIG_VFS: ::std::os::raw::c_int = 6;
pub const UNQLITE_LIB_CONFIG_STORAGE_ENGINE: ::std::os::raw::c_int = 7;
pub const UNQLITE_LIB_CONFIG_PAGE_SIZE: ::std::os::raw::c_int = 8;
pub const UNQLITE_OPEN_READONLY: ::std::os::raw::c_uint = 1;
pub const UNQLITE_OPEN_READWRITE: ::std::os::raw::c_uint = 2;
pub const UNQLITE_OPEN_CREATE: ::std::os::raw::c_uint = 4;
pub const UNQLITE_OPEN_EXCLUSIVE: ::std::os::raw::c_uint = 8;
pub const UNQLITE_OPEN_TEMP_DB: ::std::os::raw::c_uint = 16;
pub const UNQLITE_OPEN_NOMUTEX: ::std::os::raw::c_uint = 32;
pub const UNQLITE_OPEN_OMIT_JOURNALING: ::std::os::raw::c_uint = 64;
pub const UNQLITE_OPEN_IN_MEMORY: ::std::os::raw::c_uint = 128;
pub const UNQLITE_OPEN_MMAP: ::std::os::raw::c_uint = 256;
pub const UNQLITE_SYNC_NORMAL: ::std::os::raw::c_uint = 2;
pub const UNQLITE_SYNC_FULL: ::std::os::raw::c_uint = 3;
pub const UNQLITE_SYNC_DATAONLY: ::std::os::raw::c_uint = 16;
pub const UNQLITE_LOCK_NONE: ::std::os::raw::c_uint = 0;
pub const UNQLITE_LOCK_SHARED: ::std::os::raw::c_uint = 1;
pub const UNQLITE_LOCK_RESERVED: ::std::os::raw::c_uint = 2;
pub const UNQLITE_LOCK_PENDING: ::std::os::raw::c_uint = 3;
pub const UNQLITE_LOCK_EXCLUSIVE: ::std::os::raw::c_uint = 4;
pub const UNQLITE_ACCESS_EXISTS: ::std::os::raw::c_uint = 0;
pub const UNQLITE_ACCESS_READWRITE: ::std::os::raw::c_uint = 1;
pub const UNQLITE_ACCESS_READ: ::std::os::raw::c_uint = 2;
pub const UNQLITE_CURSOR_MATCH_EXACT: ::std::os::raw::c_uint = 1;
pub const UNQLITE_CURSOR_MATCH_LE: ::std::os::raw::c_uint = 2;
pub const UNQLITE_CURSOR_MATCH_GE: ::std::os::raw::c_uint = 3;
pub const UNQLITE_JOURNAL_FILE_SUFFIX: &'static [u8; 17usize] = b"_unqlite_journal\0";
pub const UNQLITE_CTX_ERR: ::std::os::raw::c_uint = 1;
pub const UNQLITE_CTX_WARNING: ::std::os::raw::c_uint = 2;
pub const UNQLITE_CTX_NOTICE: ::std::os::raw::c_uint = 3;