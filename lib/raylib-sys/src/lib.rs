#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn it_works() {
        unsafe {
            InitWindow(
                800,
                450,
                CString::new("raylib [core] example - basic window")
                    .unwrap()
                    .as_ptr(),
            );
        }
    }
}
