use crate::bytes::{ByteIndexable, WordIndexable};

fn sbox(byte: u8) -> u8 {
    const MATRIX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
    MATRIX[byte as usize]
}

fn sbox_inv(byte: u8) -> u8 {
    const MATRIX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
        0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
        0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
        0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
        0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
        0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
        0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
        0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
        0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
        0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
        0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
        0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ];
    MATRIX[byte as usize]
}

fn rcon(i: usize) -> u8 {
    const MATRIX: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

    MATRIX[i]
}

fn gmul_byte(mut a: u8, mut b: u8) -> u8 {
    let mut res = 0;
    for _ in 0..8 {
        if b & 1 == 1 {
            res ^= a;
        }
        let hi_bit_set = a & 0x80;
        a <<= 1;
        if hi_bit_set == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    res
}

fn gdot(a: u32, b: u32) -> u8 {
    let mut res = 0;
    res ^= gmul_byte(a.get_byte(0), b.get_byte(0));
    res ^= gmul_byte(a.get_byte(1), b.get_byte(1));
    res ^= gmul_byte(a.get_byte(2), b.get_byte(2));
    res ^= gmul_byte(a.get_byte(3), b.get_byte(3));
    res
}

fn sub_bytes<T: ByteIndexable>(mut val: T) -> T {
    for i in 0..(std::mem::size_of::<T>() as u8) {
        let byte = val.get_byte(i);
        let byte = sbox(byte);
        val = val.set_byte(i, byte);
    }
    val
}

fn inv_sub_bytes<T: ByteIndexable>(mut val: T) -> T {
    for i in 0..(std::mem::size_of::<T>() as u8) {
        let byte = val.get_byte(i);
        let byte = sbox_inv(byte);
        val = val.set_byte(i, byte);
    }
    val
}

fn shift_rows(val: u128) -> u128 {
    let mut res = val;

    res = res.set_byte(00 + 1, val.get_byte(04 + 1));
    res = res.set_byte(04 + 1, val.get_byte(08 + 1));
    res = res.set_byte(08 + 1, val.get_byte(12 + 1));
    res = res.set_byte(12 + 1, val.get_byte(00 + 1));

    res = res.set_byte(00 + 2, val.get_byte(08 + 2));
    res = res.set_byte(04 + 2, val.get_byte(12 + 2));
    res = res.set_byte(08 + 2, val.get_byte(00 + 2));
    res = res.set_byte(12 + 2, val.get_byte(04 + 2));
    
    res = res.set_byte(00 + 3, val.get_byte(12 + 3));
    res = res.set_byte(04 + 3, val.get_byte(00 + 3));
    res = res.set_byte(08 + 3, val.get_byte(04 + 3));
    res = res.set_byte(12 + 3, val.get_byte(08 + 3));
    
    res
}

fn inv_shift_rows(val: u128) -> u128 {
    let mut res = val;
    
    res = res.set_byte(00 + 1, val.get_byte(12 + 1));
    res = res.set_byte(04 + 1, val.get_byte(00 + 1));
    res = res.set_byte(08 + 1, val.get_byte(04 + 1));
    res = res.set_byte(12 + 1, val.get_byte(08 + 1));

    res = res.set_byte(00 + 2, val.get_byte(08 + 2));
    res = res.set_byte(04 + 2, val.get_byte(12 + 2));
    res = res.set_byte(08 + 2, val.get_byte(00 + 2));
    res = res.set_byte(12 + 2, val.get_byte(04 + 2));
    
    res = res.set_byte(00 + 3, val.get_byte(04 + 3));
    res = res.set_byte(04 + 3, val.get_byte(08 + 3));
    res = res.set_byte(08 + 3, val.get_byte(12 + 3));
    res = res.set_byte(12 + 3, val.get_byte(00 + 3));

    res
}

fn mix_columns(mut val: u128) -> u128 {
    for i in 0..4 {
        let word = val.get_word(i);
        let mut res = 0;

        res = res.set_byte(0, gdot(word, 0x01_01_03_02));
        res = res.set_byte(1, gdot(word, 0x01_03_02_01));
        res = res.set_byte(2, gdot(word, 0x03_02_01_01));
        res = res.set_byte(3, gdot(word, 0x02_01_01_03));

        val = val.set_word(i, res);
    }
    val
}

fn inv_mix_columns(mut val: u128) -> u128 {
    for i in 0..4 {
        let word = val.get_word(i);
        let mut res = 0;

        res = res.set_byte(0, gdot(word, 0x09_0D_0B_0E));
        res = res.set_byte(1, gdot(word, 0x0D_0B_0E_09));
        res = res.set_byte(2, gdot(word, 0x0B_0E_09_0D));
        res = res.set_byte(3, gdot(word, 0x0E_09_0D_0B));

        val = val.set_word(i, res);
    }
    val
}

fn schedule_core(mut val: u32, i: usize) -> u32 {
    val = val.rotate_right(8);
    val = sub_bytes(val);
    val.set_byte(0, val.get_byte(0) ^ rcon(i))
}

fn expand_key(key: u128) -> [u128; 11] {
    let mut w = [0u128; 11];
    w[0] = key;

    for i in 0..10 {
        let w_prev_0 = w[i].get_word(0);
        let w_prev_1 = w[i].get_word(1);
        let w_prev_2 = w[i].get_word(2);
        let w_prev_3 = w[i].get_word(3);

        let w_new_0 = w_prev_0 ^ schedule_core(w_prev_3, i);
        let w_new_1 = w_prev_1 ^ w_new_0;
        let w_new_2 = w_prev_2 ^ w_new_1;
        let w_new_3 = w_prev_3 ^ w_new_2;

        w[i + 1] = 0u128
            .set_word(0, w_new_0)
            .set_word(1, w_new_1)
            .set_word(2, w_new_2)
            .set_word(3, w_new_3);
    }

    w
}

fn cipher_block(keys: [u128; 11], mut block: u128) -> u128 {
    block ^= keys[0];

    for i in 1..=9 {
        block = sub_bytes(block);
        block = shift_rows(block);
        block = mix_columns(block);
        block ^= keys[i];
    }
    
    block = sub_bytes(block);
    block = shift_rows(block);
    block ^ keys[10]
}

fn decipher_block(keys: [u128; 11], mut block: u128) -> u128 {
    block ^= keys[10];
    block = inv_shift_rows(block);
    block = inv_sub_bytes(block);

    for i in (1..=9).rev() {
        block ^= keys[i];
        block = inv_mix_columns(block);
        block = inv_shift_rows(block);
        block = inv_sub_bytes(block);
    }

    block ^ keys[0]
}

pub fn cipher_msg(key: u128, data: u128) -> u128 {
    let keys = expand_key(key);
    cipher_block(keys, data)
}

pub fn decipher_msg(key: u128, data: u128) -> u128 {
    let keys = expand_key(key);
    decipher_block(keys, data)
}

pub fn cipher_vec(key: u128, data: &Vec<u128>) -> Vec<u128> {
    let keys = expand_key(key);
    let mut res = Vec::new();

    for block in data {
        res.push(cipher_block(keys, *block));
    }

    res
}

pub fn decipher_vec(key: u128, data: &Vec<u128>) -> Vec<u128> {
    let keys = expand_key(key);
    let mut res = Vec::new();

    for block in data {
        res.push(decipher_block(keys, *block));
    }

    res
}

pub fn cipher_bytes(key: u128, data: &Vec<u8>) -> Vec<u8> {
    let keys = expand_key(key);
    let mut res = Vec::new();
    
    let mut i = 0;
    while i < data.len() - 15 {
        let mut block = 0;
        for j in 0..16 {
            block = block.set_byte(j, data[i + j as usize]);
        }
        block = cipher_block(keys, block);
        for j in 0..16 {
            res.push(block.get_byte(j));
        }
        i += 16;
    }

    let mut last_block = 0;
    let mut padding = 16;
    for j in 0..(data.len() - i) as u8 {
        last_block = last_block.set_byte(j, data[i + j as usize]);
        padding -= 1;
    }
    last_block = cipher_block(keys, last_block);
    for j in 0..16 {
        res.push(last_block.get_byte(j));
    }

    res.push(padding);
    res
}

pub fn decipher_bytes(key: u128, data: &Vec<u8>) -> Vec<u8> {
    let keys = expand_key(key);
    let mut res = Vec::new();

    let mut i = 0;
    while i + 1 < data.len() {
        let mut block = 0;
        for j in 0..16 {
            block = block.set_byte(j, data[i + j as usize]);
        }
        block = decipher_block(keys, block);
        for j in 0..16 {
            res.push(block.get_byte(j));
        }
        i += 16;
    }

    let dflt = 0u8;
    let padding = *data.last().unwrap_or(&dflt);

    for _ in 0..padding {
        res.pop();
    }

    res
}
