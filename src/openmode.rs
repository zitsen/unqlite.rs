use ffi::constants;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMode {
    ReadOnly,
    // ReadWrite,
    Create,
    // Exclusive,
    TempDB,
    // NoMutex,
    // OmitJournaling,
    // InMemory,
    MMap,
}

pub use self::OpenMode::*;

impl Into<u32> for OpenMode {
    fn into(self) -> u32 {
        match self {
            ReadOnly => constants::UNQLITE_OPEN_READONLY,
            // ReadWrite => constants::UNQLITE_OPEN_READWRITE,
            Create => constants::UNQLITE_OPEN_CREATE,
            // Exclusive => constants::UNQLITE_OPEN_EXCLUSIVE,
            TempDB => constants::UNQLITE_OPEN_TEMP_DB,
            // NoMutex => constants::UNQLITE_OPEN_NOMUTEX,
            // OmitJournaling => constants::UNQLITE_OPEN_OMIT_JOURNALING,
            // InMemory => constants::UNQLITE_OPEN_IN_MEMORY,
            MMap => constants::UNQLITE_OPEN_MMAP | constants::UNQLITE_OPEN_READONLY,
        }
    }
}
