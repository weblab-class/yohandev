// @ts-ignore
import lib from "./mod.rs";

/** Shorthand for WebAssembly.Memory */
export type Memory = WebAssembly.Memory;

/** An immutable pointer. */
export type Ref<_> = number;
/** A mutable pointer. */
export type RefMut<_> = number;

// Platform(32-bit)
export type isize = number;
export type usize = number;
// 8-bit
export type u8 = number;
export type i8 = number;
// 16-bit
export type u16 = number;
export type i16 = number;
// 32-bit
export type u32 = number;
export type i32 = number;
// 64-bit
export type u64 = number;
export type i64 = number;
// Floating-point
export type f32 = number;
export type f64 = number;

/** Marks some memory as uninitialized. */
export type Uninit<_> = void;

/** Opaque type representing a packet. */
export type Packet = void;
/** Unique identifier for a channel. */
export type Connection = number;

export enum HandheldSpriteKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
}

export interface Imports extends WebAssembly.ModuleImports {
    log_info(ptr: Ref<u8>): void;
    log_error(ptr: Ref<u8>): void;
    log_warn(ptr: Ref<u8>): void;

    net_emit(to: Connection, ptr: Ref<Packet>, len: usize): void;
    net_broadcast(ptr: Ref<Packet>, len: usize): void;
    net_poll_packets(
        from: RefMut<Uninit<Connection>>,
        ptr: RefMut<Uninit<Packet>>
    ): boolean;
    net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean;
    net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean;

    render_set_player_sprite(id: u32, x: f32, y: f32, skew: f32, sx: f32, sy: f32): void;
    render_set_bullet_sprite(id: u32, x: f32, y: f32): void;
    render_set_handheld_sprite(id: u32, kind: HandheldSpriteKind, x: f32, y: f32);
    render_remove_sprite(id: u32): void;

    input_get_dx(): f32;
    input_get_dy(): f32;
    input_get_ax(): f32;
    input_get_ay(): f32;
    input_get_button(i: usize): boolean;
    input_set_player_position(x: f32, y: f32): void;

    time_now(): u32;
}

export interface Exports extends WebAssembly.Exports {
    memory: Memory;

    main(): void;
    tick(): void;
}

/** Fetch, compile, and instantiate the WebAssembly module. */
export async function instantiate(imports: Imports): Promise<Exports> {
    return lib(imports);
}

/** Decode a null-terminated string */
export function cstring(mem: Memory, ptr: Ref<u8>): string {
    const view = new Uint8Array(mem.buffer, ptr);
    const len = view.findIndex((c) => c == 0);
    
    return new TextDecoder().decode(view.subarray(0, len));   
}