use opencvmini::*;
use std::fs;
use wasmedge_tensorflow_interface::*;

fn main() {
    let image = fs::read("asset/image_1_thumb.jpg").expect("failed to open image");

    let mod_buf =
        fs::read("model/ssd_mobilenet_v1_1_metadata_1.tflite").expect("failed to open model");
    // The mod_buf is a vec<u8> which contains the model data.
    let mut session = TFLiteSession::new(&mod_buf);

    println!("start");

    let mut buf: Vec<u8> = vec![];
    buf.resize(270000, 0);

    // 270000 is from: 300 height * 300 width
    // and * 3 rgb?

    let img = imdecode(&image);
    let img = bilinear_sampling(img, 300, 300);

    // encode back to instance's buffer
    imencode(".jpg", img, &buf);

    println!("going to add input");
    session
        .add_input("normalized_input_image_tensor", &buf)
        .run();
    println!("input added");

    // Described in https://www.tensorflow.org/lite/examples/object_detection/overview#output_signature
    // 0 Locations: Multidimensional array of [N][4] floating point values between 0 and 1, the inner arrays representing bounding boxes in the form [top, left, bottom, right]
    let locations: Vec<f32> = session.get_output("TFLite_Detection_PostProcess");
    for i in 0..locations.len() / 4 {
        let h = i * 4;
        let left = (300 as f32 * locations[h]) as u32;
        let top = (300 as f32 * locations[h + 1]) as u32;
        let right = (300 as f32 * locations[h + 2]) as u32;
        let bottom = (300 as f32 * locations[h + 3]) as u32;
        println!("{}: ({}, {}, {}, {})", i, left, top, right, bottom);
        let img = rectangle(img, top, left, bottom, right);
    }
    // 1 Classes: Array of N integers (output as floating point values) each indicating the index of a class label from the labels file
    let class_names: Vec<String> = fs::read_to_string("asset/labelmap.txt")
        .expect("failed to open labelmap")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let classes: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:1");
    for f in classes {
        println!("{}: class {}", f, class_names[f as usize]);
    }
    // 2 Scores: Array of N floating point values between 0 and 1 representing probability that a class was detected
    let scores: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:2");
    println!("Scores: {:?}", scores);
    // 3 Number of detections: Integer value of N
    let number_of_detections: Vec<u8> = session.get_output("TFLite_Detection_PostProcess:3");
    let num = u32::from_ne_bytes(number_of_detections[0..4].try_into().unwrap());
    println!("Number of detections: {}", num);

    let img = imwrite("output.jpg", img);

    println!("done");
}
