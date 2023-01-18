import { Exports, Memory, Ref, RefMut, Uninit, usize } from "../../../shared/src/types";
import { ServerChannel, RawMessage, GeckosServer } from "@geckos.io/server";

/** Opaque type to represent a packet. */
type Packet = RawMessage;
/** ID for a connection. */
type Connection = number;

/** Size of a packet; constant but determined at runtime. */
let PACKET_SIZE: number;

export function imports(mem: () => Memory, io: GeckosServer) {
    // Queue of received packets.
    const rx: [Connection, Packet][] = [];
    // Currently connected players.
    const clients: { [_: Connection]: ServerChannel } = {};
    // Next connection ID.
    let nextId: Connection = 0;

    // Setup callbacks:
    io.onConnection((channel) => {
        const id = nextId++;

        channel.onRaw((packet) => {
            rx.push([id, packet]);
        });
        channel.onDisconnect((_) => {
            delete clients[id];
        });
        clients[id] = channel;
    })

    // Read a packet from memory and extend its lifetime via cloning.
    function packet(ptr: Ref<Packet>) {
        return mem().buffer.slice(ptr, ptr + PACKET_SIZE);
    }
    return {
        emit_packet(to: Connection, ptr: Ref<Packet>) {
            if (!clients[to]) {
                return;
            }
            // SAFETY:
            // A copy of the packet is made so it can be safely
            // buffered without exceeding the original lifetime.
            clients[to].raw.emit(packet(ptr));
        },
        broadcast_packet(ptr: Ref<Packet>) {
            // SAFETY:
            // A copy of the packet is made so it can be safely
            // buffered without exceeding the original lifetime.
            io.raw.emit(packet(ptr));
        },
        poll_packets(from: RefMut<Uninit<Connection>>, ptr: RefMut<Uninit<Packet>>): boolean {
            if (!rx.length) {
                return false;
            }
            const [client, payload] = rx.shift()!;
            const packet = new Uint8Array(<ArrayBuffer>payload);
            const view1 = new Uint8Array(mem().buffer, ptr, PACKET_SIZE);
            const view2 = new Uint32Array(mem().buffer, from, 1);

            // SAFETY:
            // Caller guarentees that `from` `ptr` points to
            // uninitialzed memories of sizes `sizeof(Connection)`
            // and `sizeof(Packet)`, respectively.
            view1.set(packet);
            view2.set([client]);

            return true;
        }
    };
}

export function hook(exports: Exports & {
    packet_byte_size(): usize;
}) {
    // Initialize constants:
    PACKET_SIZE = exports.packet_byte_size();
}