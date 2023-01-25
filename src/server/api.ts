import { Router } from "express";

import { verifyToken, findOrCreateUser } from "./auth";

export const api = Router();

api.post("/login", async (req, res) => {
    const id = await verifyToken(req.body.credential);
    const user = await findOrCreateUser(id!);

    res.send(user);
});