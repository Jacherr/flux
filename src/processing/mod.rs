use image::imageops::overlay;
use image::DynamicImage;

pub mod css_framebuffer;
pub mod decode;
pub mod dynamic_image_wrapper;
pub mod encode;
pub mod ffmpeg;
pub mod filetype;
pub mod framebuffer;
pub mod gegl;
pub mod gif;
pub mod makesweet;
pub mod media_object;
pub mod type_conversion;

pub fn roll_image(image: &DynamicImage, x_scroll: u32, y_scroll: u32) -> DynamicImage {
    let mut canvas = DynamicImage::new_rgba8(image.width(), image.height());
    let x_scroll = x_scroll % image.width();
    let y_scroll = y_scroll % image.height();
    let x_scroll_move_right = image.crop_imm(0, 0, x_scroll, image.height());
    let x_scroll_move_left = image.crop_imm(x_scroll, 0, image.width() - x_scroll, image.height());
    overlay(&mut canvas, &x_scroll_move_left, 0, 0);
    overlay(&mut canvas, &x_scroll_move_right, (image.width() - x_scroll) as i64, 0);
    let y_scroll_move_down = canvas.crop_imm(0, 0, image.width(), y_scroll);
    let y_scroll_move_up = canvas.crop_imm(0, y_scroll, image.width(), image.height() - y_scroll);
    overlay(&mut canvas, &y_scroll_move_up, 0, 0);
    overlay(&mut canvas, &y_scroll_move_down, 0, (image.height() - y_scroll) as i64);
    canvas
}
