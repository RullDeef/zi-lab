use std::{
    env,
    io::{Read, Write},
};

use num_primes::BigUint;

pub mod rsa;

fn cmd_gen_keys(bits: usize, key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keys = rsa::gen_key_pair(bits);
    rsa::save_key(keys.public, &format!("rsa_{key_name}.pub"))?;
    rsa::save_key(keys.private, &format!("rsa_{key_name}.prv"))?;
    Ok(())
}

fn cmd_cipher(key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("rsa_{key_name}.pub");
    let key = rsa::load_key(&path)?;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;
    let buf = BigUint::from_bytes_be(&buf);
    let buf = rsa::cipher(key, buf)?;
    std::io::stdout().write(&buf.to_bytes_be())?;
    Ok(())
}

fn cmd_decipher(key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("rsa_{key_name}.prv");
    let key = rsa::load_key(&path)?;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;
    let buf = BigUint::from_bytes_be(&buf);
    let buf = rsa::cipher(key, buf)?;
    std::io::stdout().write(&buf.to_bytes_be())?;
    Ok(())
}

/// rsa -g|--gen <bits> <key_name>
/// rsa -c|--cipher <key_name> < input > output
/// rsa -d|--decipher <key_name> < input > output
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 {
        if args[1] == "--gen" || args[1] == "-g" {
            let bits = args[2].parse::<usize>().expect("parse");
            cmd_gen_keys(bits, &args[3]).expect("cmd_gen_keys");
        } else {
            println!(r#"
app -g|--gen <bits> <key_name>
app -c|--cipher <key_name> < input > output
app -d|--decipher <key_name> < input > output"#);
        }
    } else if args.len() == 3 {
        if args[1] == "--cipher" || args[1] == "-c" {
            cmd_cipher(&args[2]).expect("cmd_cipher");
        } else if args[1] == "--decipher" || args[1] == "-d" {
            cmd_decipher(&args[2]).expect("cmd_decipher");
        } else {
            println!(r#"
app -g|--gen <bits> <key_name>
app -c|--cipher <key_name> < input > output
app -d|--decipher <key_name> < input > output"#);
        }
    } else {
        println!(r#"
app -g|--gen <bits> <key_name>
app -c|--cipher <key_name> < input > output
app -d|--decipher <key_name> < input > output"#);
    }
}
