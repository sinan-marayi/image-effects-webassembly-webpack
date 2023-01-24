use base64::{Engine, engine::general_purpose};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use image::load_from_memory;
use image::ImageOutputFormat::Png;
// use Engine::decode;
#[wasm_bindgen]
pub fn gray_scale(base64_file: &str) -> String {
    log(&"grayscale called".into());
    // let base64_to_vector = Engine::decode(&base64_file).unwrap();
    let _bytes = general_purpose::STANDARD
    .decode(&base64_file).unwrap();
    log(&"Image decoded".into());

    let mut _img = load_from_memory(&_bytes).unwrap();
    log(&"Image loaded".into());

    _img = _img.grayscale();
    log(&"grayscale applied".into());

    let mut _buffer= vec![];

    _img.write_to(&mut _buffer, Png).unwrap();
    log(&"Image writed".into());

    let _encoded_img = general_purpose::STANDARD
    .encode(&_buffer);

    let data_url =format!("data:image/png;base64,{}",_encoded_img);

    data_url

}
