import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import geckos from "@geckos.io/server";
import mongoose from "mongoose";
import express from "express";
import http from "node:http";

import { game } from "../platform/node";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const DB = `mongodb+srv://Yohan:${require("./db.txt")}@cluster0.r0khpr8.mongodb.net/?retryWrites=true&w=majority`;

// -- API --
const app = express()
    .use("/", express.static(join(ROOT, "build/pkg/")))
    .use("/assets/", express.static(join(ROOT, "src/assets/")));
// -- HTTP Server --
const server = http.createServer(app);
// -- Database --
const db = await mongoose
    .set("strictQuery", true)
    .connect(DB);
// -- UDP Socket --
const io = geckos();

io.addServer(server);
server.listen(8000);

// Create game
await game(io)

console.log("Success!");