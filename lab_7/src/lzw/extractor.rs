use std::collections::VecDeque;

pub struct Extractor {
    bytes: VecDeque<u8>,
    byte_counter: usize,
    bits: Vec<u8>,
}

impl Extractor {
    pub fn new(bytes: Vec<u8>) -> Self {
        Extractor {
            bytes: bytes.into(),
            byte_counter: 0,
            bits: vec![],
        }
    }

    // counts how may bits exists in stream
    pub fn count_left(&self) -> usize {
        match self.bytes.back() {
            Some(last) => {
                self.bits.len() + 8 * (self.bytes.len() - self.byte_counter - 1) - *last as usize
            }
            None => {
                panic!("oops, empty bytes stream");
            }
        }
    }

    pub fn drain(&mut self, bits_count: usize) -> Vec<u8> {
        while self.bits.len() < bits_count {
            self.unpack_next_byte();
        }

        self.bits.drain(..bits_count).collect()
    }

    fn unpack_next_byte(&mut self) {
        if self.byte_counter + 1 == self.bytes.len() {
            panic!("invalid counter state");
        } else {
            let mut byte = self.bytes[self.byte_counter];
            self.byte_counter += 1;
            for _ in 0..8 {
                self.bits.push(byte & 1);
                byte >>= 1;
            }
        }
    }
}

#[test]
fn test_drain_aligned() {
    let mut ext = Extractor::new(vec![5, 4, 6, 0]);

    let left = ext.count_left();
    assert_eq!(left, 24);

    let bits = ext.drain(8);
    let left = ext.count_left();
    assert_eq!(left, 16);
    assert_eq!(bits, vec![1, 0, 1, 0, 0, 0, 0, 0]);

    let bits = ext.drain(8);
    let left = ext.count_left();
    assert_eq!(left, 8);
    assert_eq!(bits, vec![0, 0, 1, 0, 0, 0, 0, 0]);

    let bits = ext.drain(8);
    let left = ext.count_left();
    assert_eq!(left, 0);
    assert_eq!(bits, vec![0, 1, 1, 0, 0, 0, 0, 0]);
}

#[test]
fn test_drain() {
    // 10100000 00100000 01(000000) [01100000]
    // 101000000 010000001 - 9-bit chunks
    let mut ext = Extractor::new(vec![5, 4, 2, 6]);

    let left = ext.count_left();
    assert_eq!(left, 18);

    let bits = ext.drain(9);
    let left = ext.count_left();
    assert_eq!(left, 9);
    assert_eq!(bits, vec![1, 0, 1, 0, 0, 0, 0, 0, 0]);

    let bits = ext.drain(9);
    let left = ext.count_left();
    assert_eq!(left, 0);
    assert_eq!(bits, vec![0, 1, 0, 0, 0, 0, 0, 0, 1]);
}
