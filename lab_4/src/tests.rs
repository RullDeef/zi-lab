#[cfg(test)]
mod tests {
    use crate::bytes::{ByteIndexable, WordIndexable};

    #[test]
    fn get_byte() {
        let val = 0x123456u32;

        let byte_0 = val.get_byte(0);
        let byte_1 = val.get_byte(1);
        let byte_2 = val.get_byte(2);
        let byte_3 = val.get_byte(3);
        
        assert_eq!(byte_0, 0x56);
        assert_eq!(byte_1, 0x34);
        assert_eq!(byte_2, 0x12);
        assert_eq!(byte_3, 0x00);
    }

    #[test]
    fn set_byte() {
        let val = 0x123456u32;

        let byte_0 = val.set_byte(0, 0xAA);
        let byte_1 = val.set_byte(1, 0xBB);
        let byte_2 = val.set_byte(2, 0xCC);
        let byte_3 = val.set_byte(3, 0xDE);

        assert_eq!(byte_0, 0x001234AAu32);
        assert_eq!(byte_1, 0x0012BB56u32);
        assert_eq!(byte_2, 0x00CC3456u32);
        assert_eq!(byte_3, 0xDE123456u32);
    }

    #[test]
    fn get_word() {
        let val = 0x000102030405060708090A0B0C0D0E0Fu128;

        let word_0 = val.get_word(0);
        let word_1 = val.get_word(1);
        let word_2 = val.get_word(2);
        let word_3 = val.get_word(3);

        assert_eq!(word_0, 0x0C0D0E0Fu32);
        assert_eq!(word_1, 0x08090A0Bu32);
        assert_eq!(word_2, 0x04050607u32);
        assert_eq!(word_3, 0x00010203u32);
    }

    #[test]
    fn set_word() {
        let val = 0x000102030405060708090A0B0C0D0E0Fu128;

        let word_0 = val.set_word(0, 0x12345678u32);
        let word_1 = val.set_word(1, 0x12345678u32);
        let word_2 = val.set_word(2, 0x12345678u32);
        let word_3 = val.set_word(3, 0x12345678u32);

        assert_eq!(word_0, 0x000102030405060708090A0B12345678u128);
        assert_eq!(word_1, 0x0001020304050607123456780C0D0E0Fu128);
        assert_eq!(word_2, 0x000102031234567808090A0B0C0D0E0Fu128);
        assert_eq!(word_3, 0x123456780405060708090A0B0C0D0E0Fu128);
    }
}
