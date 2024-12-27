use risc0_ethereum_contracts::encode_seal;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct R0G16ProofData {
    pub seal: String,
    pub journal: String
}

impl R0G16ProofData {
    // pub fn from_receipt(receipt: &Receipt) -> Self {
    //     let seal: String = format!("0x{}", hex::encode(encode_seal(receipt).unwrap()));
    //     let journal: String = format!("0x{}", hex::encode(receipt.journal.clone()));

    //     R0G16ProofData {
    //         seal,
    //         journal
    //     }
    // }

    pub fn from_saved_bin(path: &String) -> Self {
        let receipt_bytes = std::fs::read(path).expect("Read saved file error");
        let receipt = bincode::deserialize(&receipt_bytes).expect("deserilize error");
        let seal: String = format!("0x{}", hex::encode(encode_seal(&receipt).unwrap()));
        let journal: String = format!("0x{}", hex::encode(receipt.journal.clone()));

        R0G16ProofData {
            seal,
            journal
        }
    }

    pub fn from_saved_receipt(path: &String) -> Self {
        let receipt_bytes = std::fs::read(path).expect("Read saved file error");
        let receipt = serde_json::from_slice(&receipt_bytes).expect("deserilize error");
        let seal: String = format!("0x{}", hex::encode(encode_seal(&receipt).unwrap()));
        let journal: String = format!("0x{}", hex::encode(receipt.journal.clone()));

        R0G16ProofData {
            seal,
            journal
        }
    }

    pub fn save_to_local(&self, path: &String) {
        let on_chain_g16 = serde_json::to_string(self).expect("serilize on chain g16 error");
        std::fs::write(path, on_chain_g16).expect("write to local error");
    }

    pub fn load_from_local(path: &String) -> Self {
        let r0g16_bytes = std::fs::read(path).expect("read from local error");
        serde_json::from_slice(&r0g16_bytes).expect("deserilize error")
    }
}