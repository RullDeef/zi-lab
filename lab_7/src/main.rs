pub mod lzw;

use std::io::{Read, Write};

use clap::ArgGroup;
use clap::Parser;

#[derive(Parser)]
#[command(group(ArgGroup::new("action").required(true).args(["encode", "decode"])))]
struct CliArgs {
    #[arg(short = 'e', long = "encode")]
    encode: bool,

    #[arg(short = 'd', long = "decode")]
    decode: bool,
}

fn cmd_encode() -> std::io::Result<()> {
    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes)?;
    let bytes = lzw::encode_bytes(&bytes);
    std::io::stdout().write(&bytes)?;
    Ok(())
}

fn cmd_decode() -> std::io::Result<()> {
    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes)?;
    let bytes = lzw::decode_bytes(bytes);
    std::io::stdout().write(&bytes)?;
    Ok(())
}

/// lzw -e|--encode < input > output
/// lzw -d|--decode < input > output
fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    if args.encode {
        cmd_encode()
    } else {
        cmd_decode()
    }
}
