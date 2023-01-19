import {
    Memory, Ref, RefMut, Uninit,
    Packet, Connection,
    Sprite,
    usize, u32, f32, u8,
    instantiate,
} from "./mod";

/**
 * Platform implementation for a web browser.
 */
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

            },
            log_error(ptr: Ref<u8>): void {

            },
            log_warn(ptr: Ref<u8>): void {

            },
        }
    }
}

module Net {
    export function imports(mem: () => Memory) {
        return {
            net_emit(ptr: Ref<Packet>, len: usize): void {

            },
            net_poll_packets(ptr: RefMut<Uninit<Packet>>): boolean {
                return false;
            },
            net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean {
                return false;
            },
            net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean {
                return false;
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