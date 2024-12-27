use clap::Parser;

pub fn get_image_id_hex(image_id: [u32; 8]) -> String {
    let digest = risc0_zkp::core::digest::Digest::new(image_id);

    hex::encode(digest)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliData {
    #[clap(long)]
    pub img_path: String,
}

pub fn main() {
    env_logger::init();

    let args = CliData::parse();

    let image_id_bytes = std::fs::read(args.img_path).unwrap();
    let image_id: [u32; 8] = serde_json::from_slice(&image_id_bytes).unwrap();

    log::info!("the image id is: 0x{}", get_image_id_hex(image_id));
}