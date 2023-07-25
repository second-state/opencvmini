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
