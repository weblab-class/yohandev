import { Exports } from "../../../shared/src/types";

/** Key codes relevant to the game. */
enum Key {
    Up      = 0x0,
    Down    = 0x1,
    Left    = 0x2,
    Right   = 0x3,
}
/** Keyboard bindings. TODO: configurable as part of `imports` arg. */
const bindings = {
    [Key.Up]: "ArrowUp",
    [Key.Down]: "ArrowDown",
    [Key.Left]: "ArrowLeft",
    [Key.Right]: "ArrowRight",
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
        poll_key(code: Key): boolean {
            return down[bindings[code]];
        }
    }
}

export function hook(exports: Exports & {
    /** No exports */
}) {

}