use std::fmt::{Debug, Display};

/// Maps Rust types to LERC C API type codes.
pub trait LercDataType: Default + Clone + Copy + PartialEq + Debug + Display {
    const LERC_TYPE: u32;
}

impl LercDataType for i8 {
    const LERC_TYPE: u32 = 0;
}

impl LercDataType for u8 {
    const LERC_TYPE: u32 = 1;
}

impl LercDataType for i16 {
    const LERC_TYPE: u32 = 2;
}

impl LercDataType for u16 {
    const LERC_TYPE: u32 = 3;
}

impl LercDataType for i32 {
    const LERC_TYPE: u32 = 4;
}

impl LercDataType for u32 {
    const LERC_TYPE: u32 = 5;
}

impl LercDataType for f32 {
    const LERC_TYPE: u32 = 6;
}

impl LercDataType for f64 {
    const LERC_TYPE: u32 = 7;
}
