
pub struct Transaction {
    pub tid: String,
    pub cmd: String,
    pub timestamp: String,
    pub priority: Option<u8>,
}

impl Transaction {
    pub fn create(tid: String, cmd: String, timestamp: String, priority: Option<u8>) -> Transaction {
        Transaction { tid, cmd, timestamp, priority }
    }
}