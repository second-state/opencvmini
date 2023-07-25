mod generated {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

/// # Example
///
/// ```rust
/// use image_encoder::imencode;
///
/// fn main() {
///   let buf = [0u8; 100];
///   imencode(".jpg", 0, &buf);
/// }
/// ```
pub fn imencode(ext: &str, m: u32, buf: &[u8]) {
    unsafe {
        generated::imencode(ext.as_ptr(), ext.len(), m, buf.as_ptr(), buf.len());
    }
}
