import geckos from "@geckos.io/server";
import express from "express";
import http from "node:http";
import parse from "mri";

import { path } from "../../shared/build/util";

const {
    port=8000,  // The port on which the app will be running
    ...args
} = parse(process.argv.slice(2));

const app = express()
    .use(express.static(path("client/build/")));
const server = http.createServer(app);
const io = geckos();

io.addServer(server);
io.onConnection((channel) => {
    console.log("Received connection!");
    channel.onRaw((packet) => {
        console.log(`Received: ${new Int8Array(<ArrayBuffer>packet, 0, 1)}`);
    });
});
server.listen(port);