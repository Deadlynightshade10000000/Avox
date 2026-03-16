// src/transactions.rs

// Define transaction types

pub struct Transaction {
    pub id: u64,
    pub amount: f64,
    pub sender: String,
    pub receiver: String,
}

impl Transaction {
    pub fn new(id: u64, amount: f64, sender: String, receiver: String) -> Self {
        Transaction { id, amount, sender, receiver }
    }
}

// Mempool implementation

pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool { transactions: Vec::new() }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn remove_transaction(&mut self, id: u64) {
        self.transactions.retain(|tx| tx.id != id);
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}