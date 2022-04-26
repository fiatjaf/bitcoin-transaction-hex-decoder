use hex;
use sha2::{Digest, Sha256};
#[path = "./bytestream.rs"]
mod bytestream;
#[path = "./transaction.rs"]
mod transaction;

// NOTE: missing weight, script parsing

// Turns transaction hex into an object with readable fields
pub fn decode_from_hex(transaction_hex_string: &str) -> transaction::Content {
    let mut tx = bytestream::Bytestream::new(transaction_hex_string);
    let hash = Sha256::digest(Sha256::digest(hex::decode(transaction_hex_string).unwrap()));
    let txid = bytestream::Bytestream::convert_endian(&hex::encode(hash));
    let version = bytestream::Bytestream::bytes_to_u64(&tx.get_bytes(4, true));
    let no_of_inputs = tx.get_varint();
    let mut inputs = vec![];
    for _ in 0..no_of_inputs {
        let txid = hex::encode(tx.get_bytes(32, true));
        let vout = bytestream::Bytestream::bytes_to_u64(&tx.get_bytes(4, true));
        let scriptsig_size = tx.get_varint();
        let scriptsig = hex::encode(tx.get_bytes(scriptsig_size, false));
        let sequence = bytestream::Bytestream::bytes_to_u64(&tx.get_bytes(4, true));
        inputs.push(transaction::Input {
            txid,
            vout,
            scriptsig,
            sequence,
        });
    }
    let no_of_outputs = tx.get_varint();
    let mut outputs = vec![];
    for _ in 0..no_of_outputs {
        let value = bytestream::Bytestream::bytes_to_u64(&tx.get_bytes(8, true));
        let scriptpubkey_size = tx.get_varint();
        let scriptpubkey = hex::encode(tx.get_bytes(scriptpubkey_size, false));
        outputs.push(transaction::Output {
            value,
            scriptpubkey,
        });
    }
    let locktime = bytestream::Bytestream::bytes_to_u64(&tx.get_bytes(4, true));
    return transaction::Content {
        txid,
        version,
        locktime,
        inputs,
        outputs,
    };
}
