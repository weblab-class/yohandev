/**
 * [build.js] Builds `../client/` into this directory.
 */
import { join } from "node:path";
import { build } from "esbuild";

import { WatchLog } from "../../shared/build/watchlog.js";
import { Cargo } from "../../shared/build/cargo.js";
import { Argv } from "../../shared/build/argv.js";
import { path } from "../../shared/build/util.js";

import parse from "mri";

const {
    watch,      // Enable/disable rebuilding `client/` on changes.
    release,    // TODO: Build for production.
    ...args
} = parse(process.argv.slice(2));

build({
    // Bundle
    entryPoints: [path("client/src/app.js")],
    outfile: path("client/build/app.js"),
    bundle: true,
    minify: true,
    sourcemap: true,
    plugins: [
        Cargo.plugin("fetch"),
        WatchLog.plugin({ name: "c" }),
        Argv.plugin(),
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
    watch
});