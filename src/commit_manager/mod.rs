use crate::conductor::transaction::Transaction;
use crate::alice_fs::*;


pub fn commit(transaction: Transaction) {
    let data = format!("{} - {} - {}", transaction.tid, transaction.cmd, transaction.timestamp);
    write_into_file("database_name.cm.adb", data);
}

#[macro_export]
macro_rules! commit {
    ( $k:expr ) => {
        commit($k)
    };
}

