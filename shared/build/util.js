/**
 * [dirname.js] Build script utilities for ESM.
 */
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

/** Root directory of the project. */
export const ROOT = join(dirname(fileURLToPath(import.meta.url)), "../..");

/**
 * Get the absolute path of a file given its path relative
 * to the project root.
 */
export function path(p) {
    return join(ROOT, p);
}