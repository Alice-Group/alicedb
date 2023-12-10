

#[derive(Debug)]
pub struct Transaction {
    pub tid: String,
    pub cmd: String,
    pub timestamp: String,
}

#[macro_export]
macro_rules! create_transaction {
    ( $($k:expr), * ) =>
    {
        Transaction::create(
            $(
                $k.to_string()
            ),*)
    };
}

impl Transaction {
    pub fn create(tid: String, cmd: String, timestamp: String) -> Transaction {
        Transaction { tid, cmd, timestamp}
    }
}
