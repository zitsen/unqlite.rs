use ffi::*;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnqliteFailure(UnqliteFailure),
    NulError(::std::ffi::NulError),
}

impl From<::std::ffi::NulError> for Error {
    fn from(err: ::std::ffi::NulError) -> Error {
        Error::NulError(err)
    }
}

impl From<UnqliteFailure> for Error {
    fn from(err: UnqliteFailure) -> Error {
        Error::UnqliteFailure(err)
    }
}

impl From<i32> for Error {
    fn from(err: i32) -> Error {
        Error::UnqliteFailure(UnqliteFailure::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::UnqliteFailure(ref err) => err.fmt(f),
            &Error::NulError(ref err) => err.fmt(f),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ErrorCode {
    UNQLITE_NOMEM,
    UNQLITE_ABORT,
    UNQLITE_IOERR,
    UNQLITE_CORRUPT,
    UNQLITE_LOCKED,
    UNQLITE_BUSY,
    UNQLITE_DONE,
    UNQLITE_PERM,
    UNQLITE_NOTIMPLEMENTED,
    UNQLITE_NOTFOUND,
    UNQLITE_NOOP,
    UNQLITE_INVALID,
    UNQLITE_EOF,
    UNQLITE_UNKNOWN,
    UNQLITE_LIMIT,
    UNQLITE_EXISTS,
    UNQLITE_EMPTY,
    UNQLITE_COMPILE_ERR,
    UNQLITE_VM_ERR,
    UNQLITE_FULL,
    UNQLITE_CANTOPEN,
    UNQLITE_READ_ONLY,
    UNQLITE_LOCKERR,
    UNREACHABLE,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnqliteFailure {
    code: ErrorCode,
    extended_code: i32,
}

impl UnqliteFailure {
    pub fn new(result_code: i32) -> UnqliteFailure {
        let code = match result_code & 0xff {
            UNQLITE_NOMEM => ErrorCode::UNQLITE_NOMEM,
            UNQLITE_ABORT => ErrorCode::UNQLITE_ABORT,
            UNQLITE_IOERR => ErrorCode::UNQLITE_IOERR,
            UNQLITE_CORRUPT => ErrorCode::UNQLITE_CORRUPT,
            UNQLITE_LOCKED => ErrorCode::UNQLITE_LOCKED,
            UNQLITE_BUSY => ErrorCode::UNQLITE_BUSY,
            UNQLITE_DONE => ErrorCode::UNQLITE_DONE,
            UNQLITE_PERM => ErrorCode::UNQLITE_PERM,
            UNQLITE_NOTIMPLEMENTED => ErrorCode::UNQLITE_NOTIMPLEMENTED,
            UNQLITE_NOTFOUND => ErrorCode::UNQLITE_NOTFOUND,
            UNQLITE_NOOP => ErrorCode::UNQLITE_NOOP,
            UNQLITE_INVALID => ErrorCode::UNQLITE_INVALID,
            UNQLITE_EOF => ErrorCode::UNQLITE_EOF,
            UNQLITE_UNKNOWN => ErrorCode::UNQLITE_UNKNOWN,
            UNQLITE_LIMIT => ErrorCode::UNQLITE_LIMIT,
            UNQLITE_EXISTS => ErrorCode::UNQLITE_EXISTS,
            UNQLITE_EMPTY => ErrorCode::UNQLITE_EMPTY,
            UNQLITE_COMPILE_ERR => ErrorCode::UNQLITE_COMPILE_ERR,
            UNQLITE_VM_ERR => ErrorCode::UNQLITE_VM_ERR,
            UNQLITE_FULL => ErrorCode::UNQLITE_FULL,
            UNQLITE_CANTOPEN => ErrorCode::UNQLITE_CANTOPEN,
            UNQLITE_READ_ONLY => ErrorCode::UNQLITE_READ_ONLY,
            UNQLITE_LOCKERR => ErrorCode::UNQLITE_LOCKERR,
            _ => ErrorCode::UNREACHABLE,
        };
        UnqliteFailure {
            code: code,
            extended_code: result_code,
        }
    }
}

impl fmt::Display for UnqliteFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Error code {}: {}",
               self.extended_code,
               code_to_str(self.extended_code))
    }
}

impl error::Error for UnqliteFailure {
    fn description(&self) -> &str {
        code_to_str(self.extended_code)
    }
}

fn code_to_str(code: i32) -> &'static str {
    match code {
        UNQLITE_NOMEM => "Out of memory",
        UNQLITE_ABORT => "Another thread have released this instance",
        UNQLITE_IOERR => "IO error",
        UNQLITE_CORRUPT => "Corrupt pointer",
        UNQLITE_LOCKED => "Forbidden Operation",
        UNQLITE_BUSY => "The database file is locked",
        UNQLITE_DONE => "Operation done",
        UNQLITE_PERM => "Permission error",
        UNQLITE_NOTIMPLEMENTED => "Method not implemented by the underlying Key/Value storage engine",
        UNQLITE_NOTFOUND => "No such record",
        UNQLITE_NOOP => "No such method",
        UNQLITE_INVALID => "Invalid parameter",
        UNQLITE_EOF => "End Of Input",
        UNQLITE_UNKNOWN => "Unknown configuration option",
        UNQLITE_LIMIT => "Database limit reached",
        UNQLITE_EXISTS => "Record exists",
        UNQLITE_EMPTY => "Empty record",
        UNQLITE_COMPILE_ERR => "Compilation error",
        UNQLITE_VM_ERR => " Virtual machine error",
        UNQLITE_FULL => "Full database (unlikely)",
        UNQLITE_CANTOPEN => "Unable to open the database file",
        UNQLITE_READ_ONLY => "Read only Key/Value storage engine",
        UNQLITE_LOCKERR => "Locking protocol error",
        _ => "Unreachble code",
    }
}
