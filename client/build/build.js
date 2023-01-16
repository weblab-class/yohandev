/**
 * [build.js] Builds `../client/` into this directory.
 */
import { join } from "node:path";
import { build } from "esbuild";

import { WatchLog } from "../../shared/build/watchlog.js";
import { Cargo } from "../../shared/build/cargo.js";
import { ROOT } from "../../shared/build/util.js";

import parse from "mri";

const {
    watch,      // Enable/disable rebuilding `client/` on changes.
    release,    // TODO: Build for production.
    ...args
} = parse(process.argv.slice(2));

build({
    // Bundle
    entryPoints: [join(ROOT, "client/src/app.js")],
    outfile: join(ROOT, "client/build/app.js"),
    bundle: true,
    minify: true,
    sourcemap: true,
    plugins: [
        Cargo.plugin(),
        WatchLog.plugin({ name: "c" }),
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