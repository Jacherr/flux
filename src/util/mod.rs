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

pub fn get_buf_index_in_buf(buf: &[u8], search: &[u8]) -> isize {
    let mut index = 0;
    let mut result: isize = 0;
    while index < (buf.len() - search.len()) {
        let next_bytes = &buf[index..(index + search.len())];
        if next_bytes == search {
            result = index as isize;
            break;
        } else if index == buf.len() - search.len() - 1 {
            result = -1;
            break;
        };
        index += 1;
    }
    result
}

#[inline(always)]
pub fn collapse(target_limit: usize, current_value: usize) -> usize {
    current_value % target_limit
}

#[inline(always)]
pub fn collapse_neg(target_limit: isize, current_value: isize) -> usize {
    if current_value < 0 {
        (current_value % target_limit + target_limit) as usize
    } else {
        (current_value % target_limit) as usize
    }
}
