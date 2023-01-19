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

/** Enumeration of sprites in the game. */
export enum Sprite {
    Rect,
    Circle,
}

export interface Imports extends WebAssembly.ModuleImports {
    log_info(ptr: Ref<u8>): void;
    log_error(ptr: Ref<u8>): void;
    log_warn(ptr: Ref<u8>): void;

    net_emit(ptr: Ref<Packet>, len: usize): void;
    net_poll_packets(ptr: RefMut<Uninit<Packet>>): boolean;
    net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean;
    net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean;

    render_set_sprite(id: u32, sprite: Sprite, x: f32, y: f32): void;
    render_remove_sprite(id: u32): void;

    input_get_dx(): f32;
    input_get_dy(): f32;
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