// fn main() {
//     println!("Hello, world!");
// }

use bdk::bitcoin::{Script, TxOut};
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex::New;
use bdk::{Wallet, SyncOptions};
use bdk::bitcoin::util::psbt::PartiallySignedTransaction;
use bdk::bitcoin::consensus::encode::serialize;
use bdk::blockchain::Blockchain;
use hex;

fn display_explorer_links(txid: &str) {
    println!("\nTransaction Explorer Links:");
    println!("- Mempool: https://mempool.space/testnet/tx/{}", txid);
    println!("- Blockstream: https://blockstream.info/testnet/tx/{}", txid);
    println!("- BlockCypher: https://live.blockcypher.com/btc-testnet/tx/{}/", txid);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the wallet
    let wallet = Wallet::new(
        "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)", // Replace with your descriptor
        // this is a  sample one and needs to be replaced with the actual one
        None,
        bdk::bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;

    // Get a new address
    let address = wallet.get_address(New)?;
    println!("New receiving address: {}", address.to_string());
    
    // Sync the wallet to get the latest balance
    let blockchain = ElectrumBlockchain::from(
        bdk::electrum_client::Client::new("ssl://electrum.blockstream.info:60002")?,
    );
    wallet.sync(&blockchain, SyncOptions::default())?;

    // Get and display the wallet balance
    let balance = wallet.get_balance()?;
    println!("Wallet balance: {} satoshis", balance.get_total());

    // Construct the Runes payload
    let runes_payload = vec![0x72, 0x01, 0x6d, 0x4d, 0x4f, 0x4f, 0x4e]; // 'r', 0x01, 'm', 'M', 'O', 'O', 'N'

    // Create the OP_RETURN script
    let op_return_script = Script::new_op_return(&runes_payload);

    // Define the OP_RETURN output with zero value
    let op_return_output = TxOut {
        value: 0,
        script_pubkey: op_return_script,
    };

    // Build the transaction
    let mut builder = wallet.build_tx();
    builder
        .add_recipient(op_return_output.script_pubkey.clone(), op_return_output.value)
        .fee_rate(bdk::FeeRate::from_sat_per_vb(1.0));

    let (mut psbt, _) = builder.finish()?;

    // Sign the transaction
    wallet.sign(&mut psbt, bdk::SignOptions::default())?;

    // Extract the final transaction
    let final_tx = psbt.extract_tx();
    let txid = final_tx.txid().to_string();
    
    // Broadcast the transaction
    println!("Broadcasting transaction...");
    blockchain.broadcast(&final_tx)?;
    println!("Transaction successfully broadcast!");
    
    println!("\nTransaction ID: {}", txid);
    display_explorer_links(&txid);
    
    println!("\nRaw transaction: {}", hex::encode(serialize(&final_tx)));

    Ok(())
}
