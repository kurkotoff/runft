extern crate image;
extern crate rand;

use image::{RgbImage, Rgb, imageops::blur};
use rand::random;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn get_rand_img() -> RgbImage {
    let mut res = RgbImage::new(WIDTH, HEIGHT);
    let r0 = random::<i16>() % 256;
    let g0 = random::<i16>() % 256;
    let b0 = random::<i16>() % 256;

    for y in 0u32..255 {
        for x in 0u32..255 {
            res.put_pixel(y, x, Rgb([(r0 - y as i16).abs() as u8, (g0 - x as i16).abs() as u8, (b0 - x as i16 - y as i16).abs() as u8]));
        }
    }

    res
}

fn main() {
    let image = get_rand_img();
    let image = blur(&image, 10f32);
    image.save("test.png").unwrap();
}
