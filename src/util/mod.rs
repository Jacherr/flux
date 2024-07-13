use std::hash::{DefaultHasher, Hash, Hasher};

use rand::Rng;

use crate::processing::dynamic_image_wrapper::DynamicImageWrapper;

pub mod owned_child;
pub mod tmpfile;

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

pub fn windows2<F>(buf: &mut [DynamicImageWrapper], mut fun: F)
where
    F: FnMut(&mut DynamicImageWrapper, &mut DynamicImageWrapper),
{
    let mut start = 0;
    let mut end = 2;

    while end <= buf.len() {
        let slice = &mut buf[start..end];
        let (a, b) = slice.split_at_mut(1);
        fun(&mut a[0], &mut b[0]);
        start += 1;
        end += 1;
    }
}

pub fn remove_every_nth_from_vec<T>(vec: &mut Vec<T>, n: usize) -> &mut Vec<T> {
    let mut i = 1;

    while (i * n) < vec.len() {
        vec.remove(i * n);
        i += 1;
    }

    vec
}

pub fn keep_every_nth_in_vec<T: Clone>(vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut i = 1;

    let mut new = vec![];

    while (i * n) < vec.len() {
        new.push(vec[i * n].clone());
        i += 1;
    }

    new
}
