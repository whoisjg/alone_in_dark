use crate::app::{close_window, init_window, window_should_close, Game};
use crate::utils::LOGGER;

#[cfg(target_arch = "wasm32")]
use crate::wasm::*;

use log::LevelFilter;

use std::cell::RefCell;

thread_local!(static APP: RefCell<AppState> = RefCell::new(AppState::Empty));

pub enum AppState {
    Empty,
    Running(AppContext, Box<Game>),
}

pub fn run(screen_width: i32, screen_height: i32, name: &str, mut game: Box<Game>) {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    APP.with(|app| {
        let next = match *app.borrow() {
            AppState::Empty => {
                init_window(screen_width, screen_height, name);
                game.start(Default::default());
                AppState::Running(Default::default(), game)
            }
            AppState::Running(_, _) => {
                close_window();
                AppState::Empty
            }
        };

        *app.borrow_mut() = next;
    });

    _run();
}

unsafe extern "C" fn _update() {
    APP.with(|app| match *app.borrow_mut() {
        AppState::Running(ref mut ctx, ref mut g) => {
            if !window_should_close() {
                ctx.frame += 1;
                if g.update(ctx.clone()) {
                    close_window();
                }
            }
        }
        _ => (),
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn _run() {
    while !window_should_close() {
        unsafe {
            _update();
        }
    }
    APP.with(|app| {
        // Should result in drop
        // info!("closing game");
        *app.borrow_mut() = AppState::Empty;
    });
    close_window()
}

#[cfg(target_arch = "wasm32")]
fn _run() {
    unsafe {
        // TODO submit bugfix to raylib
        emscripten_sample_gamepad_data();
        emscripten_set_main_loop(_update, 0, 1);
    }
}

#[derive(Clone, Default)]
pub struct AppContext {
    pub frame: u32,
    pub should_quit: bool,
}

impl AppContext {
    pub fn get_frame_time(&self) -> f32 {
        unsafe { rl::GetFrameTime() }
    }
}
