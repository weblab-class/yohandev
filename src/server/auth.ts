import { User } from "./model";

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