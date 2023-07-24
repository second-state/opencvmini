pub mod generated;

pub fn imencode(m: u32, buf: &[u8]) {
    unsafe {
        generated::imencode(m, buf.as_ptr(), buf.len());
    }
}
