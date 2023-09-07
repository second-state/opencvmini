use std::fs;

use opencvmini::{cvt_color, imdecode, imencode, ColorConversionCodes};

fn main() {
    let image = fs::read("./asset/colour-wheel.jpg").expect("failed to open image");
    // let image = fs::read("./asset/color-wheel-additive.jpg").expect("failed to open image");
    let bytes_len = image.len();

    let img = imdecode(&image);
    let img = cvt_color(img, ColorConversionCodes::COLOR_RGB2GRAY, 412);

    let mut out_buf: Vec<u8> = Vec::with_capacity(bytes_len);

    out_buf.resize(bytes_len, 0);
    imencode(".jpg", img, &out_buf);

    if let Err(e) = fs::write("output.jpg", out_buf) {
        eprintln!("Error writing image out {}", e);
    } else {
        println!("Written image out")
    };
}
