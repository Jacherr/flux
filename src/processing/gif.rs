use bytes::Buf;
use image::codecs::gif::Repeat;

use crate::util::get_buf_index_in_buf;

pub fn gif_get_repeat_count(buf: &[u8]) -> Repeat {
    const MATCH: &[u8; 16] = b"\x21\xFF\x0BNETSCAPE2.0\x03\x01";
    let index_of_match: isize = get_buf_index_in_buf(buf, MATCH);

    // no repeat data means that the gif doesnt repeat,
    // instead of repeating infinitely, apparently...
    if index_of_match == -1 {
        return Repeat::Finite(0);
    };

    let next_bytes_index_start = index_of_match as usize + MATCH.len();
    let mut repeats_bytes = &buf[next_bytes_index_start..(next_bytes_index_start + 2)];
    let repeats = repeats_bytes.get_u16_le();

    if repeats == 0 {
        Repeat::Infinite
    } else {
        Repeat::Finite(repeats)
    }
}
