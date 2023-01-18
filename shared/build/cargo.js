/**
 * [cargo.js] Build script plugin for Cargo.
 */
import { dirname, join, resolve } from "node:path";
import { readFile } from "node:fs/promises";
import { execa as exec } from "execa";

export const Cargo = { plugin, build, manifestPath };

/**
 * Invokes `cargo build` from any path within the project.
 */
export async function build(path) {
    // Only `wasm32-unknown-unknown` makes sense here.
    const TARGET = "wasm32-unknown-unknown";
    const FORMAT = "json-diagnostic-rendered-ansi";
    const ARGS = [
        "build",
        "--target", TARGET,
        "--message-format", FORMAT,
    ];

    const messages = await exec("cargo", ARGS, { cwd: dirname(path), reject: false })
        .then(({ stdout }) => stdout
            .split(/\r?\n/)
            .map((s) => JSON.parse(s))
        );
    const artifact = messages.filter((m) => m["reason"] === "compiler-artifact");
    const diagnostics = messages.filter((m) => m["reason"] === "compiler-message");

    return {
        artifact: artifact
            .flatMap((m) => m["filenames"])
            .find((p) => p.endsWith(".wasm")),
        errors: diagnostics
            .filter((m) => m["message"]["level"] === "error")
            .map((m) => m["message"]["rendered"]),
        warnings: diagnostics
            .filter((m) => m["message"]["level"] === "warning")
            .map((m) => m["message"]["rendered"]),
    };
}

/**
 * Reports the absolute path of the nearest `Cargo.toml`.
 */
export async function manifestPath(path) {
    return exec("cargo", ["locate-project"], { cwd: dirname(path) })
        .then(({ stdout, exitCode }) => {
            return exitCode == 0 ? JSON.parse(stdout)["root"] : undefined;
        });
}

/**
 * Creates an `esbuild` plugin that compiles `.rs` imports into WebAssembly.
 * @argument {"fetch" | "inline"} impl
 * @returns {import("esbuild").Plugin}
 */
export function plugin(impl) {
    // 1. Resolve project source to its manifest.
    async function resolveSource(args) {
        const path = resolve(args.resolveDir, args.path);
        const manifest = await Cargo.manifestPath(path);
        // Resolve to the project's manifest.
        return {
            path: manifest,
            namespace: "cargo-stub",
            errors: manifest ? [] : [{
                pluginName: PLUGIN_NAME,
                text: `Could not find \`Cargo.toml\` in \`${path}\` or any parent directory.`
            }]
        };
    }
    // 2. Resolve manifest to itself.
    async function resolveManifest(args) {
        // Loading `.wasm` from within the virtual module
        if (args.namespace === "cargo-stub") {
            return {
                // Rename file, otherwise binary is emmited with `.toml`
                path: join(dirname(args.path), "cargo.wasm"),
                namespace: "cargo-binary",
            };
        }
        // Unresolveable(ignored)
        if (args.resolveDir === "") return;
        // Resolve to virtual module
        return {
            path: resolve(args.resolveDir, args.path),
            namespace: "cargo-stub",
        };
    }
    // 3. Virtual JS module to instantiate the WebAssembly artifact.
    function loadStub(args) {
        switch (impl) {
            case "fetch": return {
                contents: `
                    import wasm from "${args.path}";

                    export default async (env) => (
                        WebAssembly.instantiateStreaming(fetch(wasm), { env })
                            .then((obj) => obj.instance.exports)
                    );
                `
            };
            case "inline": return {
                contents: `
                    import wasm from "${args.path}";

                    export default async (env) => (
                        WebAssembly.instantiate(wasm, { env })
                            .then((obj) => obj.instance.exports)
                    )
                `
            };
            default: throw `Unknown implementation "${impl}".`;
        }
    }
    // 4. Compile & copy the WebAssembly artifact.
    async function loadBinary(args) {
        const {
            artifact,
            errors,
            warnings
        } = await Cargo.build(args.path);

        return {
            contents: artifact && await readFile(artifact),
            errors: errors.map((text) => ({ text, detail: { rendered: true } })),
            warnings: warnings.map((text) =>({ text, detail: { rendered: true } })),
            loader: impl == "fetch" ? "file" : "binary",
        }
    }
    return {
        name: "cargo",
        setup(build) {
            build.onResolve({ filter: /\.rs$/ }, resolveSource);
            build.onResolve({ filter: /Cargo\.toml$/ }, resolveManifest);
            build.onLoad({ filter: /.*/, namespace: "cargo-stub" }, loadStub);
            build.onLoad({ filter: /.*/, namespace: "cargo-binary" }, loadBinary);
        }
    }
}