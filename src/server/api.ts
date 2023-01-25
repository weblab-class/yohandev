import { Router } from "express";

import { verifyToken, findOrCreateUser } from "./auth";

export const api = Router();

api.post("/login", async (req, res) => {
    const id = await verifyToken(req.body.credential);
    if (!id) {
        return res.status(400).send("Bad login.");
    }
    const user = await findOrCreateUser(id);
    // Cache user session
    req.session["user"] = user;

    res.send(user);
});

api.post("/join-game", async (req, res) => {
    if (req.session["user"]) {
        console.log("play:", JSON.stringify(req.session["user"]));
    }
    res.send({});
});