use crate::bytes::ByteIndexable;

pub fn prettify<T: ByteIndexable>(val: T) -> String {
    let mut res = String::from("");

    res.push_str(format!("{0:02x}", val.get_byte(0)).as_str());
    for i in 1..(std::mem::size_of::<T>() as u8) {
        res.push_str(format!(" {0:02x}", val.get_byte(i)).as_str());
    }

    res
}
