mod generated {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

pub fn imencode(m: u32, buf: &[u8]) {
    unsafe {
        generated::imencode(m, buf.as_ptr(), buf.len());
    }
}
