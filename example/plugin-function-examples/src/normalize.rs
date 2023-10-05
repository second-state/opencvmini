use std::fs;

use opencvmini::{imdecode, imencode, imwrite, normalize, DataType, NormTypes};

fn main() {
    let image = fs::read("./asset/colour-wheel.jpg").expect("failed to open image");
    let bytes_len = image.len();
    let img = imdecode(&image);

    let img = normalize(
        img,
        0,
        150,
        NormTypes::NORM_MINMAX,
        DataType::InputMat,
        None,
    );

    let mut out_buf: Vec<u8> = Vec::new();

    // The normalize function in some cases can increase the number of bytes written out
    // This is potentially due to the how a jpg is represented as cosines
    // So to make sure we dont cut our image off, we need to allocate a larger buffer.
    out_buf.resize(bytes_len * 2, 0);

    imencode(".jpg", img, &out_buf);

    if let Err(e) = fs::write("output_rust.jpg", out_buf) {
        eprintln!("Error writing image out {}", e);
    } else {
        println!("Written image out")
        // This could lead to writing more bytes out than needed
    };

    // Alternatively we can use imwrite to write directly, the correct number of bytes to disk
    imwrite("output_imwrite.jpg", img);
}
