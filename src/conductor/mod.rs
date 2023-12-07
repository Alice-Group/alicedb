
mod transaction;
use transaction::Transaction;
use std::error::Error;

type DefaultResponse = Result<(), Box<dyn Error>>;

pub struct Conductor {
    pub transactions: Vec<Transaction>,
}

impl Conductor {
    pub fn init() -> Conductor {
        let mut transactions_vec: Vec<Transaction> = Vec::new();
        Conductor { transactions: transactions_vec }
    }

    pub fn push_back(&mut self, transaction: Transaction) -> DefaultResponse {
        return match self.transactions.push(transaction) {
            Ok(_) => Ok(()),
            Err(why) => panic!("Cant add transaction.  {:?}", why.kind()),
        };
    }

    pub fn delete_first(&mut self,) -> DefaultResponse {
        return match self.transactions.pop() {
            Ok(_) => Ok(()),
            Err(why) => panic!("Cant delete first transaction.  {:?}", why.kind()),
        };
    }
    fn spawn_threads(&mut self) -> DefaultResponse {
        Ok(())
    }
}