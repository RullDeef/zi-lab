#![allow(dead_code)]

use std::{
    env,
    io::{Read, Write},
};

use num_primes::BigUint;
use sha2::{digest::Output, Digest, Sha256};

pub mod aes;
pub mod rsa;
mod euclid;

fn get_input_hash() -> Result<Output<Sha256>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;
    let mut sha_hash = Sha256::new();
    sha_hash.update(buf);
    Ok(sha_hash.finalize())
}

fn load_digital_sign(path: &str) -> Result<BigUint, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let bytes: Vec<u8> = file.bytes().map(Result::unwrap).collect();
    Ok(BigUint::from_bytes_be(&bytes))
}

fn cmd_gen_keys(bits: usize, key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keys = rsa::gen_key_pair(bits);
    rsa::save_key(keys.public, &format!("rsa_{key_name}.pub"))?;
    rsa::save_key(keys.private, &format!("rsa_{key_name}.prv"))?;
    Ok(())
}

fn cmd_sign(key_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("rsa_{key_name}.prv");
    let key = rsa::load_key(&path)?;
    let hash = BigUint::from_bytes_be(&get_input_hash()?);
    let buf = rsa::cipher(key, hash)?;
    std::io::stdout().write(&buf.to_bytes_be())?;
    Ok(())
}

fn cmd_verify(key_name: &str, sign_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("rsa_{key_name}.pub");
    let key = rsa::load_key(&path)?;
    let hash = BigUint::from_bytes_be(&get_input_hash()?);
    let sign = load_digital_sign(sign_path)?;
    let sign = rsa::cipher(key, sign)?;

    if hash == sign {
        println!("digital signature is valid");
    } else {
        println!("digital signature is invalid. Document was modified!");
    }

    Ok(())
}

/// rsa -g|--gen <bits> <key_name>
/// rsa -s|--sign <key_name> < input > output.sgn
/// rsa -v|--verify <key_name> <sng_file> < input
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 && (args[1] == "--gen" || args[1] == "-g") {
        let bits = args[2].parse::<usize>().expect("parse");
        cmd_gen_keys(bits, &args[3]).expect("cmd_gen_keys");
    } else if args.len() == 3 && (args[1] == "--sign" || args[1] == "-s") {
        cmd_sign(&args[2]).expect("cmd_sign");
    } else if args.len() == 4 && (args[1] == "--verify" || args[1] == "-v") {
        cmd_verify(&args[2], &args[3]).expect("cmd_verify");
    } else {
        panic!("invalid args");
    }
}
