use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use num::Zero;
use num_primes::{BigUint, Generator};

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    pub val: BigUint,
    pub n: BigUint,
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public: Key,
    pub private: Key,
}

fn mod_exp(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    let mut result = one.clone();
    let mut base = &base % &modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= zero {
            break;
        }

        if &exponent % &two == one {
            result = (result * &base) % &modulus;
        }

        exponent = exponent >> 1;
        base = (&base * &base) % &modulus;
    }

    result
}

pub fn gen_key_pair(bits: usize) -> KeyPair {
    let p = Generator::new_prime(bits / 2);
    let q = Generator::new_prime(bits / 2);

    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);

    let mut e = BigUint::from(65537u32);
    while num::integer::gcd(e.clone(), phi.clone()) > BigUint::from(1u32) {
        e += 2u32;
    }

    let mut k = 1u32;
    while !((k * &phi + 1u32) % &e).is_zero() {
        k += 1;
    }

    let d = (k * &phi + 1u32) / &e;

    KeyPair {
        public: Key {
            val: e,
            n: n.clone(),
        },
        private: Key {
            val: d,
            n: n.clone(),
        },
    }
}

pub fn cipher(key: Key, data: BigUint) -> Result<BigUint, String> {
    if data >= key.n {
        Err(String::from("data must be less than n"))
    } else {
        Ok(mod_exp(data, key.val, key.n))
    }
}

pub fn save_key(key: Key, path: &str) -> io::Result<()> {
    let mut file = File::create(path).unwrap();
    file.write(format!("{}\n{}", key.val, key.n).as_bytes())?;
    Ok(())
}

pub fn load_key(path: &str) -> Result<Key, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut res = Key {
        val: BigUint::from(0u32),
        n: BigUint::from(0u32),
    };

    let mut lines = BufReader::new(file).lines();
    if let Some(Ok(line)) = lines.next() {
        res.val = BigUint::parse_bytes(&line.as_bytes(), 10).unwrap();
        if let Some(Ok(line)) = lines.next() {
            res.n = BigUint::parse_bytes(&line.as_bytes(), 10).unwrap();
            Ok(res)
        } else {
            Err(Box::new(Error { msg: String::from("invalid key file content") }))
        }
    } else {
        Err(Box::new(Error { msg: String::from("invalid key file content") }))
    }
}
