use log::{ Level, LevelFilter, Log, Metadata, Record };
use std::ffi::CString;

// -----------------------[ FFI ]-----------------------
extern {
    fn info(ptr: *const u8);    // console.log
    fn error(ptr: *const u8);   // console.error
    fn warn(ptr: *const u8);    // console.warning
}

/// Attaches all JS <-> WASM logging callbacks.
#[no_mangle]
pub extern "C" fn hook_log() {
    log::set_logger(&WasmLogger).unwrap();
    log::set_max_level(LevelFilter::Debug);
}
// -----------------------------------------------------

/// A JS <-> WASM logger.
pub struct WasmLogger;

impl Log for WasmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() < log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let func = match record.level() {
            Level::Error => error,
            Level::Warn => warn,
            Level::Info => info,
            Level::Debug => info,
            Level::Trace => info,
        };
        if let Ok(str) = CString::new(format!("{}", record.args())) {
            unsafe {
                // SAFETY:
                // Lifetime of the borrow is as long as the JavaScript
                // function, so `str` can safely be dropped after.
                func(str.as_ptr() as _);
            }
        }
    }

    fn flush(&self) {}
}