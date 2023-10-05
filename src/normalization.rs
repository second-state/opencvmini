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

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum DataType {
    // Output type the same as the input type
    InputMat = -1,
    // 8 bit unsigned integer
    CV_8U = 0,
    // 8 bit signed integer
    CV_8S = 1,
    // 16 bit usigned integer
    CV_16U = 2,
    // 16 bit usigned integer
    CV_16S = 3,
    // 32 bit signed integer
    CV_32S = 4,
    // 32 bit float
    CV_32F = 5,
    // 64 bit float
    CV_64F = 6,
    // 64 bit float
    CV_USRTYPE1 = 7,
}
