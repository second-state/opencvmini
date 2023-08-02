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

    let img = imdecode(&image);
    // let img = normalize(img);
    let img = bilinear_sampling(img, 300, 300);

    // encode back to instance's buffer
    let mut buf: Vec<u8> = vec![];
    buf.resize(270000, 0);
    imencode(".jpg", img, &buf);

    println!("going to add input");
    session
        .add_input("normalized_input_image_tensor", &buf)
        .run();
    println!("input added");

    // Described in https://www.tensorflow.org/lite/examples/object_detection/overview#output_signature
    // 0 Locations: Multidimensional array of [N][4] floating point values between 0 and 1, the inner arrays representing bounding boxes in the form [top, left, bottom, right]
    let locations: Vec<f32> = session.get_output("TFLite_Detection_PostProcess");
    // 1 Classes: Array of N integers (output as floating point values) each indicating the index of a class label from the labels file
    let class_names: Vec<String> = fs::read_to_string("asset/labelmap.txt")
        .expect("failed to open labelmap")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let classes: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:1");
    // 2 Scores: Array of N floating point values between 0 and 1 representing probability that a class was detected
    let scores: Vec<f32> = session.get_output("TFLite_Detection_PostProcess:2");
    // 3 Number of detections: Integer value of N
    let _number_of_detections: Vec<u8> = session.get_output("TFLite_Detection_PostProcess:3");
    for i in 0..locations.len() / 4 {
        let score = scores[i];
        if score < 0.5 {
            continue;
        }
        let class = classes[i];
        let class_name = &class_names[class as usize];
        println!("detect a {}", class_name);
        let image_size = 300;
        let left = (image_size as f32 * locations[i * 4]) as u32;
        let top = (image_size as f32 * locations[i * 4 + 1]) as u32;
        let right = (image_size as f32 * locations[i * 4 + 2]) as u32;
        let bottom = (image_size as f32 * locations[i * 4 + 3]) as u32;
        rectangle(
            img,
            top,
            left,
            bottom,
            right,
            Color::RGB(0., 255., 0.),
            RectangleOption::default().thickness(2),
        );
    }

    imwrite("output.jpg", img);

    println!("done");
}
