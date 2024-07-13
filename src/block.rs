use hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::linked_list::LinkedList as List;
use crate::mresult::MResult as Result;

pub struct BlockChain {
    blocks: List<Block>,
    block_index: HashMap<String, Block>,
    height_index: HashMap<u128, String>,
}

#[derive(Clone)]
pub struct Block {
    pub hash: String,
    height: u64,
    transactions: List<Transaction>,
    prev_block_hash: String,
    timestamp: u64,
    merkle_root: String,
    nonce: u32,
}

#[derive(Clone)]
pub struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}

#[derive(Clone)]
pub struct TxIn {
    prev_txid: String,
    vout: usize,
    signature: String,
    sequence: u32,
}

#[derive(Clone)]
pub struct TxOut {
    public_address: String,
    satoshis: u64,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
            block_index: HashMap::new(),
            height_index: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str> {
        if !self.is_valid_block(&block) {
            return Result::Err("Invalid block");
        }

        let block_hash = block.hash.clone();
        let block_height = block.height;

        self.height_index
            .insert(block_height.into(), block_hash.clone());
        self.blocks.push_front(block.clone());
        self.block_index.insert(block_hash, block);

        Result::Ok(())
    }

    pub fn is_valid_block(&self, block: &Block) -> bool {
        if block.height > 0 {
            self.get_block_by_hash(&block.prev_block_hash).is_some()
        } else {
            true // Genesis block
        }
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.block_index.get(hash)
    }

    pub fn get_block_by_height(&self, height: u128) -> Option<&Block> {
        self.height_index
            .get(&height)
            .and_then(|hash| self.block_index.get(hash))
    }

    pub fn get_block_count(&self) -> usize {
        self.blocks.iter().count()
    }

    pub fn get_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .find_map(|block| block.get_transaction(txid))
    }

    pub fn get_best_block_hash(&self) -> Option<String> {
        self.blocks
            .iter()
            .next()
            .map(|block| block.hash.to_string())
    }
}

impl Block {
    pub fn new(
        height: u64,
        prev_block_hash: String,
        timestamp: u64,
        merkle_root: String,
        nonce: u32,
    ) -> Self {
        let mut block = Block {
            hash: String::new(),
            height,
            transactions: List::new(),
            prev_block_hash,
            timestamp,
            merkle_root,
            nonce,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.height.to_string());
        hasher.update(&self.prev_block_hash);
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.merkle_root);
        hasher.update(self.nonce.to_string());
        hex::encode(hasher.finalize())
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        self.transactions.push_front(transaction);
        self.hash = self.calculate_hash();

        Result::Ok(())
    }

    pub fn get_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions.iter().find(|tx| tx.txid == txid)
    }
}

impl Transaction {
    pub fn new(inputs: List<TxIn>, outputs: List<TxOut>) -> Self {
        let mut tx = Transaction {
            inputs,
            outputs,
            txid: String::new(),
        };
        tx.txid = tx.calculate_txid();
        tx
    }

    pub fn calculate_txid(&self) -> String {
        let mut hasher = Sha256::new();
        for input in self.inputs.iter() {
            hasher.update(&input.prev_txid);
            hasher.update(input.vout.to_string());
            hasher.update(&input.signature);
            hasher.update(input.sequence.to_string());
        }
        for output in self.outputs.iter() {
            hasher.update(&output.public_address);
            hasher.update(output.satoshis.to_string());
        }
        hex::encode(hasher.finalize())
    }
}

impl TxIn {
    pub fn new(prev_txid: String, vout: usize, signature: String, sequence: u32) -> Self {
        TxIn {
            prev_txid,
            vout,
            signature,
            sequence,
        }
    }
}

