mod generated {
    #![allow(non_camel_case_types)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

include!("color_conversion.rs");
include!("interpolation.rs");
include!("normalization.rs");

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

pub fn bilinear_sampling(m: MatKey, w: u32, h: u32) -> MatKey {
    unsafe { generated::bilinear_sampling(m, w, h) }
}

pub fn cvt_color(m: MatKey, code: ColorConversionCodes, dest_channel_n: i32) -> MatKey {
    unsafe { generated::cvt_color(m, code as i32, dest_channel_n) }
}

pub fn resize(m: MatKey, w: u32, h: u32, interpolation_flag: InterpolationFlags) -> MatKey {
    unsafe { generated::resize(m, w, h, interpolation_flag as u32) }
}

pub fn normalize(
    m: MatKey,
    alpha: u32,
    beta: u32,
    normalization: NormTypes,
    dtype: DataType,
    mask_m_opt: Option<MatKey>,
) -> MatKey {
    let mask_m = match mask_m_opt {
        Some(mask) => mask,
        None => imdecode(&[]),
    };

    unsafe { generated::normalize(m, alpha, beta, normalization as u32, dtype as u32, mask_m) }
}

pub enum Color {
    RGB(f64, f64, f64),
}

pub struct RectangleOption {
    thickness: i32,
    line_type: i32,
    shift: i32,
}
impl Default for RectangleOption {
    fn default() -> Self {
        Self {
            thickness: 1,
            line_type: 8,
            shift: 0,
        }
    }
}
impl RectangleOption {
    pub fn thickness(self, thickness: i32) -> Self {
        RectangleOption {
            thickness: thickness,
            ..self
        }
    }
    pub fn line_type(self, line_type: i32) -> Self {
        RectangleOption {
            line_type: line_type,
            ..self
        }
    }
    pub fn shift(self, shift: i32) -> Self {
        RectangleOption {
            shift: shift,
            ..self
        }
    }
}

pub fn rectangle(
    m: MatKey,
    top: u32,
    left: u32,
    bottom: u32,
    right: u32,
    color: Color,
    opts: RectangleOption,
) {
    let Color::RGB(r, g, b) = color;
    unsafe {
        generated::rectangle(
            m,
            top,
            left,
            bottom,
            right,
            r,
            g,
            b,
            opts.thickness,
            opts.line_type,
            opts.shift,
        )
    }
}
