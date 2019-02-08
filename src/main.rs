use std::{env, process, time};

#[derive(Debug)]
struct Feature {
    id: u16,
    valid_from: u64,
    valid_thru: u64,
}

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

    println!("{:?}", features);
}

