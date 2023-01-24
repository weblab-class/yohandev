import { Router } from "express";

import { findOrCreateUser } from "./auth";

export const api = Router();

api.post("/login", async (req, res) => {
    console.log(req.body);
    if (!req.body.id) {
        res.status(400).send({ reason: "Missing ID field!" });
        return;
    }
    const user = await findOrCreateUser(req.body.id);

    res.send(user);
});