use ffi::{unqlite_begin, unqlite_commit, unqlite_rollback};

use UnQLite;
use error::{Result, Wrap};

/// Manual Transaction Manager
///
///
pub trait Transaction {
    /// Manually begin a write-transaction on the specified database handle.
    ///
    /// Begin a write-transaction on the specified database handle. If a write-transaction has
    /// already been opened, this function is a no-op.
    /// Tip: For maximum concurrency, it is preferable to let UnQLite start the transaction for you
    /// automatically. An automatic transaction is started each time upper-layers or client code
    /// request a store, delete or an append operation.
    ///
    fn begin(&self) -> Result<()>;

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
    fn commit(&self) -> Result<()>;

    /// Rollback a write-transaction on the specified database handle.
    ///
    /// If a write transaction is open, then all changes made within the transaction are reverted
    /// and the current write-transaction is closed (Dropping all exclusive locks on the target
    /// database, deletion of the journal file, etc.). Otherwise this routine is a no-op.
    ///
    fn rollback(&self) -> Result<()>;
}

impl Transaction for UnQLite {
    fn begin(&self) -> Result<()> {
        wrap_raw!(self, begin)
    }

    fn commit(&self) -> Result<()> {
        wrap_raw!(self, commit).map_err(|err| {
            let _ = self.rollback();
            err
        })
    }

    fn rollback(&self) -> Result<()> {
        wrap_raw!(self, rollback)
    }
}

#[cfg(test)]
#[cfg(feature = "enable-threads")]
mod tests {
    use UnQLite;
    use super::Transaction;
    use Config;
    #[test]
    fn transaction() {
        let uq = UnQLite::create_temp().disable_auto_commit();
        uq.begin().expect("begin");
        uq.commit().expect("commit");
        uq.rollback().expect("rollback");
    }
}
