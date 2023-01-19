import { Exports, i8 } from "../../../shared/src/types";

/** Keyboard bindings. TODO: configurable as part of `imports` arg. */
const bindings = {
    "up": "ArrowUp",
    "down": "ArrowDown",
    "left": "ArrowLeft",
    "right": "ArrowRight",
}

export function imports() {
    // Buffer key presses since it might happen between polls.
    const down = {};
    document.addEventListener("keydown", (e) => {
        down[e.key] = true;
    });
    document.addEventListener("keyup", (e) => {
        down[e.key] = false;
    })

    return {
        poll_input_dx(): i8 {
            const l = down[bindings["left"]];
            const r = down[bindings["right"]];
            // TODO: better system than this
            return (l && !r) ? -128 : (r && !l) ? 127 : 0;
        },
        poll_input_dy(): i8 {
            const u = down[bindings["down"]];
            const d = down[bindings["up"]];
            // TODO: better system than this
            return (u && !d) ? -128 : (d && !u) ? 127 : 0;
        }
    }
}

export function hook(exports: Exports & {
    /** No exports */
}) {

}