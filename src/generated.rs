pub type mat_key = u32;
#[link(wasm_import_module = "wasmedge_opencvmini")]
extern "C" {
    #[link_name = "wasmedge_opencvmini_bilinear_sampling"]
    pub fn bilinear_sampling(m: mat_key, w: u32, h: u32) -> mat_key;
    #[link_name = "wasmedge_opencvmini_blur"]
    pub fn blur(m: mat_key, kernel_width: u32, kernel_height: u32) -> mat_key;
    #[link_name = "wasmedge_opencvmini_cvt_color"]
    pub fn cvt_color(m: mat_key, code: i32, dest_channel: i32) -> mat_key;
    #[link_name = "wasmedge_opencvmini_imdecode"]
    pub fn imdecode(buf_ptr: *const u8, buf_len: usize) -> mat_key;
    #[link_name = "wasmedge_opencvmini_imencode"]
    pub fn imencode(
        ext_ptr: *const u8,
        ext_len: usize,
        m: mat_key,
        buf_ptr: *const u8,
        buf_len: usize,
    ) -> ();
    #[link_name = "wasmedge_opencvmini_imshow"]
    pub fn imshow(window_name_ptr: *const u8, window_name_len: usize, m: mat_key) -> ();
    #[link_name = "wasmedge_opencvmini_imwrite"]
    pub fn imwrite(file_name_ptr: *const u8, file_name_len: usize, m: mat_key) -> ();
    #[link_name = "wasmedge_opencvmini_normalize"]
    pub fn normalize(m: mat_key) -> mat_key;
    #[link_name = "wasmedge_opencvmini_rectangle"]
    pub fn rectangle(
        m: mat_key,
        top: u32,
        left: u32,
        bottom: u32,
        right: u32,
        R: f64,
        G: f64,
        B: f64,
        thickness: i32,
        lineType: i32,
        shift: i32,
    ) -> ();
    #[link_name = "wasmedge_opencvmini_waitkey"]
    pub fn waitkey(delay: u32) -> ();
}
