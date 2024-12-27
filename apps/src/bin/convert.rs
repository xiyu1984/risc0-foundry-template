use apps::types::proof_type::R0G16ProofData;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    #[clap(long)]
    pub receipt: String,

    #[clap(long, default_value = "zkp")]
    pub filetype: String
}

const PROOF_PATH: &str = "./proofs";

pub fn main() {
    // Parse the command line arguments.
    let args = ProveArgs::parse();

    let on_chain_g16 = match args.filetype.as_str() {
        "json" => {
            R0G16ProofData::from_saved_receipt(&format!("{}/{}", PROOF_PATH, args.receipt))
        },
        "zkp" => {
            R0G16ProofData::from_saved_bin(&format!("{}/{}", PROOF_PATH, args.receipt))
        },
        _ => {
            panic!("Invalid file type");
        }
    }; 

    on_chain_g16.save_to_local(&format!("{}/on-chain-{}.json", PROOF_PATH, args.receipt));
}