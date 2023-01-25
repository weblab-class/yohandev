import { Router } from "express";

import { verifyToken, findOrCreateUser } from "./auth";
import { StatsJoinGame } from "./model";

export const api = Router();

api.post("/login", async (req, res) => {
    const id = await verifyToken(req.body.credential);
    if (!id) {
        return res.status(400).send("Bad login.");
    }
    const user = await findOrCreateUser(id);

    // Cache user session
    req.session["user"] = user;
    // Send user their game progress
    res.send(user);
});

api.post("/join-game", async (req, res) => {
    const user = req.session["user"];
    if (user) {
        new StatsJoinGame({ gid: user.gid }).save();
    }
    res.send({});
});