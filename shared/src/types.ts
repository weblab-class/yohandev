/** Short-hand for WASM memory. */
export type Memory = WebAssembly.Memory;
/** Short-hand for WASM imports. */
export type Imports = WebAssembly.ModuleImports;
/** Short-hand for WASM exports. */
export type Exports = WebAssembly.Exports & { memory: Memory };

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