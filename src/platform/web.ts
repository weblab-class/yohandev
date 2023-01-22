/**
 * Platform implementation for a web browser.
 */
import { geckos, RawMessage } from "@geckos.io/client";
import { Shape, SVG } from "@svgdotjs/svg.js";
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
        ...Time.imports(),
    });
    wasm.main();
    requestAnimationFrame(function loop(_time) {
        requestAnimationFrame(loop);
        wasm.tick();
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
        const rx: RawMessage[] = [];
        
        // Server-assigned client ID:
        let id: Connection;
        // Event flags
        let connected = false;
        let disconnected = false;
        
        channel.onConnect((e) => {
            if (e) throw e;
        });
        channel.onDisconnect((e) => {
            if (e) throw e;
            disconnected = true;
        });
        channel.onRaw((msg) => {
            rx.push(msg);
        });
        channel.on("whoami", (msg) => {
            // Actual connection established after this exchange:
            id = msg["id"] as Connection;
            connected = true;
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
            net_broadcast(ptr: Ref<Packet>, len: usize): void {
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
                const payload = new Uint8Array(rx.shift() as ArrayBuffer);
                const packet = new Uint8Array(mem().buffer, ptr);
                const conn = new Uint32Array(mem().buffer, from);
                
                // SAFETY:
                // Caller guarentees the pointers are of correct size.
                packet.set(payload);
                conn.set([id]);
    
                return true;
            },
            net_poll_connections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (id === undefined || !connected) {
                    return false;
                }
                connected = false;

                // SAFETY:
                // Caller guarentees the pointer is of correct size.
                new Uint32Array(mem().buffer, ptr).set([id]);

                return true;
            },
            net_poll_disconnections(ptr: RefMut<Uninit<Connection>>): boolean {
                if (id === undefined || !disconnected) {
                    return false;
                }
                disconnected = false;

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
        // TODO: somehow return this to attach to DOM node
        const draw = SVG()
            .size("100%", "100%")
            .addClass("cartesian")
            .addTo(document.body);
        // Entity -> SVG cache
        const cache: {
            [id: u32]: {
                sprite: Sprite,
                shape: Shape,
            }
        } = {};
        return {
            render_set_sprite(id: u32, sprite: Sprite, x: f32, y: f32): void {
                // Create shape
                if (!cache.hasOwnProperty(id) || cache[id].sprite !== sprite) {
                    cache[id]?.shape.remove();
                    cache[id] = {
                        sprite,
                        shape: (() => {
                            switch (sprite) {
                                case Sprite.Rect: return draw.rect(20, 50);
                                case Sprite.Circle: return draw.circle(30);
                            }
                        })(),
                    };
                }
                // Update shape
                cache[id].shape.x(x).y(y);
            },
            render_remove_sprite(id: u32): void {
                // Remove from DOM
                cache[id]?.shape.remove();
                // Remove from cache
                delete cache[id];
            },
        }
    }
}

module Input {
    export function imports() {
        // TODO: make this programmatic
        const bindings = {
            axes: <{ [_: string]: [string, string] }>{
                x: ["ArrowLeft", "ArrowRight"],
                y: ["ArrowDown", "ArrowUp"],
            },
            buttons: [
                ["1", "Mouse0"],
                ["2", "Mouse2"],
                ["3"],
                ["4"],
            ]
        };
        // Buffer key presses and their timing(negative is release).
        const buf: { [key: string]: number } = {};
        
        // Calculate the value of an axis
        function axis([neg, pos]: [string, string]): f32 {
            const negv = buf[neg] ?? 0;
            const posv = buf[pos] ?? 0;

            if (posv > negv) {
                return posv > 0 ? 1.0 : 0.0;
            } else {
                return negv > 0 ? -1.0 : 0.0;
            }
        }
        document.addEventListener("keydown", (e) => {
            buf[e.key] = e.timeStamp;
        });
        document.addEventListener("keyup", (e) => {
            buf[e.key] = -e.timeStamp;
        });
        document.addEventListener("mousedown", (e) => {
            buf["Mouse" + e.button] = e.timeStamp;
        });
        document.addEventListener("mouseup", (e) => {
            buf["Mouse" + e.button] = -e.timeStamp;
        });
        return {
            input_get_dx(): f32 {
                return axis(bindings.axes.x);
            },
            input_get_dy(): f32 {
                return axis(bindings.axes.y);
            },
            input_get_button(i: usize): boolean {
                if (i >= bindings.buttons.length) {
                    return false;
                }
                return bindings.buttons[i].some((b) => buf[b] > 0)
            },
        }
    }
}

module Time {
    export function imports() {
        return {
            time_now(): u32 {
                return window.performance ? performance.now() : Date.now();
            }
        }
    }
}