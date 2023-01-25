import { OAuth2Client } from "google-auth-library";

import { User } from "./model";

const CLIENT_ID = "825478233856-jub75iqs08474an082n9hptsj94tses3.apps.googleusercontent.com";
const client = new OAuth2Client(CLIENT_ID);

export async function verifyToken(token: string) {
    const ticket = await client.verifyIdToken({
        idToken: token,
        audience: CLIENT_ID,
    });
    const payload = ticket.getPayload();
    const userid = payload?.['sub'];
    
    console.log(JSON.stringify(payload));

    return userid;
}

export async function findOrCreateUser(id: string) {
    const user = await User.findOne({ name: id });
    if (user) {
        // Account already exists
        return user;
    }
    // Create new user
    return new User({
        name: id,
        deck: [],
        collection: [],
    }).save();
}