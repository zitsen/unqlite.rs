use std::error;
use std::fmt;
use std::result;
use vars::*;

/// Custom `Result` type.
pub type Result<T> = result::Result<T, Error>;

/// Custom `Error` type.
#[derive(Debug)]
pub enum Error {
    /// UnQLite error code map
    Custom(Custom),
    /// Any kind of other errors
    Other(Box<error::Error>),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<Custom> for Error {
    fn from(err: Custom) -> Error {
        Error::Custom(err)
    }
}

impl From<::std::ffi::NulError> for Error {
    fn from(err: ::std::ffi::NulError) -> Error {
        Error::Other(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref c) => write!(f, "Custom error: {}", c),
            Error::Other(ref e) => write!(f, "Other error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(ref c) => c.description(),
            Error::Other(ref e) => e.as_ref().description(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Custom {
    kind: ErrorKind,
    raw: i32,
}

/// Error kinds from unqlite official documents.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// Successful result
    OK = 0,
    /// Out of memory
    NOMEM,
    /// Another thread have released this instance
    ABORT,
    /// IO error
    IOERR,
    /// Corrupt pointer
    CORRUPT,
    /// Forbidden Operation
    LOCKED,
    /// The database file is locked
    BUSY,
    /// Operation done, not an error
    DONE,
    /// Permission error
    PERM,
    /// Method not implemented
    NOTIMPLEMENTED,
    /// No such record
    NOTFOUND,
    /// No such method
    NOOP,
    /// Invalid parameter
    INVALID,
    /// End Of Input
    EOF,
    /// Unknown configuration option
    UNKNOWN,
    /// Database limit reached
    LIMIT,
    /// Record exists
    EXISTS,
    /// Empty record
    EMPTY,
    /// Compilation error
    COMPILE_ERR,
    /// Virtual machine error
    VM_ERR,
    /// Full database unlikely
    FULL,
    /// Unable to open the database file
    CANTOPEN,
    /// Read only Key/Value storage engine
    READ_ONLY,
    /// Locking protocol error
    LOCKERR,
    #[doc(hidden)] __Nonexhaustive,
}

impl From<i32> for ErrorKind {
    fn from(code: i32) -> ErrorKind {
        match code {
            UNQLITE_OK => ErrorKind::OK,
            UNQLITE_NOMEM => ErrorKind::NOMEM,
            UNQLITE_ABORT => ErrorKind::ABORT,
            UNQLITE_IOERR => ErrorKind::IOERR,
            UNQLITE_CORRUPT => ErrorKind::CORRUPT,
            UNQLITE_LOCKED => ErrorKind::LOCKED,
            UNQLITE_BUSY => ErrorKind::BUSY,
            UNQLITE_DONE => ErrorKind::DONE,
            UNQLITE_PERM => ErrorKind::PERM,
            UNQLITE_NOTIMPLEMENTED => ErrorKind::NOTIMPLEMENTED,
            UNQLITE_NOTFOUND => ErrorKind::NOTFOUND,
            UNQLITE_NOOP => ErrorKind::NOOP,
            UNQLITE_INVALID => ErrorKind::INVALID,
            UNQLITE_EOF => ErrorKind::EOF,
            UNQLITE_UNKNOWN => ErrorKind::UNKNOWN,
            UNQLITE_LIMIT => ErrorKind::LIMIT,
            UNQLITE_EXISTS => ErrorKind::EXISTS,
            UNQLITE_EMPTY => ErrorKind::EMPTY,
            UNQLITE_COMPILE_ERR => ErrorKind::COMPILE_ERR,
            UNQLITE_VM_ERR => ErrorKind::VM_ERR,
            UNQLITE_FULL => ErrorKind::FULL,
            UNQLITE_CANTOPEN => ErrorKind::CANTOPEN,
            UNQLITE_READ_ONLY => ErrorKind::READ_ONLY,
            UNQLITE_LOCKERR => ErrorKind::LOCKERR,
            _ => ErrorKind::__Nonexhaustive,
        }
    }
}

/// A wrap trait for unqlite FFI error code to Rust-y `Result`.
///
/// To populate better visual style, we add a `Wrap` trait to original
/// unqlite return value. The `Wrap` trait has only one method `drop` -
/// which is used to wrap the unqlite return value to Rust `Result`.
/// So the FFI-related methods should just use `.wrap()` like this:
///
/// ```ignore
/// unsafe {
///     unqlite_open(...).wrap()  // Now it is Result<(), Error>
/// }
/// ```
///
/// This should be nice for functional programming style.
pub(crate) trait Wrap {
    fn wrap(self) -> Result<()>;
}

impl Wrap for i32 {
    fn wrap(self) -> Result<()> {
        Custom::from_raw(self)
    }
}

impl Custom {
    pub fn from_raw(result: i32) -> Result<()> {
        let kind = ErrorKind::from(result);
        match kind {
            ErrorKind::OK => Ok(()),
            _ => Err(Custom {
                kind: kind,
                raw: result,
            }.into()),
        }
    }

    pub fn error(&self) -> &str {
        match self.kind {
            ErrorKind::NOMEM => "Out of memory",
            ErrorKind::ABORT => "Another thread have released this instance",
            ErrorKind::IOERR => "IO error",
            ErrorKind::CORRUPT => "Corrupt pointer",
            ErrorKind::LOCKED => "Forbidden Operation",
            ErrorKind::BUSY => "The database file is locked",
            ErrorKind::DONE => "Operation done",
            ErrorKind::PERM => "Permission error",
            ErrorKind::NOTIMPLEMENTED => {
                "Method not implemented by the underlying Key/Value storage engine"
            }
            ErrorKind::NOTFOUND => "No such record",
            ErrorKind::NOOP => "No such method",
            ErrorKind::INVALID => "Invalid parameter",
            ErrorKind::EOF => "End Of Input",
            ErrorKind::UNKNOWN => "Unknown configuration option",
            ErrorKind::LIMIT => "Database limit reached",
            ErrorKind::EXISTS => "Record exists",
            ErrorKind::EMPTY => "Empty record",
            ErrorKind::COMPILE_ERR => "Compilation error",
            ErrorKind::VM_ERR => " Virtual machine error",
            ErrorKind::FULL => "Full database (unlikely)",
            ErrorKind::CANTOPEN => "Unable to open the database file",
            ErrorKind::READ_ONLY => "Read only Key/Value storage engine",
            ErrorKind::LOCKERR => "Locking protocol error",
            ErrorKind::OK => unreachable!(),
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

impl error::Error for Custom {
    fn description(&self) -> &str {
        self.error()
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.error())
    }
}
