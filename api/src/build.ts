/**
 * [build.ts] Build script for the frontend in `www/`.
 */
import { Plugin, BuildOptions, build as esbuild } from "esbuild";
import { execa as exec } from "execa";
import { readFile } from "fs/promises";
import { dirname, resolve, join } from "path";
import { fileURLToPath } from "url";
import { default as loadingAnimation } from "ora";
import { default as chalk } from "chalk";

export const __dirname = dirname(fileURLToPath(import.meta.url));
export const WWW = join(__dirname, "../../www");
export const PKG = join(WWW, "build/");

/**
 * Build, bundle, and minify the frontend from `www/` into `www/build`.
 */
export default async function build(watch: boolean) {
    const BUILD_OPTIONS: BuildOptions = {
        // Bundle
        entryPoints: [join(WWW, "src/app.js")],
        bundle: true,
        minify: true,
        sourcemap: true,
        // target: ['chrome58', 'firefox57', 'safari11', 'edge16'],
        outfile: join(WWW, "build/app.js"),
        plugins: [
            Cargo.Plugin,
            WatchLog.Plugin,
        ],

        // Preact
        jsxFactory: 'h',
        jsxFragment: 'Fragment',
        jsxImportSource: 'preact',
        jsx: 'automatic',
        loader: {
            '.js': 'jsx',
        },

        // Rebuilding
        watch,
    };
    return esbuild(BUILD_OPTIONS);
}

/**
 * Wrapper over Cargo for compatibility with ESBuild.
 */
module Cargo {
    export const Plugin: Plugin = {
        name: "cargo",
        setup: (build) => {
            build.onResolve({ filter: /\.rs$/ }, async (args) => {
                const path = resolve(args.resolveDir, args.path);
                const manifest = await Cargo.manifestPath(path);
                // Resolve to the project's manifest.
                return {
                    path: manifest,
                    namespace: "cargo-stub",
                    errors: manifest ? [] : [{
                        pluginName: Cargo.Plugin.name,
                        text: `Could not find \`Cargo.toml\` in \`${path}\` or any parent directory.`
                    }]
                };
            });
            build.onResolve({ filter: /Cargo\.toml$/ }, (args) => {
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
            });
            build.onLoad({ filter: /.*/, namespace: "cargo-stub" }, (args) => ({
                // Virtual module code:
                contents: `
                    import wasm from "${args.path}";

                    export default async (env) => (
                        WebAssembly.instantiateStreaming(fetch(wasm), { env })
                            .then((obj) => obj.instance.exports)
                    );
                `
            }));
            build.onLoad({ filter: /.*/, namespace: "cargo-binary" }, async (args) => {
                const {
                    artifact,
                    errors,
                    warnings
                } = await Cargo.build(args.path);

                return {
                    contents: artifact && await readFile(artifact),
                    errors: errors.map((text) => ({ text, detail: { rendered: true } })),
                    warnings: warnings.map((text) =>({ text, detail: { rendered: true } })),
                    loader: "file",
                }
            });
        }
    };

    /**
     * Invokes `cargo build` from any path within the project.
     */
    export async function build(path: string) {
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
                .map((s) => <any>JSON.parse(s))
            );
        const artifact = messages.find((m) => m["reason"] === "compiler-artifact");
        const diagnostics = messages.filter((m) => m["reason"] === "compiler-message");

        return {
            artifact: <string|undefined>artifact?.filenames[0],
            errors: diagnostics
                .filter((m) => m["message"]["level"] === "error")
                .map((m) => <string>m["message"]["rendered"]),
            warnings: diagnostics
                .filter((m) => m["message"]["level"] === "warning")
                .map((m) => <string>m["message"]["rendered"]),
        };
    }

    /**
     * Reports the absolute path of the nearest `Cargo.toml`.
     */
    export async function manifestPath(path: string) {
        return exec("cargo", ["locate-project"], { cwd: dirname(path) })
            .then(({ stdout,exitCode }) => {
                return exitCode == 0 ? <string>JSON.parse(stdout)["root"] : undefined;
            });
    }
}

/**
 * Plugin to display pretty messages on the console for builds.
 */
module WatchLog {
    export const Plugin: Plugin = {
        name: 'log',
        setup: (build) => {
            const anim = loadingAnimation({
                text: "Building",
                color: "cyan"
            });
            // Override esbuild outputs.
            build.initialOptions.logLevel = "silent";

            build.onStart(() => {
                anim.start();
            });
            build.onEnd((res) => {
                if (res.errors.length !== 0) {
                    anim.fail("Build failed.");
                } else if (res.warnings.length !== 0) {
                    anim.warn("Build has warnings.");
                } else {
                    anim.succeed("Build succeeded.");
                }
                res.errors.concat(res.warnings).forEach((e) => {
                    // Skip message formatting.
                    if (e.detail?.rendered) {
                        console.error(e.text);
                        return;
                    }
                    console.error("\n", chalk.redBright.bold(e.text), "\n");
                    if (e.location) {
                        const { file, line, column, lineText, length } = e.location;
                        // Format the error:
                        console.error("   ", `${file}:${line}:${column}`);
                        console.error("      "
                            + chalk.whiteBright(line, "│ ")
                            + chalk.whiteBright(lineText.slice(0, column))
                            + chalk.greenBright(lineText.slice(column, column + length))
                            + chalk.whiteBright(lineText.slice(column + length))
                        );
                        console.error(
                            " ".repeat(line.toString().length + 6),
                            "╵",
                            " ".repeat(column - 1),
                            chalk.greenBright("^")
                        );
                    }
                });
            });
        }
    };
}