/**
 * Platform implementation for a web browser.
 */
import { geckos, RawMessage } from "@geckos.io/client";
import {
    Memory, Ref, RefMut, Uninit,
    cstring,
    Packet, Connection,
    Sprite,
    usize, u32, f32, u8,
    instantiate,
} from "./mod";

export async function game() {
    const wasm = await instantiate({
        ...Log.imports(() => wasm.memory),
        ...Net.imports(() => wasm.memory),
        ...Render.imports(),
        ...Input.imports(),
    });
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
    export function imports(mem: () => Memory) {
        // TODO: this shouldn't be hard-coded
        const channel = geckos({ port: 8000 });
        
        // Buffer incoming messages:
        let rx: RawMessage[] = [];
        // Server-assigned client ID:
        let id: Connection;
        let event: "connected" | "disconnected" | undefined;
        
        channel.onConnect((e) => {
            if (e) throw e;
        });
        channel.onDisconnect((e) => {
            if (e) throw e;
            event = "disconnected";
        });
        channel.onRaw((msg) => {
            rx.push(msg);
        });
        channel.on("whoami", (msg) => {
            // Actual connection established after this exchange:
            id = <Connection> msg["id"]
            event = "connected";
        });

        return {
            net_emit(to: Connection, ptr: Ref<Packet>, len: usize): void {
                console.assert(to === id);
                channel.raw.emit(
                    // SAFETY:
                    // Lifetime of the packet is extended since `slice`
                    // creates a copy.
                    mem().buffer.slice(ptr, ptr + len)
                );
            },
            net_poll_packets(
                from: RefMut<Uninit<Connection>>,
                ptr: RefMut<Uninit<Packet>>
            ): boolean {
                // Exhausted or haven't received whoami yet.
                if (!rx.length || id === undefined) {
                    return false;
                }
                const payload = new Uint8Array(<ArrayBuffer>rx.shift());
                const packet = new Uint8Array(mem().buffer, ptr);
                const conn = new Uint32Array(mem().buffer, from);
                
                // SAFETY:
                // Caller guarentees the pointers are of correct size.
                packet.set(payload);
                conn.set([id]);
    
                return true;
            },
            net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (id === undefined || event !== "connected") {
                    return false;
                }
                event = undefined;

                // SAFETY:
                // Caller guarentees the pointer is of correct size.
                new Uint32Array(mem().buffer, ptr).set([id]);

                return true;
            },
            net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (id === undefined || event !== "disconnected") {
                    return false;
                }
                event = undefined;

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