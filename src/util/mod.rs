use std::hash::{DefaultHasher, Hash, Hasher};

use rand::Rng;

pub mod owned_child;

pub fn pad_left(s: String, m: usize, c: char) -> String {
    if let Some(columns_short) = m.checked_sub(s.len()) {
        let padding_width = 1;
        // Saturate the columns_short
        let padding_needed = columns_short + padding_width - 1 / padding_width;
        let mut t = String::with_capacity(s.len() + padding_needed);
        t.extend((0..padding_needed).map(|_| c));
        t.push_str(&s);
        t
    } else {
        s
    }
}

pub fn hash_buffer(buf: &[u8]) -> String {
    let mut body_hasher = DefaultHasher::new();
    buf.hash(&mut body_hasher);
    let rand = rand::thread_rng().gen::<usize>();
    format!("{:x}{:x}", body_hasher.finish(), rand)
}

pub fn convert_ratio_to_integer(numer: u32, denom: u32) -> u32 {
    let ratio = num_rational::Ratio::from_integer(numer / denom);
    let out = ratio.numer();

    *out
}
