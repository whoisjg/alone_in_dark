use std::ffi::CString;

use crate::app::AppContext;

pub(crate) fn init_window(screen_width: i32, screen_height: i32, name: &str) {
    unsafe {
        rl::InitWindow(
            screen_width,
            screen_height,
            CString::new(name).unwrap().as_ptr(),
        );
    }
}

pub(crate) fn close_window() {
    unsafe {
        rl::CloseWindow();
    }
}

#[cfg(not(target_arch = "wasm32"))]

pub(crate) fn window_should_close() -> bool {
    unsafe {
        return rl::WindowShouldClose();
    }
}

#[cfg(target_arch = "wasm32")]

pub(crate) fn window_should_close() -> bool {
    return false;
}

impl AppContext {
    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe {
            rl::SetTargetFPS(fps);
        }
    }
}
