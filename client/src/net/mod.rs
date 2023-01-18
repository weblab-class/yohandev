use std::mem::MaybeUninit;
use shared::Packet;

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
    fn poll(ptr: *mut MaybeUninit<Packet>) -> bool;
}

#[no_mangle]
pub extern "C" fn packet_byte_size() -> usize {
    core::mem::size_of::<Packet>()
}

#[no_mangle]
pub extern "C" fn ping() {
    unsafe {
        // SAFETY:
        // Lifetime of the borrow is that of the function call.
        emit(&Packet::Ping);
    }
    log::info!("Pinging!");
}