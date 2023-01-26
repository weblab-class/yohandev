import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import geckos from "@geckos.io/server";
import bodyParser from "body-parser";
import mongoose from "mongoose";
import session from "express-session";
import express from "express";
import https from "node:https";
import http from "node:http";
import fs from "node:fs/promises";

// @ts-ignore
import args, { port } from "env";

import { game } from "../platform/node";
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
// -- Database --
const db = await mongoose
    .set("strictQuery", true)
    .connect(DB);
// -- UDP Socket --
const io = geckos();

io.addServer(server);
server.listen(port);

// Create game
await game(io)

console.log("Success!");