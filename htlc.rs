use bitcoin::util::hash::BitcoinHash;
use rgb::{self, fungible::Amount, schema::constants::*, schema::scripts::*, util::Value};
use secp256k1::rand::rngs::OsRng;
use std::str::FromStr;

fn create_htlc_input_output() {
    // Define the HTLC parameters
    let preimage = [0x01; 32];
    let hash = bitcoin::hashes::sha256::Hash::hash(&preimage);
    let value = 1000;
    let timelock = 100;

    // Create the HTLC input
    let htlc_input = HTLCOutput {
        value,
        timelock,
        script: HTLC_SCRIPT.clone(),
        hash: HashLock::from_inner(hash.into_inner()),
        nonce: Nonce::from_inner([0; 32]),
        outpoint: OutPoint::from_str("deadbeef").unwrap(),
    };

    // Create the HTLC output
    let mut htlc_output = HTLCOutput {
        value,
        timelock,
        script: HTLC_SCRIPT.clone(),
        hash: HashLock::from_inner(hash.into_inner()),
        nonce: Nonce::from_inner(OsRng.gen::<[u8; 32]>()),
        outpoint: OutPoint::default(),
    };
    htlc_output.set_vout(0);

    // Create RGB Core input and output
    let htlc_input_value = Value::from_sat(value as u64);
    let htlc_output_value = Value::from_sat(value as u64);
    let htlc_asset_id = AssetId::from_str("HTLC").unwrap();
    let htlc_input_entry = Entry {
        commitment: None,
        proof: None,
        data: Data::Fungible(FungibleData::new(
            Amount::from_value(htlc_input_value, htlc_asset_id).unwrap(),
            vec![],
        )),
    };
    let htlc_output_entry = Entry {
        commitment: None,
        proof: None,
        data: Data::Fungible(FungibleData::new(
            Amount::from_value(htlc_output_value, htlc_asset_id).unwrap(),
            vec![htlc_output.serialize().to_vec()],
        )),
    };
}
