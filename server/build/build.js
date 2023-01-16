/**
 * [build.js] Builds `../server/` into this directory.
 */
import { fork } from "node:child_process";
import { build } from "esbuild";

import { WatchLog } from "../../shared/build/watchlog.js";
import { Cargo } from "../../shared/build/cargo.js";
import { path } from "../../shared/build/util.js";

import parse from "mri";

const {
    watch,      // Enable/disable rebuilding `server/` on changes.
    run,        // Enable/disable (re)running `server/` after each build.
    release,    // TODO: Build for production.
    ...args
} = parse(process.argv.slice(2));

build({
    // Bundle
    entryPoints: [path("server/src/app.ts")],
    outfile: path("server/build/app.js"),
    bundle: true,
    plugins: [
        Cargo.plugin(),
        WatchLog.plugin({ name: "s" }),
    ],

    // Node
    format: "esm",
    packages: "external",
    platform: "node",

    // Rebuilding
    watch: watch && {
        onRebuild(err, _res) {
            // Skip on errors.
            if (run && !err) {
                Process.restart();
            }
        }
    },
}).finally((_) => {
    // Run the server.
    run && Process.restart();
});

const Process = {
    PATH: path("server/build/app.js"),
    /**
     * (Re)start the `server/` node.js process.
    */
    restart() {
        this.child?.kill();
        this.child = fork(this.PATH, [...args._], { stdio: 'inherit' });
    }
};