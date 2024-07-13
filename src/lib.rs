use std::io::{Cursor};
use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat, Rgba};
use base64::{engine::general_purpose, Engine as _};
use imageproc::drawing::{draw_filled_circle_mut};
use rand::Rng;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn trick_png() -> String{
    let mut img = DynamicImage::new_rgba8(320,320);
    // 円の中心座標
    let center_x = img.width() / 2;
    let center_y = img.height() / 2;
    let mut rng = rand::thread_rng();
    for _i in 0..=4 {
        draw_filled_circle_mut(&mut img, (center_x as i32, center_y as i32), rng.gen_range(0..=155), Rgba([rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255), 255]));
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}
