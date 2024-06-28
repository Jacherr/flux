use std::ffi::{c_char, c_int};

extern "C" {
    pub fn v_vips_init() -> c_int;
    pub fn v_transcode_to(
        input: *const u8,
        len: usize,
        output: *mut *mut u8,
        size: *mut usize,
        format: *const c_char,
    ) -> c_int;
    pub fn v_gravity(
        input: *const u8,
        len: usize,
        output: *mut *mut u8,
        size: *mut usize,
        width: usize,
        height: usize,
    ) -> c_int;
    pub fn v_generate_caption_header(
        buf: *mut *mut u8,
        size: *mut usize,
        height: *mut usize,
        width: usize,
        text: *const c_char,
    ) -> c_int;
    pub fn v_generate_meme_text(
        buf: *mut *mut u8,
        size: *mut usize,
        height: *mut usize,
        width: usize,
        text: *const c_char,
    ) -> c_int;
    pub fn v_generate_motivate_text(
        buf: *mut *mut u8,
        size: *mut usize,
        height: *mut usize,
        width: usize,
        text: *const c_char,
        text_size: usize,
        pad_height: c_int,
    ) -> c_int;
    pub fn v_g_free(ptr: *const ());
    pub fn v_get_error() -> *const c_char;
}
