// blockchain.rs

pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: Vec::new() }
    }

    pub fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32;
        let timestamp = chrono::Utc::now().to_string();
        let previous_hash = self.chain.last().map_or(String::new(), |block| block.previous_hash.clone());
        let block = Block { index, timestamp, data, previous_hash };
        self.chain.push(block);
    }
}