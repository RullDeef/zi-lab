mod compressor;
mod extractor;
mod phrasedict;

use self::{extractor::Extractor, phrasedict::PhrasesDict};
use compressor::Compressor;
#[allow(unused_imports)]
use rand::Rng;

const BITS_PER_PHRASE: usize = 10;

pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut phrase_dict = PhrasesDict::new(BITS_PER_PHRASE);
    let mut compressor = Compressor::new();

    let mut phrase = vec![];
    for byte in bytes {
        phrase.push(*byte);

        if phrase_dict.lookup(&phrase).is_none() {
            match phrase_dict.lookup(&phrase[0..phrase.len() - 1]) {
                Some(code) => {
                    compressor.feed_bits(code);
                }
                None => {
                    panic!("Oops, must not occur...");
                }
            }

            phrase_dict.add(phrase);
            phrase = vec![*byte];
        }
    }

    // last phrase
    match phrase_dict.lookup(&phrase) {
        Some(code) => {
            compressor.feed_bits(code);
        }
        None => {
            panic!("Oops, must not occur...");
        }
    }

    compressor.get_bytes()
}

pub fn decode_bytes(bytes: Vec<u8>) -> Vec<u8> {
    let mut phrase_dict = PhrasesDict::new(BITS_PER_PHRASE);
    let mut extractor = Extractor::new(bytes);
    let mut result = Vec::new();

    let mut old_code = extractor.drain(BITS_PER_PHRASE);

    let mut old_string;
    #[allow(unused_assignments)]
    let mut char = 0;

    match phrase_dict.get_phrase(old_code.clone()) {
        Some(phrase) => {
            let mut phrase = phrase.clone();
            old_string = phrase.clone();
            char = phrase[0];
            result.append(&mut phrase);
        }
        None => {
            panic!("Oops, bad decoding sequence: {:?}", old_code.clone());
        }
    }

    while extractor.count_left() > 0 {
        let new_code = extractor.drain(BITS_PER_PHRASE);
        let mut string;

        match phrase_dict.get_phrase(new_code.clone()) {
            Some(phrase) => {
                string = phrase.clone();
            }
            None => {
                string = phrase_dict.get_phrase(old_code.clone()).unwrap().clone();
                string.push(char);
            }
        }

        char = string[0];
        phrase_dict.add([old_string.clone(), vec![char]].concat());
        old_string = string.clone();
        old_code = new_code;

        result.append(&mut string);
    }

    result
}

#[test]
fn test_encode() {
    let bytes = "BABAABAAA".as_bytes();
    println!("bytes:   {bytes:?}");

    let enc = encode_bytes(bytes);
    println!("encoded: {enc:?}");

    let dec = decode_bytes(enc);
    println!("decoded: {dec:?}")
}

#[test]
fn test_random() {
    for _ in 0..1000 {
        let mut bytes = [0; 100];
        bytes.fill_with(|| rand::thread_rng().gen());

        let enc = encode_bytes(&bytes);
        let dec = decode_bytes(enc);

        assert_eq!(bytes.to_vec(), dec);
    }
}
