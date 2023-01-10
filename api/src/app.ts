import build, { PKG } from "./build";
import express from "express";
import chalk from "chalk";
import args from "args";

const ARGS = args
    .option("port", "The port on which the app will be running", 8000)
    .option("watch", "Enable/disable rebuilding the frontend on changes")
    .option("release", "TODO: Optimizes the WASM package.")
    .parse(process.argv);
const PORT = ARGS.port;

const app = express()
    .use(express.static(PKG));

// Start server:
app.listen(PORT);
console.clear();
console.log(chalk.bold.cyanBright(`Server running at http://localhost:${PORT}`));

// Build client. It takes care of its own logging.
build(ARGS.watch);