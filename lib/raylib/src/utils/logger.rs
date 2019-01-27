extern crate log;

pub static LOGGER: TraceLogger = TraceLogger { empty: 0 };

use log::{Level, Metadata, Record};

use std::ffi::CString;

#[derive(Default)]
pub struct TraceLogger {
    empty: i32,
}

impl log::Log for TraceLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let args = CString::new(format!("{}", record.args())).unwrap();
        let level = match record.level() {
            Level::Error => rl::TraceLogType::LOG_ERROR,
            Level::Warn => rl::TraceLogType::LOG_WARNING,
            Level::Info => rl::TraceLogType::LOG_INFO,
            Level::Debug => rl::TraceLogType::LOG_DEBUG,
            Level::Trace => rl::TraceLogType::LOG_DEBUG,
        };
        if cfg!(target_arch = "wasm32") {
            unsafe {
                rl::TraceLog(2, args.as_ptr());
            }
            println!("{:?}", args);

        } else {
            println!("{:?}", args);
        }
    }

    fn flush(&self) {}
}
