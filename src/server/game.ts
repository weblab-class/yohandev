import { GeckosServer } from "@geckos.io/server";
import { game } from "../platform/node";

/** Singleton instance of the game. */
export let instance;

export async function init(io: GeckosServer) {
    instance = await game(io);
}