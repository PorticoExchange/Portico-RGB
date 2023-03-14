use bitcoin::util::address::Address;
use bitcoin::util::key::PublicKey;
use bitcoin::util::ecdsa::PublicKey as EcdsaPublicKey;
use bitcoin::blockdata::opcodes::all::*;
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::network::constants::Network;
use bitcoin::util::hash::BitcoinHash;
use bitcoin::util::bip143::SighashComponents;
use bitcoin::util::bip143::SigHashCache;
use bitcoin::util::bip32::{ChildNumber, ExtendedPrivKey};
use bitcoin::util::psbt::{serialize::Serialize, PartiallySignedTransaction};
use secp256k1::{Secp256k1, All};
use rgb::contract::ContractId;
use rgb::contract::Offer;
use rgb::contract::OfferPayload;
use rgb::contract::Payment;
use rgb::contract::PaymentPayload;
use rgb::contract::PublicRight;
use rgb::contract::Schema;
use rgb::contract::TransitionType;
use rgb::encode::bitcoin::Encode;
use rgb::encode::Extension;
use rgb::prelude::*;
use rgb::traits::NeededTx;
use rgb::util::tx::PartialTx;
use std::collections::HashMap;

fn main() {
    // Set up a Bitcoin transaction with an output to the RGB contract address
    let contract_address = Address::from_str("mzK3P2Q5Fh9JX6mthP6RUtjRzwZ1rb6U5P", Network::Testnet).unwrap();
    let contract_output_value = 10000; // in satoshis
    let txout = bitcoin::TxOut {
        value: contract_output_value,
        script_pubkey: contract_address.script_pubkey(),
    };
    let mut tx = bitcoin::Transaction {
        version: 2,
        lock_time: 0,
        input: vec![],
        output: vec![txout],
    };

    // Set up the RGB contract
    let contract_id = ContractId::from_str("78d8b560e0f16d221f62c1c70e8c0ddfec6a965e747aafbe6e8d21c2a46647d1").unwrap();
    let contract_schema= CntractID::from_str("").unwrap();

   
