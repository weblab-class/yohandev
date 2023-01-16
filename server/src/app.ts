import express from "express";
import chalk from "chalk";
import parse from "mri";

import { path } from "../../shared/build/util";

const {
    port=8000,  // The port on which the app will be running
    ...args
} = parse(process.argv.slice(2));

const app = express()
    .use(express.static(path("client/build/")));

// Start server:
app.listen(port);
console.log(chalk.bold.cyanBright(`Server running at http://localhost:${port}`));