import { Ref, Memory, Imports, Exports, u8 } from "../types";

export function imports(mem: () => Memory): Imports {
    // Decode null-terminated string
    function cstring(ptr: Ref<u8>) {
        const view = new Uint8Array(mem().buffer, ptr);
        const len = view.findIndex((c) => c == 0);
        
        return new TextDecoder().decode(view.subarray(0, len));
    }
    return {
        info(ptr: Ref<u8>) {
            console.log(cstring(ptr));
        },
        error(ptr: Ref<u8>) {
            console.error(cstring(ptr));
        },
        warn(ptr: Ref<u8>) {
            console.warn(cstring(ptr));
        },
    };
};

export function hook(exports: Exports & {
    hook(): void;
}) {
    exports.hook();
}