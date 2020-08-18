use image::{Rgb, RgbImage};
use ndarray::Array2;
use crate::random;
use crate::rng::sample;

fn merge_color(old: u8, new: u8, alpha: u8) -> u8 {
    let a2 = (!alpha) as u16;
    let tot = (new as u16) * (alpha as u16) + (old as u16) * a2 + 127;
    (tot / 255) as u8
}

pub fn merge_one(img: &mut RgbImage, layer: &Array2<u8>, color: Rgb<u8>) {
    for (x, y, pix) in img.enumerate_pixels_mut() {
        let alpha = layer[(x as usize, y as usize)];
        for i in 0..3 {
            pix[i] = merge_color(pix[i], color[i], alpha);
        }
    }
}

pub fn merge_random_color<'a>(img: &'a mut RgbImage) -> impl FnMut(&'a Array2<u8>)
{
    move | layer | {
        merge_one(img, layer, Rgb(sample(random::Color)));
    }
}
