use core::mem::MaybeUninit;
use shared::Packet;

// -----------------------[ FFI ]-----------------------
extern {
    // SAFETY:
    // Packets are sent/received as-is.
    #[allow(improper_ctypes)]
    fn emit(to: Connection, ptr: *const Packet);

    // SAFETY:
    // Packets are sent/received as-is.
    #[allow(improper_ctypes)]
    fn broadcast(ptr: *const Packet);

    // SAFETY:
    // 1. Packets are sent/received as-is.
    // 2. JS will initialize `from`, `ptr` with the payload
    //    or return `false`(it buffers packets, not us).
    #[allow(improper_ctypes)] 
    fn poll(from: *mut MaybeUninit<Connection>, ptr: *mut MaybeUninit<Packet>) -> bool;
}

#[no_mangle]
pub extern "C" fn packet_byte_size() -> usize {
    core::mem::size_of::<Packet>()
}
// -----------------------------------------------------

/// ID for a JS `geckos.io` channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Connection(u32);

/// I/O operations over UDP channels. 
pub struct Network;

impl Network {
    /// Iterate over incoming packets.
    pub fn poll() -> impl Iterator<Item=(Connection, Packet)> {
        core::iter::from_fn(|| {
            let mut client = MaybeUninit::<Connection>::uninit();
            let mut packet = MaybeUninit::<Packet>::uninit();

            if unsafe { poll(&mut client as _, &mut packet as _) } {
                Some(unsafe {
                    // SAFETY:
                    // The JS implementation returns `true` iff
                    // the pointers passed to it were initialized.
                    (client.assume_init(), packet.assume_init())
                })
            } else {
                None
            }
        })
    }

    /// Send a packet to a single client.
    pub fn send(to: Connection, packet: &Packet) {
        unsafe {
            // SAFETY:
            // Lifetime of the borrow is that of the function call.
            emit(to, packet as _);
        }
    }

    /// Send a packet to every client.
    pub fn broadcast(packet: &Packet) {
        unsafe {
            // SAFETY:
            // Lifetime of the borrow is that of the function call.
            broadcast(packet as _);
        }
    }
}