impl TxOut {
    pub fn new(public_address: String, satoshis: u64) -> Self {
        TxOut {
            public_address,
            satoshis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::linked_list::LinkedList as List;

    #[test]
    fn test_txin() {
        let txin = TxIn::new(
            "previous_txid".to_string(),
            0,
            "signature".to_string(),
            0xffffffff,
        );
        assert_eq!(txin.prev_txid, "previous_txid");
        assert_eq!(txin.vout, 0);
        assert_eq!(txin.signature, "signature");
        assert_eq!(txin.sequence, 0xffffffff);
    }

    #[test]
    fn test_txout() {
        let txout = TxOut::new("public_address".to_string(), 50_000_000);
        assert_eq!(txout.satoshis, 50_000_000);
        assert_eq!(txout.public_address, "public_address");
    }

    #[test]
    fn test_transaction() {
        let mut inputs = List::new();
        inputs.push_front(TxIn::new(
            "prev_txid".to_string(),
            0,
            "script_sig".to_string(),
            0xffffffff,
        ));

        let mut outputs = List::new();
        outputs.push_front(TxOut::new("script_pubkey".to_string(), 50_000_000));

        let tx = Transaction::new(inputs, outputs);
        assert!(!tx.txid.is_empty());
        assert_eq!(tx.inputs.iter().count(), 1);
        assert_eq!(tx.outputs.iter().count(), 1);
    }

    #[test]
    fn test_block() {
        let block = Block::new(
            0,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            1231006505,
            "merkle_root".to_string(),
            2083236893,
        );

        assert!(!block.hash.is_empty());
        assert_eq!(block.height, 0);
        assert_eq!(
            block.prev_block_hash,
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(block.timestamp, 1231006505);
        assert_eq!(block.merkle_root, "merkle_root");
        assert_eq!(block.nonce, 2083236893);
    }

    #[test]
    fn test_block_add_transaction() {
        let mut block = Block::new(
            0,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            1231006505,
            "merkle_root".to_string(),
            2083236893,
        );

        let tx = Transaction::new(List::new(), List::new());
        let result = block.add_transaction(tx);
        assert!(result.is_ok());
        assert_eq!(block.transactions.iter().count(), 1);
    }

    #[test]
    fn test_blockchain() {
        let mut blockchain = BlockChain::new();
        assert_eq!(blockchain.get_block_count(), 0);

        let genesis_block = Block::new(
            0,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            1231006505,
            "merkle_root".to_string(),
            2083236893,
        );
        let result = blockchain.add_block(genesis_block);
        assert!(result.is_ok());
        assert_eq!(blockchain.get_block_count(), 1);

        let best_hash = blockchain.get_best_block_hash();
        assert!(best_hash.is_some());

        let block_by_height = blockchain.get_block_by_height(0);
        assert!(block_by_height.is_some());

        let block_by_hash = blockchain.get_block_by_hash(&best_hash.unwrap());
        assert!(block_by_hash.is_some());
    }

    #[test]
    fn test_blockchain_add_multiple_blocks() {
        let mut blockchain = BlockChain::new();

        // Add genesis block
        let genesis_block = Block::new(
            0,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            1231006505,
            "merkle_root_0".to_string(),
            2083236893,
        );
        blockchain.add_block(genesis_block).unwrap();

        // Add more blocks
        for i in 1..5 {
            let prev_hash = blockchain.get_best_block_hash().unwrap();
            let block = Block::new(
                i,
                prev_hash,
                1231006505 + i * 600,
                format!("merkle_root_{}", i),
                2083236893 + i as u32,
            );
            blockchain.add_block(block).unwrap();
        }

        assert_eq!(blockchain.get_block_count(), 5);

        // Check if we can retrieve all blocks
        for i in 0..5 {
            let block = blockchain.get_block_by_height(i);
            assert!(block.is_some());
            assert_eq!(block.unwrap().height, i as u64);
        }
    }

    #[test]
    fn test_blockchain_invalid_block() {
        let mut blockchain = BlockChain::new();

        // Add genesis block
        let genesis_block = Block::new(
            0,
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            1231006505,
            "merkle_root_0".to_string(),
            2083236893,
        );
        blockchain.add_block(genesis_block).unwrap();

        // Try to add an invalid block (wrong previous hash)
        let invalid_block = Block::new(
            1,
            "invalid_previous_hash".to_string(),
            1231006505 + 600,
            "merkle_root_1".to_string(),
            2083236894,
        );
        let result = blockchain.add_block(invalid_block);
        assert!(result.is_err());
        assert_eq!(blockchain.get_block_count(), 1);
    }
}
