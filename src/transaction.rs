use super::UnQlite;
use ffi::{unqlite_begin, unqlite_commit, unqlite_rollback};

/// Manual Transaction Manager
impl<'transaction> UnQlite {
    /// Manually begin a write-transaction on the specified database handle.
    ///
    /// Begin a write-transaction on the specified database handle. If a write-transaction has
    /// already been opened, this function is a no-op.
    /// Tip: For maximum concurrency, it is preferable to let UnQLite start the transaction for you
    /// automatically. An automatic transaction is started each time upper-layers or client code
    /// request a store, delete or an append operation.
    ///
    pub fn begin(&mut self) -> ::Result<()> {
        error_or!(unsafe { unqlite_begin(self.as_raw_mut_ptr()) })
    }

    /// Commit all changes to the database.
    ///
    /// Commit all changes to the database and release the exclusive lock. In other words, make
    /// sure that all changes reaches the disk surface.
    ///
    /// **Note**: Normally, a call to this routine is not necessary since transactions are committed
    /// automatically by the engine when the database is closed after lifetime end unless the
    /// `disable_auto_commit()` option is set. In which case, you should manually call
    /// `commit()`. Otherwise, the database is rolled back.
    ///
    /// *Tip*: For maximum concurrency, it is recommended that you commit your transaction manually
    /// as soon as you have no more insertions. Also, for very large insertions (More than 20000),
    /// you should call `commit()` periodically to free some memory (A new transaction is
    /// started automatically in the next insertion).
    pub fn commit(&mut self) -> ::Result<()> {
        match error_or!(unsafe { unqlite_commit(self.as_raw_mut_ptr()) }) {
            Ok(_) => Ok(()),
            Err(err) => {
                let _ = self.rollback();
                Err(err)
            }
        }
    }

    /// Rollback a write-transaction on the specified database handle.
    ///
    /// If a write transaction is open, then all changes made within the transaction are reverted
    /// and the current write-transaction is closed (Dropping all exclusive locks on the target
    /// database, deletion of the journal file, etc.). Otherwise this routine is a no-op.
    ///
    fn rollback(&mut self) -> ::Result<()> {
        error_or!(unsafe { unqlite_rollback(self.as_raw_mut_ptr()) })
    }
}
