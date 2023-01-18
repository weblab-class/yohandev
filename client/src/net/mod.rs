use shared::Packet;
use core::mem::MaybeUninit;

pub mod spawn;

// -----------------------[ FFI ]-----------------------
extern {
    // SAFETY:
    // Packets are sent/received as-is.
    #[allow(improper_ctypes)]
    fn emit(ptr: *const Packet);

    // SAFETY:
    // 1. Packets are sent/received as-is.
    // 2. JS will initialize `ptr` with the payload, or
    //    return `false`(it buffers packets, not us).
    #[allow(improper_ctypes)] 
    fn poll_packets(ptr: *mut MaybeUninit<Packet>) -> bool;
}

#[no_mangle]
pub extern "C" fn packet_byte_size() -> usize {
    core::mem::size_of::<Packet>()
}
// -----------------------------------------------------

/// Iterate over incoming packets.
pub fn poll() -> impl Iterator<Item=Packet> {
    core::iter::from_fn(|| {
        let mut packet = MaybeUninit::<Packet>::uninit();

        if unsafe { poll_packets(&mut packet as _) } {
            Some(unsafe {
                // SAFETY:
                // The JS implementation returns `true` iff
                // the pointers passed to it were initialized.
                packet.assume_init()
            })
        } else {
            None
        }
    })
}

/// Send a packet to the server.
pub fn send(packet: &Packet) {
    unsafe {
        // SAFETY:
        // Lifetime of the borrow is that of the function call.
        emit(packet as _);
    }
}