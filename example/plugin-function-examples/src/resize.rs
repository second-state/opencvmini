use std::fs;

use opencvmini::{imdecode, imencode, resize, InterpolationFlags};

fn main() {
    let image = fs::read("./asset/colour-wheel.jpg").expect("failed to open image");
    let bytes_len = image.len();

    let img = imdecode(&image);

    let img = resize(img, 20, 20, InterpolationFlags::INTER_NEAREST);

    let mut out_buf: Vec<u8> = Vec::with_capacity(bytes_len);

    out_buf.resize(bytes_len, 0);
    imencode(".jpg", img, &out_buf);

    if let Err(e) = fs::write("output.jpg", out_buf) {
        eprintln!("Error writing image out {}", e);
    } else {
        println!("Written image out")
    };
}
