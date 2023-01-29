/**
 * Platform implementation for a web browser.
 */
import { geckos, RawMessage } from "@geckos.io/client";
import { Shape, Svg, SVG } from "@svgdotjs/svg.js";
import {
    Memory, Ref, RefMut, Uninit,
    cstring,
    Packet, Connection,
    Costume, Visibility,
    usize, u32, f32, u8, f64,
    instantiate,
} from "./mod";

export async function game(port: number) {
    const draw = SVG();
    const wasm = await instantiate({
        ...Log.imports(() => wasm.memory),
        ...Net.imports(() => wasm.memory, port),
        ...Render.imports(() => wasm.memory, draw),
        ...Input.imports(),
        ...Time.imports(),
    });
    wasm.main();
    requestAnimationFrame(function loop(_time) {
        requestAnimationFrame(loop);
        wasm.tick();
    });

    return {
        hook(node: HTMLElement): void {
            draw.addTo(node);
        }
    }
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
    export function imports(mem: () => Memory, port: number = 8000) {
        const channel = geckos({ port });
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
    export function imports(mem: () => Memory, draw: Svg) {
        draw.size("100%", "100%")
            .addClass("cartesian");
        
        // Entity -> SVG cache
        const cache = {
            inner: <Shape[]>[],
            free: <u32[]>[],
            add(element: Shape) {
                // Use free or create new slot
                const handle = cache.free.length
                    ? cache.free.pop()!
                    : cache.inner.push(element) - 1;

                cache.inner[handle] = element;
                return handle;
            },
            drop(handle: u32) {
                delete cache.inner[handle];
                cache.free.push(handle);
            },
            get(handle: u32) {
                return cache.inner[handle];
            }
        };
        
        // Parse `Ref<Costume>` enum
        function costume(ptr: Ref<Costume>): [tag: Costume, args: Float32Array] {
            return [
                new Uint32Array(mem().buffer, ptr)[0],
                new Float32Array(mem().buffer, ptr + 4),
            ];
        }
        return {
            render_new_sprite(ptr: Ref<Costume>): u32 {
                // Creates a new SVG element for the costume
                const element = () => {
                    switch (costume(ptr)[0]) {
                        case Costume.Player:
                            return draw.rect(30, 50).fill("#EFC643");
                        case Costume.Bullet:
                            return draw.circle(3);
                        case Costume.Shotgun:
                            return draw
                                .group()
                                .add(draw
                                    .image("assets/weapons/shotgun.svg")
                                    .scale(0.2, -0.2)
                                );
                    }
                };
                return cache.add(element());
            },
            render_update_sprite(handle: u32, ptr: Ref<Costume>, visibility: Visibility) {
                const [tag, args] = costume(ptr);
                const element = cache.get(handle);

                // Visibility
                switch (visibility) {
                    case Visibility.Shown:
                        element.show();
                        break;
                    case Visibility.Hidden:
                        element.hide();
                        break;
                }
                // Position
                switch (tag) {
                    case Costume.Player:
                    case Costume.Bullet:
                    case Costume.Shotgun:
                        element
                            .x(args[0])
                            .y(args[1]);
                }
                // Rotation
                switch (tag) {
                    case Costume.Shotgun:
                        element.transform({
                            rotate: 180 / Math.PI * args[2],
                            scaleY: Math.abs(args[2]) > Math.PI / 2 ? -1 : 1,
                        });
                }
                // Player
                if (tag == Costume.Player) {
                    element.transform({
                        scaleX: args[2],
                        scaleY: args[3],
                        skewX: args[4],
                    });
                }
            },
            render_drop_sprite(handle: u32) {
                // Remove from DOM
                cache.get(handle)?.remove();
                // Remove from cache
                cache.drop(handle);
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
        // Mouse control emulates a joystick
        const origin = { x: 0.0, y: 0.0 };
        
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
        // Emulate joystick using the mouse
        function emulateJoystick(cx: number, cy: number) {
            const x = (buf["MouseX"] ?? 0) - cx;
            const y = (buf["MouseY"] ?? 0) - cy;
            const v = Math.sqrt(x*x + y*y) ?? 1.0;

            return { x: x / v, y: y / v };
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
        document.addEventListener("mousemove", (e) => {
            buf["MouseX"] = e.clientX;
            buf["MouseY"] = (e.view?.innerHeight ?? 0) - e.clientY;
        });
        return {
            input_get_dx(): f32 {
                return axis(bindings.axes.x);
            },
            input_get_dy(): f32 {
                return axis(bindings.axes.y);
            },
            input_get_ax(): f32 {
                return emulateJoystick(origin.x, origin.y).x;
            },
            input_get_ay(): f32 {
                return emulateJoystick(origin.x, origin.y).y;
            },
            input_get_button(i: usize): boolean {
                if (i >= bindings.buttons.length) {
                    return false;
                }
                return bindings.buttons[i].some((b) => buf[b] > 0)
            },
            input_set_player_position(x: f32, y: f32): void {
                // if (controls is joystick) return
                origin.x = x;
                origin.y = y;
            }
        }
    }
}

module Time {
    export function imports() {
        return {
            time_now(): u32 {
                return window.performance ? performance.now() : Date.now();
            },
            now(): f64 {
                return Date.now();
            }
        }
    }
}