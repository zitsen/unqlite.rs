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
            ReadOnly => crate::vars::UNQLITE_OPEN_READONLY,
            // ReadWrite => vars::UNQLITE_OPEN_READWRITE,
            Create => crate::vars::UNQLITE_OPEN_CREATE,
            // Exclusive =>vars::UNQLITE_OPEN_EXCLUSIVE,
            TempDB => crate::vars::UNQLITE_OPEN_TEMP_DB,
            // NoMutex =>vars::UNQLITE_OPEN_NOMUTEX,
            // OmitJournaling =>vars::UNQLITE_OPEN_OMIT_JOURNALING,
            // InMemory =>vars::UNQLITE_OPEN_IN_MEMORY,
            MMap => crate::vars::UNQLITE_OPEN_MMAP | crate::vars::UNQLITE_OPEN_READONLY,
        }
    }
}
