/// The UNQLITE_VERSION C preprocessor macroevaluates to a string literal
/// that is the unqlite version in the format "X.Y.Z" where X is the major
/// version number and Y is the minor version number and Z is the release
/// number.
///
pub const UNQLITE_VERSION: &'static str = "1.1.6";

pub const UNQLITE_VERSION_NUMBER: ::libc::c_int = 1001006;

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
pub const UNQLITE_IDENT: &'static str = "unqlite:b172a1e2c3f62fb35c8e1fb2795121f82356cad6";
// Copyright notice.
// If you have any questions about the licensing situation, please
// visit http://unqlite.org/licensing.html
// or contact Symisc Systems via:
//   legal@symisc.net
//   licensing@symisc.net
//   contact@symisc.net
//
pub const UNQLITE_COPYRIGHT: &'static str = "Copyright (C) Symisc Systems, S.U.A.R.L [Mrad Chems Eddine <chm@symisc.net>] 2012-2013, http://unqlite.org/";

// Standard return values from Symisc public interfaces
pub const SXRET_OK: ::libc::c_int = 0;      /* Not an error */
pub const SXERR_MEM: ::libc::c_int = (-1); /* Out of memory */
pub const SXERR_IO: ::libc::c_int = (-2); /* IO error */
pub const SXERR_EMPTY: ::libc::c_int = (-3); /* Empty field */
pub const SXERR_LOCKED: ::libc::c_int = (-4); /* Locked operation */
pub const SXERR_ORANGE: ::libc::c_int = (-5); /* Out of range value */
pub const SXERR_NOTFOUND: ::libc::c_int = (-6); /* Item not found */
pub const SXERR_LIMIT: ::libc::c_int = (-7); /* Limit reached */
pub const SXERR_MORE: ::libc::c_int = (-8); /* Need more input */
pub const SXERR_INVALID: ::libc::c_int = (-9); /* Invalid parameter */
pub const SXERR_ABORT: ::libc::c_int = (-10); /* User callback request an operation abort */
pub const SXERR_EXISTS: ::libc::c_int = (-11); /* Item exists */
pub const SXERR_SYNTAX: ::libc::c_int = (-12); /* Syntax error */
pub const SXERR_UNKNOWN: ::libc::c_int = (-13); /* Unknown error */
pub const SXERR_BUSY: ::libc::c_int = (-14); /* Busy operation */
pub const SXERR_OVERFLOW: ::libc::c_int = (-15); /* Stack or buffer overflow */
pub const SXERR_WILLBLOCK: ::libc::c_int = (-16); /* Operation will block */
pub const SXERR_NOTIMPLEMENTED: ::libc::c_int = (-17); /* Operation not implemented */
pub const SXERR_EOF: ::libc::c_int = (-18); /* End of input */
pub const SXERR_PERM: ::libc::c_int = (-19); /* Permission error */
pub const SXERR_NOOP: ::libc::c_int = (-20); /* No-op */
pub const SXERR_FORMAT: ::libc::c_int = (-21); /* Invalid format */
pub const SXERR_NEXT: ::libc::c_int = (-22); /* Not an error */
pub const SXERR_OS: ::libc::c_int = (-23); /* System call return an error */
pub const SXERR_CORRUPT: ::libc::c_int = (-24); /* Corrupted pointer */
pub const SXERR_CONTINUE: ::libc::c_int = (-25); /* Not an error: Operation in progress */
pub const SXERR_NOMATCH: ::libc::c_int = (-26); /* No match */
pub const SXERR_RESET: ::libc::c_int = (-27); /* Operation reset */
pub const SXERR_DONE: ::libc::c_int = (-28); /* Not an error */
pub const SXERR_SHORT: ::libc::c_int = (-29); /* Buffer too short */
pub const SXERR_PATH: ::libc::c_int = (-30); /* Path error */
pub const SXERR_TIMEOUT: ::libc::c_int = (-31); /* Timeout */
pub const SXERR_BIG: ::libc::c_int = (-32); /* Too big for processing */
pub const SXERR_RETRY: ::libc::c_int = (-33); /* Retry your call */
pub const SXERR_IGNORE: ::libc::c_int = (-63); /* Ignore */
// Standard UnQLite return values
pub const UNQLITE_OK: ::libc::c_int = SXRET_OK; /* Successful result */
// Beginning of error codes
pub const UNQLITE_NOMEM: ::libc::c_int = SXERR_MEM; /* Out of memory */
pub const UNQLITE_ABORT: ::libc::c_int = SXERR_ABORT; /* Another thread have released this instance */
pub const UNQLITE_IOERR: ::libc::c_int = SXERR_IO; /* IO error */
pub const UNQLITE_CORRUPT: ::libc::c_int = SXERR_CORRUPT; /* Corrupt pointer */
pub const UNQLITE_LOCKED: ::libc::c_int = SXERR_LOCKED; /* Forbidden Operation */
pub const UNQLITE_BUSY: ::libc::c_int = SXERR_BUSY; /* The database file is locked */
pub const UNQLITE_DONE: ::libc::c_int = SXERR_DONE; /* Operation done */
pub const UNQLITE_PERM: ::libc::c_int = SXERR_PERM; /* Permission error */
pub const UNQLITE_NOTIMPLEMENTED: ::libc::c_int = SXERR_NOTIMPLEMENTED; /* Method not implemented by the underlying Key/Value storage engine */
pub const UNQLITE_NOTFOUND: ::libc::c_int = SXERR_NOTFOUND; /* No such record */
pub const UNQLITE_NOOP: ::libc::c_int = SXERR_NOOP; /* No such method */
pub const UNQLITE_INVALID: ::libc::c_int = SXERR_INVALID; /* Invalid parameter */
pub const UNQLITE_EOF: ::libc::c_int = SXERR_EOF; /* End Of Input */
pub const UNQLITE_UNKNOWN: ::libc::c_int = SXERR_UNKNOWN; /* Unknown configuration option */
pub const UNQLITE_LIMIT: ::libc::c_int = SXERR_LIMIT; /* Database limit reached */
pub const UNQLITE_EXISTS: ::libc::c_int = SXERR_EXISTS; /* Record exists */
pub const UNQLITE_EMPTY: ::libc::c_int = SXERR_EMPTY; /* Empty record */
pub const UNQLITE_COMPILE_ERR: ::libc::c_int = -70; /* Compilation error */
pub const UNQLITE_VM_ERR: ::libc::c_int = (-71); /* Virtual machine error */
pub const UNQLITE_FULL: ::libc::c_int = (-73); /* Full database (unlikely) */
pub const UNQLITE_CANTOPEN: ::libc::c_int = (-74); /* Unable to open the database file */
pub const UNQLITE_READ_ONLY: ::libc::c_int = (-75); /* Read only Key/Value storage engine */
pub const UNQLITE_LOCKERR: ::libc::c_int = (-76); /* Locking protocol error */
// end-of-error-codes

