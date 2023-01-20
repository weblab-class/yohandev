//! Frontend for a mini "game engine," to be implemented by
//! some WebAssembly runtime.

use std::mem::{ MaybeUninit, self };
use std::ffi::{ CString, c_char };
use once_cell::sync::OnceCell;

use crate::packets::Packet;
use crate::render::Sprite;

// ----------------[ FFI ]----------------
extern {
    // SAFETY:
    // Lifetime of `ptr` can only be guarenteed for the duration
    // of the function call. Copy if needed for longer.
    fn log_info(ptr: *const c_char);
    fn log_error(ptr: *const c_char);
    fn log_warn(ptr: *const c_char);

    // SAFETY:
    // 1. Lifetime of `ptr` can only be guarenteed for the duration
    //    of the function call. Copy if needed for longer.
    // 2. Poll should return `true` iff it initialized the `ptr`.
    fn net_emit(to: Connection, ptr: *const Packet, len: usize);
    fn net_poll_packets(
        from: *mut MaybeUninit<Connection>,
        ptr: *mut MaybeUninit<Packet>
    ) -> bool;
    fn net_poll_connections(ptr: *mut MaybeUninit<Connection>) -> bool;
    fn net_poll_disconnections(ptr: *mut MaybeUninit<Connection>) -> bool;

    fn render_set_sprite(id: u32, sprite: Sprite, x: f32, y: f32);
    fn render_remove_sprite(id: u32);

    fn input_get_dx() -> f32;
    fn input_get_dy() -> f32;
}

#[no_mangle]
extern "C" fn main() {
    Logger::hook();

    crate::main();
}

#[no_mangle]
extern "C" fn tick() {
    if let Some(func) = TICK.get() {
        func();
    }
}
// ---------------------------------------

/// Callback for every tick event.
static TICK: OnceCell<fn()> = OnceCell::new();

/// Start the main event loop with the passed-in function.
pub fn run(func: fn()) {
    TICK.set(func).unwrap();
}

/// See [log].
#[derive(Default)]
pub struct Logger;

impl Logger {
    // Attaches all WASM <-> "OS" logging callbacks.
    pub fn hook() {
        log::set_logger(&Logger).unwrap();
        log::set_max_level(log::LevelFilter::Debug);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() < log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let func = match record.level() {
            log::Level::Error => log_error,
            log::Level::Warn => log_warn,
            log::Level::Info => log_info,
            log::Level::Debug => log_info,
            log::Level::Trace => log_info,
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

/// Abstraction over a networked channel.
#[derive(Default)]
pub struct Socket {
    /// Buffered packets that have been received.
    recv: Vec<(Connection, Packet)>,
    /// Buffered NEW connections since last poll, NOT every client.
    connections: Vec<Connection>,
    /// Buffered new disconnections.
    disconnections: Vec<Connection>,
}

/// Unique identifier for a networked connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Connection(u32);

impl Socket {
    /// Send an unreliable packet.
    pub fn send(&self, to: Connection, packet: &Packet) {
        unsafe {
            net_emit(to, packet as _, mem::size_of_val(packet));
        }
    }

    /// Clear the internal packet buffer and poll new ones.
    pub fn poll(&mut self) {
        self.recv.clear();
        self.connections.clear();
        self.disconnections.clear();

        // Packets
        let mut packet = MaybeUninit::uninit();
        let mut conn = MaybeUninit::uninit();
        while unsafe {
            net_poll_packets(&mut conn as _, &mut packet as _)
        } {
            self.recv.push(unsafe {
                // SAFETY:
                // Poll will return true iff initialized.
                (conn.assume_init(), packet.assume_init_read())
            });
        }
        // Connections
        while unsafe { net_poll_connections(&mut conn as _) } {
            self.connections.push(unsafe {
                // SAFETY:
                // Poll will return true if `conn` has been
                // initialized.
                conn.assume_init_read()
            });
        }
        // Disconnections
        while unsafe { net_poll_disconnections(&mut conn as _) } {
            self.disconnections.push(unsafe {
                // SAFETY:
                // Poll will return true if `conn` has been
                // initialized.
                conn.assume_init_read()
            });
        }
    }

    /// Iterate over the packets received since last tick.
    pub fn packets(&self) -> impl Iterator<Item = &(Connection, Packet)> {
        self.recv.iter()
    }

    /// Iterate over the connections since last tick.
    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.iter()
    }

    /// Iterate over the disconnections since last tick.
    pub fn disconnections(&self) -> impl Iterator<Item = &Connection> {
        self.disconnections.iter()
    }
}

/// Abstraction over a sprite renderer.
#[derive(Default)]
pub struct Canvas;

impl Canvas {
    /// Add or update the sprite associated with `id`.
    pub fn set(&self, id: u32, sprite: Sprite, x: f32, y: f32) {
        unsafe {
            render_set_sprite(id, sprite, x, y);
        }
    }

    // Remove the sprite associated with `id`.
    pub fn remove(&self, id: u32) {
        unsafe {
            render_remove_sprite(id);
        }
    }
}

/// Abstraction over keyboard/controller input.
#[derive(Default)]
pub struct Gamepad;

impl Gamepad {
    /// Query the X direction of movement
    pub fn dx(&self) -> f32 {
        unsafe {
            input_get_dx()
        }
    }

    /// Query the Y direction of movement
    pub fn dy(&self) -> f32 {
        unsafe {
            input_get_dy()
        }
    }
}