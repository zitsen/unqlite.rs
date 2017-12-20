//! Document Store (JSON via Jx9) Interfaces.

mod doc_store;
mod vm_value;

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests;

pub use self::doc_store::{Jx9, UnQLiteVm};
pub use self::vm_value::{Map, Value};
