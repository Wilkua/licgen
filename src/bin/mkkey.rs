use std::process;
use std::fs::File;
use std::io::Write;

use base64;
use sodiumoxide::crypto::sign;

fn main() {
    let (pk, sk) = sign::gen_keypair();

    let pkfile_name = String::from("my_key.pubkey");
    let mut pkfile = match File::create(&pkfile_name) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Failed to open public key file {}", &pkfile_name);
            process::exit(1);
        },
    };
    let encoded_data = base64::encode_config(&pk[..], base64::STANDARD);
    pkfile.write_all(&encoded_data.into_bytes());

    let skfile_name = "my_key.pem";
    let mut skfile = match File::create(&skfile_name) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Failed to open private key file {}", &skfile_name);
            process::exit(1);
        },
    };
    let encoded_data = base64::encode_config(&sk[..], base64::STANDARD);
    skfile.write_all(&encoded_data.into_bytes());
}
