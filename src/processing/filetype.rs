use std::cmp::min;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Gif,
    Jpeg,
    Png,
    Webp,
    Mp4,
    Webm,
}
impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Type::Gif => "gif",
            Type::Jpeg => "jpeg",
            Type::Png => "png",
            Type::Webp => "webp",
            Type::Mp4 => "mp4",
            Type::Webm => "webm",
        }
    }
    pub fn as_mime(&self) -> &'static str {
        match self {
            Type::Gif => "image/gif",
            Type::Jpeg => "image/jpeg",
            Type::Png => "image/png",
            Type::Webp => "image/webp",
            Type::Mp4 => "video/mp4",
            Type::Webm => "video/webm",
        }
    }
    pub fn is_video(&self) -> bool {
        matches!(self, Type::Mp4 | Type::Webm)
    }
}

const GIF: [u8; 3] = [71, 73, 70];
const JPEG: [u8; 3] = [255, 216, 255];
const PNG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const WEBP: [u8; 4] = [87, 69, 66, 80];
const MP4: [u8; 4] = [0x66, 0x74, 0x79, 0x70];
const WEBM: [u8; 4] = [0x1A, 0x45, 0xDF, 0xA3];

fn bounded_range(start: usize, end: usize, len: usize) -> Range<usize> {
    min(len, start)..min(len, end)
}

fn sig(that: &[u8], eq: &[u8]) -> bool {
    that[0..std::cmp::min(eq.len(), that.len())].eq(eq)
}

fn check_webp(that: &[u8]) -> bool {
    let bytes_offset_removed = &that[bounded_range(8, 12, that.len())];
    sig(bytes_offset_removed, &WEBP)
}

fn check_mp4(that: &[u8]) -> bool {
    let bytes_offset_removed = &that[bounded_range(4, 8, that.len())];
    sig(bytes_offset_removed, &MP4)
}

pub fn get_sig(buf: &[u8]) -> Option<Type> {
    if buf.len() < 8 {
        return None;
    };
    if sig(buf, &GIF) {
        Some(Type::Gif)
    } else if sig(buf, &JPEG) {
        Some(Type::Jpeg)
    } else if sig(buf, &PNG) {
        Some(Type::Png)
    } else if check_webp(buf) {
        Some(Type::Webp)
    } else {
        None
    }
}

pub fn get_sig_incl_mp4(buf: &[u8]) -> Option<Type> {
    if buf.len() < 8 {
        return None;
    };
    if sig(buf, &GIF) {
        Some(Type::Gif)
    } else if sig(buf, &JPEG) {
        Some(Type::Jpeg)
    } else if sig(buf, &PNG) {
        Some(Type::Png)
    } else if check_webp(buf) {
        Some(Type::Webp)
    } else if check_mp4(buf) {
        Some(Type::Mp4)
    } else if sig(buf, &WEBM) {
        Some(Type::Webm)
    } else {
        None
    }
}
