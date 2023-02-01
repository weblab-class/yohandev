import { Router } from "express";

import { verifyToken, findOrCreateUser } from "./auth";
import { StatsJoinGame, User } from "./model";
import { instance } from "./game";

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
    const uuid = req.body.uuid;
    // Default deck(not logged in)
    let deck = [
        "shotgun",
        "dual-gun",
        "assault-rifle",
        "shield",
    ];
    
    if (!uuid) {
        // Client sends its own WebRTC ID
        return res.status(400).send({
            reason: "No UUID!"
        });
    }
    if (user) {
        // Stats
        new StatsJoinGame({ gid: user.gid }).save();
        // Deck
        deck = user.deck;
    }
    instance.spawnPlayer(uuid, deck);
    res.send({});
});

api.post("/edit-deck", async (req, res) => {
    let user = req.session["user"];
    const { added, removed } = req.body;
    if (!user) {
        // No login -> OK
        user = req.session["user"] = {
            dummy: true,
            unlocked: [
                // Default deck:
                "shotgun",
                "dual-gun",
                "assault-rifle",
                "shield"
            ],
            deck: [
                // Default deck:
                "shotgun",
                "dual-gun",
                "assault-rifle",
                "shield"
            ]
        }
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
    if (!user.dummy) {
        await User.findByIdAndUpdate(user._id, { deck: user.deck });
    }
    res.send({});
});