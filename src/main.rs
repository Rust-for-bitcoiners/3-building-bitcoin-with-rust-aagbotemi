use a_3::block::*;
use a_3::linked_list::LinkedList;

fn main() {
    let mut blockchain = BlockChain::new();

    // Create and add the genesis block
    let genesis_block = Block::new(
        0,
        String::from("0000000000000000000000000000000000000000000000000000000000000000"),
        1231006505,
        String::from("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"),
        2083236893,
    );
    blockchain.add_block(genesis_block).unwrap();

    // Add some transactions and blocks
    for i in 1..10 {
        let mut block = Block::new(
            i,
            blockchain.get_best_block_hash().unwrap(),
            1231006505 + i * 600,
            format!("merkle_root_{}", i),
            (2083236893 + i).try_into().unwrap(),
        );

        let tx = Transaction::new(
            LinkedList::new(), // For simplicity, we're not adding inputs
            {
                let mut outputs = LinkedList::new();
                outputs.push_front(TxOut::new(format!("pubkey_{}", i), 50 * 100000000));
                outputs
            },
        );

        block.add_transaction(tx).unwrap();
        blockchain.add_block(block).unwrap();
    }

    // Example usage of blockchain functions
    println!("Block count: {}", blockchain.get_block_count());
    println!("Best block hash: {:?}", blockchain.get_best_block_hash());
}
