use bitcoin::blockdata::transaction::{OutPoint, TxIn, TxOut, Transaction};
use bitcoin::util::address::Address;
use bitcoin::util::hash::BitcoinHash;
use bitcoin::network::constants::Network;

fn main() {
    // Set up inputs and outputs for the transaction
    let input = TxIn {
        previous_output: OutPoint::null(),
        script_sig: Default::default(),
        sequence: 0xFFFFFFFF,
        witness: vec![],
    };
    let output1 = TxOut {
        value: 1000000,
        script_pubkey: Address::p2pkh(
            &PublicKey::from_str("022d2d5ef85b5e966a5a5f9b25c903b20aa1c2af8593df937ee661f20d7b6ed390")
                .unwrap().to_pubkeyhash(),
            Network::Testnet,
        ).unwrap().script_pubkey(),
    };
    let output2 = TxOut {
        value: 9000000,
        script_pubkey: Address::p2pkh(
            &PublicKey::from_str("03c329b1f8b14f9c3174f75cfb60925a8a8fb35c4a554adcd2f4b3d3be2ce9e7d9")
                .unwrap().to_pubkeyhash(),
            Network::Testnet,
        ).unwrap().script_pubkey(),
    };
    let tx = Transaction {
        version: 1,
        lock_time: 0,
        input: vec![input],
        output: vec![output1, output2],
    };
    // Calculate the transaction hash
    let txid = tx.bitcoin_hash();
    // Print the transaction hash
    println!("{}", txid);
}
