import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import geckos, { iceServers } from "@geckos.io/server";
import bodyParser from "body-parser";
import mongoose from "mongoose";
import session from "express-session";
import express from "express";
import https from "node:https";
import http from "node:http";
import fs from "node:fs/promises";

// @ts-ignore
import args, { port } from "env";

import * as game from "./game";
import { api } from "./api";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const DB = `mongodb+srv://Yohan:${require("./db.txt")}@cluster0.r0khpr8.mongodb.net/?retryWrites=true&w=majority`;

// -- API --
const app = express()
    .use("/", express.static(join(ROOT, "build/pkg/")))
    .use("/assets", express.static(join(ROOT, "src/assets/")))
    .use(bodyParser.urlencoded({ extended: false }))
    .use(bodyParser.json())
    .use(session({
        secret: "session-secret",
        resave: false,
        saveUninitialized: false,
    }))
    .use("/api", api);
// -- HTTP(S) Server --
const server = args.https
    ? https.createServer({
        cert: await fs.readFile(join(ROOT, "src/certificates/cert.crt")),
        ca: await fs.readFile(join(ROOT, "src/certificates/ca.ca-bundle")),
        key: await fs.readFile(join(ROOT, "src/certificates/private.key")),
    }, app)
    : http.createServer(app);
// -- HTTP Rerouting --
if (https) {
    express()
        .get('*', (_, res) => res.redirect("https://boxbrawl.com"))
        .listen(80);
}
// -- Database --
const db = await mongoose
    .set("strictQuery", true)
    .connect(DB);
// -- UDP Socket --
const io = geckos({ iceServers });

io.addServer(server);
server.listen(port);
await game.init(io);

console.log("Success!");