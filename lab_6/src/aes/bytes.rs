use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

pub trait ByteIndexable: Copy {
    fn get_byte(self, index: u8) -> u8;
    fn set_byte(self, index: u8, value: u8) -> Self;
    fn reverse_bytes(self) -> Self;
}

pub trait WordIndexable: Copy {
    fn get_word(self, index: u8) -> u32;
    fn set_word(self, index: u8, value: u32) -> Self;
}

impl<T> ByteIndexable for T
where
    T: Not<Output = T>
        + Shl<T, Output = T>
        + Shr<T, Output = T>
        + BitAnd<T, Output = T>
        + From<u8>
        + TryInto<u8>
        + BitOr<T, Output = T>
        + std::marker::Copy,
    <T as TryInto<u8>>::Error: std::fmt::Debug,
{
    fn get_byte<'a>(self, index: u8) -> u8 {
        (self.shr(T::from(index * 8)) & T::from(0xFFu8))
            .try_into()
            .unwrap()
    }

    fn set_byte(self, index: u8, value: u8) -> T {
        (self & !(T::from(0xFF) << T::from(index * 8))) | (T::from(value).shl(T::from(index * 8)))
    }

    fn reverse_bytes(self) -> T {
        let mut res = self;
        let len = std::mem::size_of::<T>() as u8;
        for i in 0..len {
            res = res.set_byte(i, self.get_byte(len - i - 1));
        }
        res
    }
}

impl<T> WordIndexable for T
where
    T: std::marker::Copy
        + Not<Output = T>
        + Shr<T, Output = T>
        + Shl<T, Output = T>
        + From<u32>
        + BitAnd<T, Output = T>
        + BitOr<T, Output = T>
        + TryInto<u32>,
    <T as TryInto<u32>>::Error: std::fmt::Debug,
{
    fn get_word(self, index: u8) -> u32 {
        ((self.shr(T::from(index as u32 * 32))) & T::from(0xFFFFFFFF))
            .try_into()
            .unwrap()
    }

    fn set_word(self, index: u8, value: u32) -> Self {
        (self & !(T::from(0xFFFFFFFF) << T::from((index as u32) * 32)))
            | (T::from(value) << T::from((index as u32) * 32))
    }
}

pub fn pack_bytes(bytes: &[u8; 16]) -> u128 {
    let mut res = 0;
    for i in 0..16 {
        res = res.set_byte(i, bytes[i as usize]);
    }
    res
}
