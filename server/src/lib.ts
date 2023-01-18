// @ts-ignore
import instantiate from "./lib.rs";
import { GeckosServer } from "@geckos.io/server";

import * as log from "../../shared/src/log";
import * as net from "./net";

/**
 * Fetch, compile, and instantiate a new instance of the game.
 */
export async function game(io: GeckosServer) {
    // Load WebAssembly module.
    const wasm = await instantiate({
        ...log.imports(() => wasm.memory),
        ...net.imports(() => wasm.memory, io),
    });
    // Set module callbacks.
    log.hook(wasm);
    net.hook(wasm);

    // Tick:
    setInterval(() => wasm.tick(), 50);

    return { wasm };
}