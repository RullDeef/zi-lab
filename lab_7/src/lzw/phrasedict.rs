// vector of bytes
pub type Phrase = Vec<u8>;
// vector of bits
pub type Code = Vec<u8>;

#[derive(Debug)]
pub struct PhrasesDict {
    sym_table: Vec<Phrase>,
    sym_len: usize,
}

impl PhrasesDict {
    pub fn new(sym_len: usize) -> Self {
        let mut dict = PhrasesDict {
            sym_len,
            sym_table: vec![],
        };

        // initialize sym_table
        for index in 0..=255 {
            dict.add(vec![index]);
        }

        dict
    }

    pub fn add(&mut self, phrase: Phrase) {
        if self.sym_table.len() < 1 << self.sym_len {
            self.sym_table.push(phrase);
        } else {
            //println!("table is full! cannot add more phrases");
        }
    }

    pub fn lookup(&self, lookup_phrase: &[u8]) -> Option<Code> {
        for (index, phrase) in self.sym_table.iter().enumerate() {
            if *phrase == *lookup_phrase {
                return Some(self.index_to_code(index));
            }
        }
        None
    }

    pub fn get_phrase(&self, mut code: Vec<u8>) -> Option<&Vec<u8>> {
        let mut index = 0usize;
        while let Some(bit) = code.pop() {
            index <<= 1;
            index |= bit as usize;
        }
        
        self.sym_table.get(index)
    }

    fn index_to_code(&self, mut index: usize) -> Code {
        let mut code = Code::new();

        while index > 0 {
            code.push((index & 1) as u8);
            index >>= 1;
        }

        while code.len() < self.sym_len {
            code.push(0);
        }

        code
    }
}

#[test]
fn test_new() {
    for sl in [9, 10, 11, 12] {
        let dict = PhrasesDict::new(sl);

        assert_eq!(dict.sym_len, sl);
        assert_eq!(dict.sym_table.len(), 256);

        assert_eq!(
            *dict.get_phrase(vec![0, 1, 0, 1, 0, 1, 0, 1]).unwrap(),
            vec![0b10101010]
        );
    }
}

#[test]
fn test_index_to_code() {
    let dict = PhrasesDict::new(10);
    let index = 28;

    let code = dict.index_to_code(index);

    assert_eq!(code, vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0]);
}
