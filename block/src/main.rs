// 2024 Paweł Rybak

use bmp::Image;
use cbc::cbc;
use ecb::ecb;
use rand::Rng;

mod cbc;
mod ecb;

const BLOCK_SIZE: usize = 16;

fn pad(data: &mut Vec<u8>) {
    let pad_len = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    data.extend(vec![pad_len as u8; pad_len]);
}

fn bmp_to_bytes(img: &Image) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((img.get_width() * img.get_height() * 3) as usize);
    for (x, y) in img.coordinates() {
        let pixel = img.get_pixel(x, y);
        bytes.push(pixel.r);
        bytes.push(pixel.g);
        bytes.push(pixel.b);
    }
    bytes
}

fn bytes_to_bmp(img: &mut Image, data: &[u8]) {
    let mut i = 0;
    for (x, y) in img.coordinates() {
        let mut pixel = img.get_pixel(x, y);
        pixel.r = data[i];
        pixel.g = data[i + 1];
        pixel.b = data[i + 2];
        img.set_pixel(x, y, pixel);
        i += 3;
    }
}

fn main() {
    let image = bmp::open("plain.bmp").unwrap();
    let pixel_data = bmp_to_bytes(&image);

    let key = b"abcdefghijklmnop";
    let mut rng = rand::thread_rng();
    let iv: [u8; BLOCK_SIZE] = rng.gen();

    let ecb_data = ecb(&pixel_data, key);
    let cbc_data = cbc(&pixel_data, key, &iv);

    let mut ecb_img = image.clone();
    let mut cbc_img = image.clone();

    bytes_to_bmp(&mut ecb_img, &ecb_data);
    bytes_to_bmp(&mut cbc_img, &cbc_data);

    ecb_img.save("ecb_crypto.bmp").unwrap();
    cbc_img.save("cbc_crypto.bmp").unwrap();
}
