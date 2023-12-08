
pub mod transaction;
use transaction::Transaction;
use std::error::Error;
use std::thread;
type DefaultResponse = Result<(), Box<dyn Error>>;

pub struct Conductor {
    pub transactions: Vec<Transaction>,

}

impl Conductor {
    pub fn init() -> Conductor {
        let mut transactions_vec: Vec<Transaction> = Vec::new();
        Conductor { transactions: transactions_vec, }
    }

    pub fn push_back(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn delete_first(&mut self,) {
        self.transactions.pop();
    }
    pub fn execute(&self) {
        todo!();
    }
}
