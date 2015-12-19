/// The UNQLITE_VERSION C preprocessor macroevaluates to a string literal
/// that is the unqlite version in the format "X.Y.Z" where X is the major
/// version number and Y is the minor version number and Z is the release
/// number.
///
/// ```c
/// #define UNQLITE_VERSION "1.1.6"
/// ```
pub const UNQLITE_VERSION: &'static str = "1.1.6";

/// ```c
/// #define UNQLITE_VERSION_NUMBER 1001006
/// ```
pub const UNQLITE_VERSION_NUMBER: i32 = 1001006;

/// The UNQLITE_SIG C preprocessor macro evaluates to a string
/// literal which is the public signature of the unqlite engine.
/// This signature could be included for example in a host-application
/// generated Server MIME header as follows:
///
/// ```mime
///    Server: YourWebServer/x.x unqlite/x.x.x \r\n
/// ```
pub const UNQLITE_SIG: &'static str = "unqlite/1.1.6";

// UnQLite identification in the Symisc source tree:
// Each particular check-in of a particular software released
// by symisc systems have an unique identifier associated with it.
// This macro hold the one associated with unqlite.
//
/// ```c
/// #define UNQLITE_IDENT "unqlite:b172a1e2c3f62fb35c8e1fb2795121f82356cad6"
/// ```
pub const UNQLITE_IDENT: &'static str = "unqlite:b172a1e2c3f62fb35c8e1fb2795121f82356cad6";
// Copyright notice.
// If you have any questions about the licensing situation, please
// visit http://unqlite.org/licensing.html
// or contact Symisc Systems via:
//   legal@symisc.net
//   licensing@symisc.net
//   contact@symisc.net
//
/// ```c
/// #define UNQLITE_COPYRIGHT "Copyright (C) Symisc Systems, S.U.A.R.L [Mrad Chems Eddine <chm@symisc.net>] 2012-2013, http://unqlite.org/"
/// ```
pub const UNQLITE_COPYRIGHT: &'static str = "Copyright (C) Symisc Systems, S.U.A.R.L [Mrad Chems Eddine <chm@symisc.net>] 2012-2013, http://unqlite.org/";

