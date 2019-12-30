extern crate image;

use image::{DynamicImage, FilterType};
use std::str::{from_utf8_unchecked};
use self::image::GenericImageView;

/*
❤️ stolen (with love) from https://github.com/edelsonc/asciify ❤️
*/
fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx];
        }
    }

    ascii_chars[(n_chars - 1) as usize]
}

// tried to return a Vec of strings, but my knowledge of Rust is terrible...
// just printing it should be fine!
pub fn print_image(im: DynamicImage, width: Option<u32>, height: Option<u32>) {
    // i fell in love with matches. don't judge me
    let (w, h) = match (width, height) {
        (None, None) => (im.width(), im.height()),
        (Some(w), None) => (w, im.height()),
        (None, Some(h)) => (im.width(), h),
        (Some(w), Some(h)) => (w, h),
    };

    let ascii = im
        .resize_exact(w, h, FilterType::Nearest)
        .to_luma()
        .pixels()
        .map(|p| intensity_to_ascii(&p[0]))
        .fold(String::new(), |s, p| s + p);

    let parts: Vec<&str> = ascii.as_bytes()
        .chunks(w as usize)
        .map(|c| unsafe { from_utf8_unchecked(c)}) // we can use unsafe, because the characters are surely ascii
        .collect();
    for s in parts {
        println!("{}", s);
    }
}
