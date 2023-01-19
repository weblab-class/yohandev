/**
 * Headless platform implementation for node.
 */
import { GeckosServer, RawMessage, ServerChannel } from "@geckos.io/server";
import {
    Memory, Ref, RefMut, Uninit,
    cstring,
    Packet, Connection,
    Sprite,
    usize, u32, f32, u8,
    instantiate,
} from "./mod";

export async function game(io: GeckosServer) {
    const wasm = await instantiate({
        ...Log.imports(() => wasm.memory),
        ...Net.imports(() => wasm.memory, io),
        ...Render.imports(),
        ...Input.imports(),
    });
    wasm.main();
    setInterval(function loop() {
        wasm.tick();
    }, 25)
}

module Log {
    export function imports(mem: () => Memory) {
        return {
            log_info(ptr: Ref<u8>): void {
                console.log(cstring(mem(), ptr));
            },
            log_error(ptr: Ref<u8>): void {
                console.error(cstring(mem(), ptr));
            },
            log_warn(ptr: Ref<u8>): void {
                console.warn(cstring(mem(), ptr));
            },
        }
    }
}

module Net {
    export function imports(mem: () => Memory, io: GeckosServer) {
        // Maps connection IDs to their client channel.
        const clients: { [_: Connection]: ServerChannel } = {};
        // Buffer incoming messages:
        const rx: [Connection, RawMessage][] = [];
        // Buffer new (dis)connections:
        const connections: Connection[] = [];
        const disconnections: Connection[] = [];

        // Assigns client IDs. For a small "lobby" this is fine.
        let nextId: Connection = 0;

        io.onConnection((channel) => {
            const id = nextId++;

            channel.onRaw((msg) => {
                rx.push([id, msg]);
            });
            channel.onDisconnect((_) => {
                delete clients[id];
                // Notify lib
                disconnections.push(id);
            });
            // Special event, client only connects when true.
            channel.emit("whoami", { id }, { reliable: true });
            // Notify lib
            connections.push(id);

            clients[id] = channel;
        });

        return {
            net_emit(to: Connection, ptr: Ref<Packet>, len: usize): void {
                if (!clients.hasOwnProperty(to)) {
                    return;
                }
                clients[to].raw.emit(
                    // SAFETY:
                    // Lifetime of the packet is extended via cloning.
                    mem().buffer.slice(ptr, ptr + len)
                );
            },
            net_poll_packets(
                from: RefMut<Uninit<Connection>>,
                ptr: RefMut<Uninit<Packet>>
            ): boolean {
                if (!rx.length) {
                    return false;
                }
                const [id, msg] = rx.shift()!;
                const payload = new Uint8Array(msg as ArrayBuffer);
                const packet = new Uint8Array(mem().buffer, ptr);
                const conn = new Uint32Array(mem().buffer, from);

                // SAFETY:
                // Caller guarentees the pointers are of correct size.
                packet.set(payload);
                conn.set([id]);

                return true;
            },
            net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (!connections.length) {
                    return false;
                }
                const id = connections.shift()!;
                // SAFETY:
                // Caller guarentees the pointer is of correct size.
                new Uint32Array(mem().buffer, ptr).set([id]);

                return true;
            },
            net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (!disconnections.length) {
                    return false;
                }
                const id = disconnections.shift()!;
                // SAFETY:
                // Caller guarentees the pointer is of correct size.
                new Uint32Array(mem().buffer, ptr).set([id]);

                return true;
            },
        }
    }
}

module Render {
    export function imports() {
        return {
            render_set_sprite(id: u32, sprite: Sprite, x: f32, y: f32): void {

            },
            render_remove_sprite(id: u32): void {

            },
        }
    }
}

module Input {
    export function imports() {
        return {
            input_get_dx(): f32 {
                return 0.0;
            },
            input_get_dy(): f32 {
                return 0.0;
            },
        }
    }
}