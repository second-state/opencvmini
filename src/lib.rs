mod generated {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

type MatKey = generated::mat_key;

pub fn imdecode(buf: &[u8]) -> MatKey {
    unsafe { generated::imdecode(buf.as_ptr(), buf.len()) }
}
/// # Example
///
/// ```rust
/// fn main() {
///   let buf = [0u8; 100];
///   imencode(".jpg", 0, &buf);
/// }
/// ```
pub fn imencode(ext: &str, m: MatKey, buf: &[u8]) {
    unsafe {
        generated::imencode(ext.as_ptr(), ext.len(), m, buf.as_ptr(), buf.len());
    }
}
pub fn imwrite(file_name: &str, m: MatKey) {
    unsafe {
        generated::imwrite(file_name.as_ptr(), file_name.len(), m);
    }
}
pub fn imshow(window_name: &str, m: MatKey) {
    unsafe { generated::imshow(window_name.as_ptr(), window_name.len(), m) }
}
pub fn waitkey(delay: u32) {
    unsafe { generated::waitkey(delay) }
}

pub fn blur(m: MatKey, kernel_width: u32, kernel_height: u32) -> MatKey {
    unsafe { generated::blur(m, kernel_width, kernel_height) }
}
pub fn normalize(m: MatKey) -> MatKey {
    unsafe { generated::normalize(m) }
}
pub fn bilinear_sampling(m: MatKey, w: u32, h: u32) -> MatKey {
    unsafe { generated::bilinear_sampling(m, w, h) }
}
pub fn cvt_color(m: MatKey) -> MatKey {
    unsafe { generated::cvt_color(m) }
}

pub fn rectangle(m: MatKey, top: u32, left: u32, bottom: u32, right: u32) {
    unsafe { generated::rectangle(m, top, left, bottom, right) }
}
