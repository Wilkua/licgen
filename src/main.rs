use std::{env, process, time};
use std::io::{self, Read, Write};

use base64;
use sodiumoxide::crypto::sign;

struct Feature {
    id: u16,
    valid_from: u64,
    valid_thru: u64,
}

const LIC_VERSION: u16 = 1;

fn main() {
    let created_time = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH);
    let created_time = match created_time {
        Ok(t) => t.as_secs(),
        Err(_) => {
            eprintln!("Fatal: Clock drifted -- try again");
            process::exit(1);
        },
    };
    let mut features: Vec<Feature> = Vec::new();
    // License files always have a blank root index
    features.push(Feature {
        id: 0,
        valid_from: created_time,
        valid_thru: created_time
    });

    let mut args = env::args();
    // skip exec path item in args
    let _ = args.next();
    for f in args {
        let f: Vec<&str> = f.split(",").collect();
        let id = match u16::from_str_radix(&f[0], 10) {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Fatal: Invalid feature ID '{}' -- must be u16", &f[0]);
                process::exit(1);
            }
        };
        let valid_from = if f.len() > 1 {
            match u64::from_str_radix(&f[1], 10) {
                Ok(vf) => vf,
                Err(_) => {
                    eprintln!("Fatal: Invalid valid from timestamp '{}' for ID '{}' -- must be u64", &f[1], id);
                    process::exit(1);
                }
            }
        } else {
            0 as u64 // we'll just default to 0 when not specified
        };
        let valid_thru = if f.len() > 2 {
            match u64::from_str_radix(&f[2], 10) {
                Ok(vt) => vt,
                Err(_) => {
                    eprintln!("Fatal: Invalid valid thru timestamp '{}' for ID '{}' -- must be u64", &f[2], id);
                    process::exit(1)
                }
            }
        } else {
            0 as u64 // we'll just default to 0 when not specified
        };
        features.push(Feature { id, valid_from, valid_thru });
    };

    features.sort_unstable_by(|a, b| a.id.cmp(&b.id));
    let feature_count = features.len();
    // Index
    //   version -> 2 bytes
    //   for n features:
    //     id         -> 2 bytes
    //     offset     -> 4 bytes
    // Entries
    //   for n features:
    //     valid_from -> 8 bytes
    //     valid_thru -> 8 bytes
    let mut lic_bytes: Vec<u8> = Vec::with_capacity((feature_count * 22) + 2);
    let _ = lic_bytes.write(&LIC_VERSION.to_le_bytes());
    for (idx, f) in features.iter().enumerate() {
        // write the ID to the index table
        let _ = lic_bytes.write(&f.id.to_le_bytes());
        // write the offset to the index table
        let _ = lic_bytes.write(&(idx as u32 * 16u32).to_le_bytes());
    }
    for f in features.iter() {
        // write timestamps to data block
        let _ = lic_bytes.write(&f.valid_from.to_le_bytes());
        let _ = lic_bytes.write(&f.valid_thru.to_le_bytes());
    }

    let mut key_data: Vec<u8> = Vec::new();
    match io::stdin().read_to_end(&mut key_data) {
        Err(_) => {
            eprintln!("Failed reading STDIN for secret key");
            process::exit(1);
        },
        _ => {},
    };
    // Knock the \r\n off the end of the byte stream
    let _ = key_data.pop();
    let _ = key_data.pop();
    let key_data = match base64::decode_config(&key_data, base64::STANDARD) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to decode key data: {}", e);
            process::exit(1);
        },
    };
    let sk = sign::SecretKey::from_slice(&key_data.as_slice());
    let sk = match sk {
        Some(k) => k,
        None => {
            eprintln!("The provided secret key was not valid");
            process::exit(1);
        },
    };
    let signed_data = sign::sign(&lic_bytes.as_slice(), &sk);
    let signed_data = base64::encode_config(&signed_data, base64::STANDARD);

    println!("{}", signed_data);
}

