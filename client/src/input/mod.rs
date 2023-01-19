// -----------------------[ FFI ]-----------------------
extern {
    fn poll_key(code: Key) -> bool;
}
// -----------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Key {
    Up      = 0x0,
    Down    = 0x1,
    Left    = 0x2,
    Right   = 0x3,
}

impl Key {
    pub fn variants() -> impl Iterator<Item = &'static Key> {
        const KEYS: &[Key] = &[
            Key::Up,
            Key::Down,
            Key::Left,
            Key::Right,
        ];
        KEYS.iter()
    }
}

/// Bit-mask of the [Key]s that are pressed.
static mut PRESSED: u32 = 0;

/// System that polls buffered inputs each frame.
pub fn poll() {
    for &key in Key::variants() {
        let pressed = unsafe {
            // SAFETY:
            // Just a normal function call.
            poll_key(key)
        };
        unsafe {
            // SAFETY:
            // WebAssembly is single threaded so...
            PRESSED &= !(1 << key as u32);
            PRESSED |= (pressed as u32) << (key as u32);
        }
    }
}

/// Get whether the key is down this frame.
pub fn is_down(code: Key) -> bool {
    unsafe {
        // SAFETY:
        // WebAssembly is single threaded so...
        (PRESSED & (1 << code as u32)) != 0
    }
}