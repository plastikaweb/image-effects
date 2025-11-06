use std::vec;

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::prelude::*;
use image::load_from_memory;
use std::io::Cursor;
use image::ImageFormat::Png;

#[wasm_bindgen]

pub fn grayscale(file: &str) -> String {
    log(&"grayscale() called:".into());

    let base64_to_vector = BASE64_STANDARD.decode(file)
    .unwrap();

    log(&"Image decoded from base64".into());

    let mut image = load_from_memory(&base64_to_vector)
    .unwrap();

    image = image.grayscale();

    log(&"Image converted to grayscale".into());

    let mut buffer = vec![];
    image.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();

    log(&"New image encoded to base64".into());

    let encoded_image = BASE64_STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_image);

    log(&"Data URL created".into());

    return data_url;
}

