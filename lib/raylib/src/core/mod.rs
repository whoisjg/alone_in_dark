use std::ffi::CString;

pub mod input;

pub use self::input::*;

// int MeasureText(const char *text, int fontSize);
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    unsafe { rl::MeasureText(CString::new(text).unwrap().as_ptr(), font_size) }
}
