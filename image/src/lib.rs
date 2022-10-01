mod spaces;

use std::io::{Cursor, Read};
use std::time::Instant;
use image::{ColorType, DynamicImage, EncodableLayout, ImageBuffer, Pixel, Rgb, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};


fn get_base_image() -> DynamicImage {
    let before_call = Instant::now();
    let mut img = image::open("image/assets/base-2.jpg").unwrap();
    println!("image:get_base_image elapsed: {}", before_call.elapsed().as_millis());

    return img;
}

fn get_font() -> Font<'static> {
    let before_call = Instant::now();
    let font_data = Vec::from(include_bytes!("../assets/Roboto-Regular.ttf") as &[u8]);
    let font: Font<'static> = Font::try_from_vec(font_data).expect("could not get font");
    println!("image::get_font elapsed: {}", before_call.elapsed().as_millis());

    return font;
}

pub async fn generate_image(text: &str) {
    let before_call = Instant::now();

    let mut img = get_base_image().clone();
    let font = get_font().clone();

    let height = 72.8;
    let scale = Scale {
        x: height * 2.0,
        y: height
    };

    let before_call_text = Instant::now();
    draw_text_mut(&mut img, Rgba([0u8, 0u8, 255u8, 0u8]), 0, 0, scale, &font, text);
    println!("image:generate_image:draw_text_mut elapsed :{}", before_call_text.elapsed().as_millis());

    let before_call_buffer = Instant::now();
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Jpeg(80)).unwrap();
    println!("image:generate_image:write_to elapsed: {}", before_call_buffer.elapsed().as_millis());

    println!("image:generate_image elapsed: {}", before_call.elapsed().as_millis());
    let op = spaces::upload_image(bytes);

    return op.await;
}