/**
 * Build script for server & client.
 */
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

import esbuild from "esbuild";
import args from "args";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const FLAGS = args
    .option("config", "The build configuration(s) to build")
    .option("run", "Optionally run the produced server artifact")
    .option("port", "The port to build and serve on.", 8000)
    .parse(process.argv);

const CONFIGS = {
    /** @type {import("esbuild").BuildOptions} */
    "server": () => ({
        // Bundle
        entryPoints: [path("src/server/app.ts")],
        outfile: path("build/app.js"),
        bundle: true,
        plugins: [
            Cargo.plugin("inline", ["server"]),
            Toml.plugin(),
            Env.plugin(),
        ],

        // Node
        format: "esm",
        packages: "external",
        platform: "node",
    }),
    /** @type {import("esbuild").BuildOptions} */
    "client": () => ({
        // Bundle
        entryPoints: [path("src/www/app.js")],
        outfile: path("build/pkg/app.js"),
        bundle: true,
        minify: true,
        sourcemap: true,
        plugins: [
            Cargo.plugin("fetch", ["client"]),
            Toml.plugin(),
            Env.plugin(),
        ],

        // Preact
        jsxFactory: 'h',
        jsxFragment: 'Fragment',
        jsxImportSource: 'preact',
        jsx: 'automatic',
        loader: {
            '.js': 'jsx',
        },
    }),
};

/**
 * Get the absolute path of a file given its path relative
 * to the project root.
 */
function path(p) {
    return join(ROOT, p);
}

/**
 * ESBuild plugin for Cargo.
 */
import { resolve } from "node:path";
import { readFile } from "node:fs/promises";
import { execa as exec } from "execa";

const Cargo = {
    /**
     * Invokes `cargo build` from any path within the project.
     */
    async build(path, cfg=[]) {
        // Only `wasm32-unknown-unknown` makes sense here.
        const TARGET = "wasm32-unknown-unknown";
        const FORMAT = "json-diagnostic-rendered-ansi";
        const ARGS = [
            "rustc",
            "--target", TARGET,
            "--message-format", FORMAT,
            "--",
            "--cfg", cfg.join(),
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
    },

    /**
     * Reports the absolute path of the nearest `Cargo.toml`.
     */
    async manifestPath(path) {
        return exec("cargo", ["locate-project"], { cwd: dirname(path) })
            .then(({ stdout, exitCode }) => {
                return exitCode == 0 ? JSON.parse(stdout)["root"] : undefined;
            });
    },

    /**
     * Creates an `esbuild` plugin that compiles `.rs` imports into WebAssembly.
     * @argument {"fetch" | "inline"} impl
     * @returns {import("esbuild").Plugin}
     */
    plugin(impl, cfg=[]) {
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
            } = await Cargo.build(args.path, cfg);

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
        };
    },
};

/**
 * ESBuild plugin for TOML.
 */
import { parse as parseToml } from "toml";

const Toml = {
    /**
     * Creates an `esbuild` plugin that imports `.toml` as JS objects.
     * @returns {import("esbuild").Plugin}
     */   
    plugin() {
        // 1. Resolve all `.toml`
        function resolveToml(args) {
            return {
                path: resolve(args.resolveDir, args.path),
                namespace: "toml"
            };
        }
        // 2. Parse and import as JSON
        async function loadToml(args) {
            const toml = await readFile(args.path);
            const json = parseToml(toml);
            return {
                contents: JSON.stringify(json),
                loader: "json"
            };
        }
        return {
            name: "toml",
            setup(build) {
                build.onResolve({ filter: /\.toml$/ }, resolveToml);
                build.onLoad({ filter: /.*/, namespace: "toml" }, loadToml);
            }
        };
    }
}

import parse from "mri";

/**
 * ESBuild plugin for command-line arguments and environment variables.
 */
const Env = {
    /**
     * Creates an `esbuild` plugin that imports compile-time constants from `env`.
     * @returns {import("esbuild").Plugin}
     */
    plugin() {
        // 1. Resolve all `env`
        function resolveEnv(args) {
            return {
                path: args.path,
                namespace: "argv"
            }
        }
        // 2. Load environment variables and CLI arguments
        function loadEnv(_) {
            return {
                contents: JSON.stringify({
                    ...process.env,
                    ...parse(process.argv.slice(2)),
                }),
                loader: 'json'
            }
        }
        return {
            name: "env",
            setup(build) {
                build.onResolve({ filter: /^env$/ }, resolveEnv);
                build.onLoad({ filter: /.*/, namespace: "argv" }, loadEnv);
            }
        }
    }
}

import { fork } from "node:child_process";

/**
 * Actual build script.
 */
(async () => {
    // Check arguments
    if (!FLAGS["config"]) {
        console.error("Missing build configuration!");
        return;
    }
    // Build every specified configuration
    await Promise.all(
        [FLAGS["config"].split(",")]
            .flat()
            .map((name) => CONFIGS[name]())
            .map((config) => esbuild.build(config))
    );
    // Run artifact
    if (FLAGS["run"]) {
        fork(CONFIGS["server"]().outfile, {stdio: "inherit" });
    }
})();