pub const UNQLITE_CONFIG_JX9_ERR_LOG: ::libc::c_int = 1;
pub const UNQLITE_CONFIG_MAX_PAGE_CACHE: ::libc::c_int = 2;
pub const UNQLITE_CONFIG_ERR_LOG: ::libc::c_int = 3;
pub const UNQLITE_CONFIG_KV_ENGINE: ::libc::c_int = 4;
pub const UNQLITE_CONFIG_DISABLE_AUTO_COMMIT: ::libc::c_int = 5;
pub const UNQLITE_CONFIG_GET_KV_NAME: ::libc::c_int = 6;

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
pub const UNQLITE_VM_CONFIG_OUTPUT: ::libc::c_int = 1;  /* TWO ARGUMENTS: int (*xConsumer)(const void *pOut, unsigned int nLen, void *pUserData), void *pUserData */
pub const UNQLITE_VM_CONFIG_IMPORT_PATH: ::libc::c_int = 2;  /* ONE ARGUMENT: const char *zIncludePath */
pub const UNQLITE_VM_CONFIG_ERR_REPORT: ::libc::c_int = 3;  /* NO ARGUMENTS: Report all run-time errors in the VM output */
pub const UNQLITE_VM_CONFIG_RECURSION_DEPTH: ::libc::c_int = 4;  /* ONE ARGUMENT: int nMaxDepth */
pub const UNQLITE_VM_OUTPUT_LENGTH: ::libc::c_int = 5;  /* ONE ARGUMENT: unsigned int *pLength */
pub const UNQLITE_VM_CONFIG_CREATE_VAR: ::libc::c_int = 6;  /* TWO ARGUMENTS: const char *zName, unqlite_value *pValue */
pub const UNQLITE_VM_CONFIG_HTTP_REQUEST: ::libc::c_int = 7;  /* TWO ARGUMENTS: const char *zRawRequest, int nRequestLength */
pub const UNQLITE_VM_CONFIG_SERVER_ATTR: ::libc::c_int = 8;  /* THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen */
pub const UNQLITE_VM_CONFIG_ENV_ATTR: ::libc::c_int = 9;  /* THREE ARGUMENTS: const char *zKey, const char *zValue, int nLen */
pub const UNQLITE_VM_CONFIG_EXEC_VALUE: ::libc::c_int = 10;  /* ONE ARGUMENT: unqlite_value **ppValue */
pub const UNQLITE_VM_CONFIG_IO_STREAM: ::libc::c_int = 11;  /* ONE ARGUMENT: const unqlite_io_stream *pStream */
pub const UNQLITE_VM_CONFIG_ARGV_ENTRY: ::libc::c_int = 12;  /* ONE ARGUMENT: const char *zValue */
pub const UNQLITE_VM_CONFIG_EXTRACT_OUTPUT: ::libc::c_int = 13;  /* TWO ARGUMENTS: const void **ppOut, unsigned int *pOutputLen */

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
pub const UNQLITE_KV_CONFIG_HASH_FUNC: ::libc::c_int = 1; /* ONE ARGUMENT: unsigned int (*xHash)(const void *,unsigned int) */
pub const UNQLITE_KV_CONFIG_CMP_FUNC: ::libc::c_int = 2; /* ONE ARGUMENT: int (*xCmp)(const void *,const void *,unsigned int) */

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
pub const UNQLITE_LIB_CONFIG_USER_MALLOC: ::libc::c_int = 1; /* ONE ARGUMENT: const SyMemMethods *pMemMethods */
pub const UNQLITE_LIB_CONFIG_MEM_ERR_CALLBACK: ::libc::c_int = 2; /* TWO ARGUMENTS: int (*xMemError)(void *), void *pUserData */
pub const UNQLITE_LIB_CONFIG_USER_MUTEX: ::libc::c_int = 3; /* ONE ARGUMENT: const SyMutexMethods *pMutexMethods */
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_SINGLE: ::libc::c_int = 4; /* NO ARGUMENTS */
pub const UNQLITE_LIB_CONFIG_THREAD_LEVEL_MULTI: ::libc::c_int = 5; /* NO ARGUMENTS */
pub const UNQLITE_LIB_CONFIG_VFS: ::libc::c_int = 6; /* ONE ARGUMENT: const unqlite_vfs *pVfs */
pub const UNQLITE_LIB_CONFIG_STORAGE_ENGINE: ::libc::c_int = 7; /* ONE ARGUMENT: unqlite_kv_methods *pStorage */
pub const UNQLITE_LIB_CONFIG_PAGE_SIZE: ::libc::c_int = 8; /* ONE ARGUMENT: int iPageSize */

