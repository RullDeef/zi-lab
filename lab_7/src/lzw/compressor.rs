pub struct Compressor {
    bits: [u8; 8],
    bits_counter: usize,
    bytes: Vec<u8>,
}

impl Compressor {
    pub fn new() -> Self {
        Compressor {
            bits: [0; 8],
            bits_counter: 0,
            bytes: vec![],
        }
    }

    pub fn feed_bits(&mut self, bits: Vec<u8>) {
        for bit in bits {
            self.feed_bit(bit);
        }
    }

    pub fn feed_bit(&mut self, bit: u8) {
        if bit != 0 && bit != 1 {
            panic!("bit must be 0 or 1. Got: {bit}");
        }

        self.bits[self.bits_counter] = bit;
        self.bits_counter += 1;

        if self.bits_counter == 8 {
            let mut byte = 0;
            for i in 0..8 {
                byte |= self.bits[i] << i;
            }
            self.bytes.push(byte);
            self.bits_counter = 0;
            self.bits = [0; 8];
        }
    }

    pub fn get_bytes(mut self) -> Vec<u8> {
        let extra_bits = if self.bits_counter == 0 {
            0
        } else {
            8 - self.bits_counter as u8
        };

        if self.bits_counter != 0 {
            let mut byte = 0;
            for i in 0..8 {
                byte |= self.bits[i] << i;
            }
            self.bytes.push(byte);
        }

        self.bytes.push(extra_bits);
        self.bytes
    }
}

#[test]
fn test_get_bytes() {
    let mut comp = Compressor::new();

    comp.feed_bits(vec![0, 0, 1]);

    let bytes = comp.get_bytes();
    assert_eq!(bytes, vec![4, 5]);
}

#[test]
fn test_get_bytes_aligned() {
    let mut comp = Compressor::new();

    // 18 64 128 = 82 128 = 210
    // 2 4 32 128 = 166
    comp.feed_bits(vec![0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1]);

    let bytes = comp.get_bytes();
    assert_eq!(bytes, vec![210, 166, 0]);
}
