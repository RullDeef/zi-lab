use md5;
use std::env;
use std::io;

pub mod aes;
pub mod bytes;
pub mod prettify;
pub mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!(
            "usage: {0} [--cipher|--decipher] key [< input_file] [> output_file]",
            args[0]
        );
    } else if args[1] == "--cipher" {
        let md5sum = md5::compute(&args[2]);
        let key = bytes::pack_bytes(&md5sum.0);
        cipher_input_data(key);
    } else if args[1] == "--decipher" {
        let md5sum = md5::compute(&args[2]);
        let key = bytes::pack_bytes(&md5sum.0);
        decipher_input_data(key);
    } else {
        println!(
            "invalid option: \"{0}\". Must be --cipher or --decipher",
            args[1]
        );
    }
}

fn cipher_input_data(key: u128) {
    let mut bytes = vec![];

    for byte in io::Read::bytes(io::stdin()) {
        bytes.push(byte.unwrap());
    }

    let bytes = aes::cipher_bytes(key, &bytes);
    io::Write::write(&mut std::io::stdout(), &bytes).unwrap();
}

fn decipher_input_data(key: u128) {
    let mut bytes = vec![];

    for byte in io::Read::bytes(io::stdin()) {
        bytes.push(byte.unwrap());
    }

    let bytes = aes::decipher_bytes(key, &bytes);
    io::Write::write(&mut std::io::stdout(), &bytes).unwrap();
}
