// @ts-ignore
import instantiate from "./lib.rs";
import { geckos } from "@geckos.io/client";
import { SVG } from "@svgdotjs/svg.js";

import * as log from "../../shared/src/log";
import * as net from "./net";

/**
 * Fetch, compile, and instantiate a new instance of the game.
 */
export async function game(port: number) {
    // Connect to server.
    const io = geckos({ port });

    // Create canvas.
    const draw = SVG().size("100%", "100%");

    // Load WebAssembly module.
    const wasm = await instantiate({
        ...log.imports(() => wasm.memory),
        ...net.imports(() => wasm.memory, io),
    });
    // Set module callbacks.
    log.hook(wasm);
    net.hook(wasm);

    // Begin game loop.
    wasm.setup();
    requestAnimationFrame(function frame(t) {
        wasm.tick(t - (this.t ?? t));
        // Delta time:
        this.t = t;
        // Request next frame:
        requestAnimationFrame(frame);
    });

    return {
        io, draw, wasm,

        /** Hook this instance to the DOM. */
        hook(node: HTMLElement) {
            draw.addTo(node);
        }
    };
}