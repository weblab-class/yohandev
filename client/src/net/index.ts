import { Exports, Memory, Ref, RefMut, Uninit, usize } from "../../../shared/src/types";
import { ClientChannel, RawMessage } from "@geckos.io/client";

/** Opaque type to represent a packet. */
type Packet = RawMessage;
/** Size of a packet; constant but determined at runtime. */
let PACKET_SIZE: number;

export function imports(mem: () => Memory, io: ClientChannel) {
    // Queue of received packets
    const rx: Packet[] = [];
    // Setup callbacks:
    io.onConnect((e) => {
        if (e) throw e;
    });
    io.onRaw((packet) => {
        rx.push(packet);
    });

    return {
        emit(ptr: Ref<Packet>) {
            // SAFETY:
            // A copy of the packet is made so it can be safely
            // buffered without exceeding the original lifetime.
            io.raw.emit(mem().buffer.slice(ptr, ptr + PACKET_SIZE));
        },
        poll(ptr: RefMut<Uninit<Packet>>): boolean {
            if (!rx.length) {
                return false;
            }
            const packet = new Uint8Array(<ArrayBuffer>rx.shift());
            const view = new Uint8Array(mem().buffer, ptr, PACKET_SIZE);
            
            // SAFETY:
            // Caller guarentees that `ptr` points to uninitialzed
            // memory of size `sizeof(Packet)`.
            view.set(packet);

            return true;
        }
    };
}

export function hook(exports: Exports & {
    packet_byte_size(): usize;
    ping(): void;
}) {
    // Initialize constants:
    PACKET_SIZE = exports.packet_byte_size();
}