/**
 * [argv.js] Build script plugin for reading command line arguments
 *           at compile time.
 */
import parse from "mri";

export const Argv = { plugin };

/**
 * Creates an `esbuild` plugin that resolves `argv:*` imports.
 * @returns {import("esbuild").Plugin}
 */
export function plugin() {
    return {
        name: "argv",
        setup(build) {
            build.onResolve({ filter: /^argv$/ }, (args) => ({
                path: args.path,
                namespace: "argv"
            }));
            build.onLoad({ filter: /.*/, namespace: "argv" }, (_) => ({
                contents: JSON.stringify(parse(process.argv.slice(2))),
                loader: 'json',
            }));
        }
    }
}