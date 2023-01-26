import { Router } from "express";

import { verifyToken, findOrCreateUser } from "./auth";
import { StatsJoinGame, User } from "./model";

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

api.post("/edit-deck", async (req, res) => {
    const user = req.session["user"];
    const { added, removed } = req.body;
    if (!user) {
        // No login -> OK
        return res.send({});
    }
    // Cheating?!
    if (!user.unlocked.includes(added) || !user.unlocked.includes(removed)) {
        // TODO: server reconcillation
        return res.status(400).send("stop cheating");
    }
    user.deck = user.deck.map((id: string) => {
        // Swap into deck
        if (id === removed) {
            return added;
        }
        // Deck-deck reordering
        if (id === added) {
            return removed;
        }
        return id;
    });
    // Save to DB
    await User.findByIdAndUpdate(user._id, { deck: user.deck });
    res.send({});
});