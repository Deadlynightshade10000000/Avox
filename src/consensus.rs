// src/consensus.rs

// Implementation of Proof of Work mining.

pub struct ProofOfWork {
    pub difficulty: u32,
}

impl ProofOfWork {
    pub fn new(difficulty: u32) -> Self {
        ProofOfWork { difficulty }
    }

    pub fn mine(&self, data: &[u8]) -> (u32, String) {
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            // Create a hash of the nonce and data
            hash = format!("{}-{}", sha256::digest(&data), nonce);

            // Check if we have reached the required difficulty
            if self.check_difficulty(&hash) {
                break;
            }

            nonce += 1;
        }

        (nonce, hash)
    }

    fn check_difficulty(&self, hash: &str) -> bool {
        hash.chars().take(self.difficulty as usize).all(|c| c == '0')
    }
}