// Standard return values from Symisc public interfaces
pub const SXRET_OK: i32 = 0;      /* Not an error */
pub const SXERR_MEM: i32 = (-1); /* Out of memory */
pub const SXERR_IO: i32 = (-2); /* IO error */
pub const SXERR_EMPTY: i32 = (-3); /* Empty field */
pub const SXERR_LOCKED: i32 = (-4); /* Locked operation */
pub const SXERR_ORANGE: i32 = (-5); /* Out of range value */
pub const SXERR_NOTFOUND: i32 = (-6); /* Item not found */
pub const SXERR_LIMIT: i32 = (-7); /* Limit reached */
pub const SXERR_MORE: i32 = (-8); /* Need more input */
pub const SXERR_INVALID: i32 = (-9); /* Invalid parameter */
pub const SXERR_ABORT: i32 = (-10); /* User callback request an operation abort */
pub const SXERR_EXISTS: i32 = (-11); /* Item exists */
pub const SXERR_SYNTAX: i32 = (-12); /* Syntax error */
pub const SXERR_UNKNOWN: i32 = (-13); /* Unknown error */
pub const SXERR_BUSY: i32 = (-14); /* Busy operation */
pub const SXERR_OVERFLOW: i32 = (-15); /* Stack or buffer overflow */
pub const SXERR_WILLBLOCK: i32 = (-16); /* Operation will block */
pub const SXERR_NOTIMPLEMENTED: i32 = (-17); /* Operation not implemented */
pub const SXERR_EOF: i32 = (-18); /* End of input */
pub const SXERR_PERM: i32 = (-19); /* Permission error */
pub const SXERR_NOOP: i32 = (-20); /* No-op */
pub const SXERR_FORMAT: i32 = (-21); /* Invalid format */
pub const SXERR_NEXT: i32 = (-22); /* Not an error */
pub const SXERR_OS: i32 = (-23); /* System call return an error */
pub const SXERR_CORRUPT: i32 = (-24); /* Corrupted pointer */
pub const SXERR_CONTINUE: i32 = (-25); /* Not an error: Operation in progress */
pub const SXERR_NOMATCH: i32 = (-26); /* No match */
pub const SXERR_RESET: i32 = (-27); /* Operation reset */
pub const SXERR_DONE: i32 = (-28); /* Not an error */
pub const SXERR_SHORT: i32 = (-29); /* Buffer too short */
pub const SXERR_PATH: i32 = (-30); /* Path error */
pub const SXERR_TIMEOUT: i32 = (-31); /* Timeout */
pub const SXERR_BIG: i32 = (-32); /* Too big for processing */
pub const SXERR_RETRY: i32 = (-33); /* Retry your call */
pub const SXERR_IGNORE: i32 = (-63); /* Ignore */
// Standard UnQLite return values
pub const UNQLITE_OK: i32 = SXRET_OK; /* Successful result */
// Beginning of error codes
pub const UNQLITE_NOMEM: i32 = SXERR_MEM; /* Out of memory */
pub const UNQLITE_ABORT: i32 = SXERR_ABORT; /* Another thread have released this instance */
pub const UNQLITE_IOERR: i32 = SXERR_IO; /* IO error */
pub const UNQLITE_CORRUPT: i32 = SXERR_CORRUPT; /* Corrupt pointer */
pub const UNQLITE_LOCKED: i32 = SXERR_LOCKED; /* Forbidden Operation */
pub const UNQLITE_BUSY: i32 = SXERR_BUSY; /* The database file is locked */
pub const UNQLITE_DONE: i32 = SXERR_DONE; /* Operation done */
pub const UNQLITE_PERM: i32 = SXERR_PERM; /* Permission error */
pub const UNQLITE_NOTIMPLEMENTED: i32 = SXERR_NOTIMPLEMENTED; /* Method not implemented by the underlying Key/Value storage engine */
pub const UNQLITE_NOTFOUND: i32 = SXERR_NOTFOUND; /* No such record */
pub const UNQLITE_NOOP: i32 = SXERR_NOOP; /* No such method */
pub const UNQLITE_INVALID: i32 = SXERR_INVALID; /* Invalid parameter */
pub const UNQLITE_EOF: i32 = SXERR_EOF; /* End Of Input */
pub const UNQLITE_UNKNOWN: i32 = SXERR_UNKNOWN; /* Unknown configuration option */
pub const UNQLITE_LIMIT: i32 = SXERR_LIMIT; /* Database limit reached */
pub const UNQLITE_EXISTS: i32 = SXERR_EXISTS; /* Record exists */
pub const UNQLITE_EMPTY: i32 = SXERR_EMPTY; /* Empty record */
pub const UNQLITE_COMPILE_ERR: i32 = -70; /* Compilation error */
pub const UNQLITE_VM_ERR: i32 = (-71); /* Virtual machine error */
pub const UNQLITE_FULL: i32 = (-73); /* Full database (unlikely) */
pub const UNQLITE_CANTOPEN: i32 = (-74); /* Unable to open the database file */
pub const UNQLITE_READ_ONLY: i32 = (-75); /* Read only Key/Value storage engine */
pub const UNQLITE_LOCKERR: i32 = (-76); /* Locking protocol error */
// end-of-error-codes

pub const UNQLITE_CONFIG_JX9_ERR_LOG: i32 = 1;
pub const UNQLITE_CONFIG_MAX_PAGE_CACHE: i32 = 2;
pub const UNQLITE_CONFIG_ERR_LOG: i32 = 3;
pub const UNQLITE_CONFIG_KV_ENGINE: i32 = 4;
pub const UNQLITE_CONFIG_DISABLE_AUTO_COMMIT: i32 = 5;
pub const UNQLITE_CONFIG_GET_KV_NAME: i32 = 6;

