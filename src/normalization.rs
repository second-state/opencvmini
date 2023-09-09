// https://docs.opencv.org/4.8.0/d2/de8/group__core__array.html#gad12cefbcb5291cf958a85b4b67b6149f
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum NormTypes {
    NORM_INF = 1,

    NORM_L1 = 2,

    NORM_L2 = 4,

    NORM_L2SQR = 5,

    NORM_HAMMING = 6,

    NORM_HAMMING2 = 7,

    // In the opencv repo NORM_TYPE_MASK and NORM_HAMMING2 have the same value,
    // I am unsure if this is intended, but for now rust does not support enums having the same value,
    // For now: If you need to use NORM_TYPE_MASK use NORM_HAMMING2
    // NORM_TYPE_MASK = 7,
    NORM_RELATIVE = 8,

    NORM_MINMAX = 32,
}