// These bit values are intended for use in the 3rd parameter to the [unqlite_open()] interface
// and in the 4th parameter to the xOpen method of the [unqlite_vfs] object.
//
pub const UNQLITE_OPEN_READONLY: ::libc::c_uint = 0x00000001; /* Read only mode. Ok for [unqlite_open] */
pub const UNQLITE_OPEN_READWRITE: ::libc::c_uint = 0x00000002; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_CREATE: ::libc::c_uint = 0x00000004; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_EXCLUSIVE: ::libc::c_uint = 0x00000008; /* VFS only */
pub const UNQLITE_OPEN_TEMP_DB: ::libc::c_uint = 0x00000010; /* VFS only */
pub const UNQLITE_OPEN_NOMUTEX: ::libc::c_uint = 0x00000020; /* Ok for [unqlite_open] */
pub const UNQLITE_OPEN_OMIT_JOURNALING: ::libc::c_uint = 0x00000040; /* Omit journaling for this database. Ok for [unqlite_open] */
pub const UNQLITE_OPEN_IN_MEMORY: ::libc::c_uint = 0x00000080; /* An in memory database. Ok for [unqlite_open]*/
pub const UNQLITE_OPEN_MMAP: ::libc::c_uint = 0x00000100; /* Obtain a memory view of the whole file. Ok for [unqlite_open] */
// Synchronization Type Flags
//
// When UnQLite invokes the xSync() method of an [unqlite_io_methods] object it uses
// a combination of these integer values as the second argument.
//
// When the UNQLITE_SYNC_DATAONLY flag is used, it means that the sync operation only
// needs to flush data to mass storage.: ::libc::c_int = Inode information need not be flushed.
// If the lower four bits of the flag equal UNQLITE_SYNC_NORMAL, that means to use normal
// fsync() semantics. If the lower four bits equal UNQLITE_SYNC_FULL, that means to use
// Mac OS X style fullsync instead of fsync().
//
pub const UNQLITE_SYNC_NORMAL: ::libc::c_int = 0x00002;
pub const UNQLITE_SYNC_FULL: ::libc::c_int = 0x00003;
pub const UNQLITE_SYNC_DATAONLY: ::libc::c_int = 0x00010;
// File Locking Levels
//
// UnQLite uses one of these integer values as the second
// argument to calls it makes to the xLock() and xUnlock() methods
// of an [unqlite_io_methods] object.
//
pub const UNQLITE_LOCK_NONE: ::libc::c_int = 0;
pub const UNQLITE_LOCK_SHARED: ::libc::c_int = 1;
pub const UNQLITE_LOCK_RESERVED: ::libc::c_int = 2;
pub const UNQLITE_LOCK_PENDING: ::libc::c_int = 3;
pub const UNQLITE_LOCK_EXCLUSIVE: ::libc::c_int = 4;

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
pub const UNQLITE_ACCESS_EXISTS: ::libc::c_int = 0;
pub const UNQLITE_ACCESS_READWRITE: ::libc::c_int = 1;
pub const UNQLITE_ACCESS_READ: ::libc::c_int = 2;
// Possible seek positions.
//
pub const UNQLITE_CURSOR_MATCH_EXACT: ::libc::c_int = 1;
pub const UNQLITE_CURSOR_MATCH_LE: ::libc::c_int = 2;
pub const UNQLITE_CURSOR_MATCH_GE: ::libc::c_int = 3;
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

pub const UNQLITE_CTX_ERR: ::libc::c_int = 1; /* Call context error such as unexpected number of arguments, invalid types and so on. */
pub const UNQLITE_CTX_WARNING: ::libc::c_int = 2; /* Call context Warning */
pub const UNQLITE_CTX_NOTICE: ::libc::c_int = 3; /* Call context Notice */