// UnQLite/Jx9 Virtual Machine Configuration Commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the Jx9 (Via UnQLite) Virtual machine.
// These constants must be passed as the second argument to the [unqlite_vm_config()]
// interface.
// Each options require a variable number of arguments.
// The [unqlite_vm_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// There are many options but the most importants are: UNQLITE_VM_CONFIG_OUTPUT which install
// a VM output consumer callback, UNQLITE_VM_CONFIG_HTTP_REQUEST which parse and register
// a HTTP request and UNQLITE_VM_CONFIG_ARGV_ENTRY which populate the $argv array.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_vm_config.html
//
pub const UNQLITE_VM_CONFIG_OUTPUT: i32 = 1;  /* TWO ARGUMENTS: int (*xConsumer)(const void *pOut, unsigned int nLen, void *pUserData), void *pUserData */
pub const UNQLITE_VM_CONFIG_IMPORT_PATH: i32 = 2;  /* ONE ARGUMENT: const char *zIncludePath */
pub const UNQLITE_VM_CONFIG_ERR_REPORT: i32 = 3;  /* NO ARGUMENTS: Report all run-time errors in the VM output */
pub const UNQLITE_VM_CONFIG_RECURSION_DEPTH: i32 = 4;  /* ONE ARGUMENT: int nMaxDepth */
pub const UNQLITE_VM_OUTPUT_LENGTH: i32 = 5;  /* ONE ARGUMENT: unsigned int *pLength */
pub const UNQLITE_VM_CONFIG_CREATE_VAR: i32 = 6;  /* TWO ARGUMENTS: const char *zName, unqlite_value *pValue */
pub const UNQLITE_VM_CONFIG_HTTP_REQUEST: i32 = 7;  /* TWO ARGUMENTS: const char *zRawRequest, int nRequestLength */
pub const UNQLITE_VM_CONFIG_SERVER_ATTR: i32 = 8;  /* THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen */
pub const UNQLITE_VM_CONFIG_ENV_ATTR: i32 = 9;  /* THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen */
pub const UNQLITE_VM_CONFIG_EXEC_VALUE: i32 = 10;  /* ONE ARGUMENT: unqlite_value **ppValue */
pub const UNQLITE_VM_CONFIG_IO_STREAM: i32 = 11;  /* ONE ARGUMENT: const unqlite_io_stream *pStream */
pub const UNQLITE_VM_CONFIG_ARGV_ENTRY: i32 = 12;  /* ONE ARGUMENT: const char *zValue */
pub const UNQLITE_VM_CONFIG_EXTRACT_OUTPUT: i32 = 13;  /* TWO ARGUMENTS: const void **ppOut, unsigned int *pOutputLen */

// Storage engine configuration commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the underlying storage engine (i.e Hash, B+tree, R+tree).
// These constants must be passed as the first argument to [unqlite_kv_config()].
// Each options require a variable number of arguments.
// The [unqlite_kv_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_kv_config.html
//
pub const UNQLITE_KV_CONFIG_HASH_FUNC: i32 = 1; /* ONE ARGUMENT: unsigned int (*xHash)(const void *,unsigned int) */
pub const UNQLITE_KV_CONFIG_CMP_FUNC: i32 = 2; /* ONE ARGUMENT: int (*xCmp)(const void *,const void *,unsigned int) */

// Global Library Configuration Commands.
//
// The following set of constants are the available configuration verbs that can
// be used by the host-application to configure the whole library.
// These constants must be passed as the first argument to [unqlite_lib_config()].
//
// Each options require a variable number of arguments.
// The [unqlite_lib_config()] interface will return UNQLITE_OK on success, any other return
// value indicates failure.
// Notes:
// The default configuration is recommended for most applications and so the call to
// [unqlite_lib_config()] is usually not necessary. It is provided to support rare
// applications with unusual needs.
// The [unqlite_lib_config()] interface is not threadsafe. The application must insure that
// no other [unqlite_*()] interfaces are invoked by other threads while [unqlite_lib_config()]
// is running. Furthermore, [unqlite_lib_config()] may only be invoked prior to library
// initialization using [unqlite_lib_init()] or [unqlite_init()] or after shutdown
// by [unqlite_lib_shutdown()]. If [unqlite_lib_config()] is called after [unqlite_lib_init()]
// or [unqlite_init()] and before [unqlite_lib_shutdown()] then it will return UNQLITE_LOCKED.
// For a full discussion on the configuration verbs and their expected parameters, please
// refer to this page:
//      http://unqlite.org/c_api/unqlite_lib.html
//
pub const UNQLITE_LIB_CONFIG_USER_MALLOC: i32 = 1; /* ONE ARGUMENT: const SyMemMethods *pMemMethods */
pub const UNQLITE_LIB_CONFIG_MEM_ERR_CALLBACK: i32 = 2; /* TWO ARGUMENTS: int (*xMemError)(void *), void *pUserData */
pub const UNQLITE_LIB_CONFIG_USER_MUTEX: i32 = 3; /* ONE ARGUMENT: const SyMutexMethods *pMutexMethods */
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE: i32 = 4; /* NO ARGUMENTS */
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI: i32 = 5; /* NO ARGUMENTS */
pub const UNQLITE_LIB_CONFIG_VFS: i32 = 6; /* ONE ARGUMENT: const unqlite_vfs *pVfs */
pub const UNQLITE_LIB_CONFIG_STORAGE_ENGINE: i32 = 7; /* ONE ARGUMENT: unqlite_kv_methods *pStorage */
pub const UNQLITE_LIB_CONFIG_PAGE_SIZE: i32 = 8; /* ONE ARGUMENT: int iPageSize */

