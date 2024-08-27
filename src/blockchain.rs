use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            chain: vec![Block::new(0, "Genesis Block".to_string(), "0".to_string())],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash
                != Block::calculate_hash(
                    current_block.index,
                    current_block.timestamp,
                    &current_block.data,
                    &current_block.previous_hash,
                )
            {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

pub fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());

    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
