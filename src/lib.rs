pub mod generated;

pub fn imencode(buf: &[u8], m: u32) {
    unsafe {
        generated::imencode(buf.as_ptr(), buf.len(), m);
    }
}