// These bit values are intended for use in the 3rd parameter to the [unqlite_open()] interface
// and in the 4th parameter to the xOpen method of the [unqlite_vfs] object.
//
pub const UNQLITE_OPEN_READONLY: i32 = 0x00000001; /* Read only mode. Ok for [unqlite_open] */
pub const UNQLITE_OPEN_READWRITE: i32 = 0x00000002; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_CREATE: i32 = 0x00000004; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_EXCLUSIVE: i32 = 0x00000008; /* VFS only */
pub const UNQLITE_OPEN_TEMP_DB: i32 = 0x00000010; /* VFS only */
pub const UNQLITE_OPEN_NOMUTEX: i32 = 0x00000020; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_OMIT_JOURNALING: i32 = 0x00000040; /* Omit journaling for this database. Ok for [unqlite_open] */
pub const UNQLITE_OPEN_IN_MEMORY: i32 = 0x00000080; /* An in memory database. Ok for [unqlite_open]*/
pub const UNQLITE_OPEN_MMAP: i32 = 0x00000100; /* Obtain a memory view of the whole file. Ok for [unqlite_open] */
// Synchronization Type Flags
//
// When UnQLite invokes the xSync() method of an [unqlite_io_methods] object it uses
// a combination of these integer values as the second argument.
//
// When the UNQLITE_SYNC_DATAONLY flag is used, it means that the sync operation only
// needs to flush data to mass storage.: i32 = Inode information need not be flushed.
// If the lower four bits of the flag equal UNQLITE_SYNC_NORMAL, that means to use normal
// fsync() semantics. If the lower four bits equal UNQLITE_SYNC_FULL, that means to use
// Mac OS X style fullsync instead of fsync().
//
pub const UNQLITE_SYNC_NORMAL: i32 = 0x00002;
pub const UNQLITE_SYNC_FULL: i32 = 0x00003;
pub const UNQLITE_SYNC_DATAONLY: i32 = 0x00010;
// File Locking Levels
//
// UnQLite uses one of these integer values as the second
// argument to calls it makes to the xLock() and xUnlock() methods
// of an [unqlite_io_methods] object.
//
pub const UNQLITE_LOCK_NONE: i32 = 0;
pub const UNQLITE_LOCK_SHARED: i32 = 1;
pub const UNQLITE_LOCK_RESERVED: i32 = 2;
pub const UNQLITE_LOCK_PENDING: i32 = 3;
pub const UNQLITE_LOCK_EXCLUSIVE: i32 = 4;

// Flags for the xAccess VFS method
//
// These integer constants can be used as the third parameter to
// the xAccess method of an [unqlite_vfs] object.  They determine
// what kind of permissions the xAccess method is looking for.
// With UNQLITE_ACCESS_EXISTS, the xAccess method
// simply checks whether the file exists.
// With UNQLITE_ACCESS_READWRITE, the xAccess method
// checks whether the named directory is both readable and writable
// (in other words, if files can be added, removed, and renamed within
// the directory).
// The UNQLITE_ACCESS_READWRITE constant is currently used only by the
// [temp_store_directory pragma], though this could change in a future
// release of UnQLite.
// With UNQLITE_ACCESS_READ, the xAccess method
// checks whether the file is readable.  The UNQLITE_ACCESS_READ constant is
// currently unused, though it might be used in a future release of
// UnQLite.
//
pub const UNQLITE_ACCESS_EXISTS: i32 = 0;
pub const UNQLITE_ACCESS_READWRITE: i32 = 1;
pub const UNQLITE_ACCESS_READ: i32 = 2;
// Possible seek positions.
//
pub const UNQLITE_CURSOR_MATCH_EXACT: i32 = 1;
pub const UNQLITE_CURSOR_MATCH_LE: i32 = 2;
pub const UNQLITE_CURSOR_MATCH_GE: i32 = 3;
// UnQLite journal file suffix.
//
// #ifndef UNQLITE_JOURNAL_FILE_SUFFIX
pub const UNQLITE_JOURNAL_FILE_SUFFIX: &'static str = "_unqlite_journal";
// #endif
//
// Call Context - Error Message Serverity Level.
//
// The following constans are the allowed severity level that can
// passed as the second argument to the [unqlite_context_throw_error()] or
// [unqlite_context_throw_error_format()] interfaces.
// Refer to the official documentation for additional information.
//

pub const UNQLITE_CTX_ERR: i32 = 1; /* Call context error such as unexpected number of arguments, invalid types and so on. */
pub const UNQLITE_CTX_WARNING: i32 = 2; /* Call context Warning */
pub const UNQLITE_CTX_NOTICE: i32 = 3; /* Call context Notice */
