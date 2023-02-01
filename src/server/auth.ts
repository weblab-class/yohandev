import { OAuth2Client } from "google-auth-library";

import { User } from "./model";
// @ts-ignore
import { abilities } from "../assets/abilities.toml";

const CLIENT_ID = "825478233856-jub75iqs08474an082n9hptsj94tses3.apps.googleusercontent.com";
const client = new OAuth2Client(CLIENT_ID);

/**
 * Verify a "Sign in with Google" certificate and return its Google-issued ID.
 */
export async function verifyToken(token: string) {
    const ticket = await client.verifyIdToken({
        idToken: token,
        audience: CLIENT_ID,
    });
    const payload = ticket.getPayload();
    const userid = payload?.['sub'];

    return userid;
}

/**
 * Find a user from their Google-issued ID.
 */
export async function findOrCreateUser(gid: string) {
    const user = await User.findOne({ gid });
    if (user) {
        // Account already exists
        return user;
    }
    // Create new user
    return new User({
        name: "player1234",
        gid,
        // TODO: Until the collection aspect of the game is implemented,
        // every single ability is unlocked(yay!)
        deck: ["shotgun", "push", "time-freeze", "dual-gun"],
        unlocked: Object.keys(abilities).filter((n) => ![
            // not yet implemented
            "sniper", "grappling"
        ].includes(n)),
    }).save();
}