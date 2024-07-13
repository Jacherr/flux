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

pub fn gif_get_comments(buf: &[u8]) -> Vec<String> {
    let comment_block_match = &[0x00u8, 0x21u8, 0xFEu8];
    let mut next_match_index = get_buf_index_in_buf(buf, comment_block_match);
    let mut results: Vec<String> = vec![];
    let mut used: Vec<isize> = vec![];

    while next_match_index != -1 {
        let mut index = next_match_index as usize + comment_block_match.len();

        let mut comment_raw: Vec<u8> = vec![];

        while buf[index] != 0 {
            let sub_block_len = buf[index];
            let data_start = index + 1;
            let sub_block = &buf
                .get(data_start..(sub_block_len as usize + data_start))
                .unwrap_or(&[]);

            // fallback
            if sub_block.is_empty() {
                return vec![];
            }

            comment_raw.extend(sub_block.iter());

            index += sub_block_len as usize + 1;
        }

        // this is just a fallback in case the extractor somehow finds something that isnt really
        // a valid comment (assyst probably wont handle unicode well)
        let stringified = String::from_utf8_lossy(&comment_raw).to_string();
        if stringified.is_ascii() {
            results.push(stringified);
        }

        used.push(next_match_index);

        next_match_index = get_buf_index_in_buf(&buf[(next_match_index + 1) as usize..buf.len()], comment_block_match);

        if used.contains(&next_match_index) {
            break;
        };
    }

    results
}
