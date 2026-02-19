pub mod moka;
pub mod request;

pub mod set {
    use std::collections::HashSet;

    pub fn to_bytes(set: HashSet<i16>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for b in set.into_iter() {
            bytes[(b / 8) as usize] |= 1 << b % 8;
        }
        bytes
    }
}

pub mod str {
    use std::collections::HashSet;

    pub fn to_set(str: String) -> HashSet<i16> {
        let mut set = HashSet::new();
        let bytes = str.as_bytes();
        for b in 0..bytes.len() {
            if bytes[b / 8] >> (b % 8) & 1 == 1 {
                set.insert(b as i16);
            }
        }
        set
    }
}
