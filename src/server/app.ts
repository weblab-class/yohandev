import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import geckos from "@geckos.io/server";
import express from "express";
import http from "node:http";

import { game } from "../platform/node";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

console.log(join(ROOT, "build/pkg/"));
const app = express()
    .use(express.static(join(ROOT, "build/pkg/")));
const server = http.createServer(app);
const io = geckos();

io.addServer(server);
server.listen(8000);

// Create game
game(io).then(() => console.log("Success!"));