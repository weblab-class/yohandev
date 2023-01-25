import { model, Schema } from "mongoose";

export const User = model("stats-user", new Schema({
    name: String,           // Display username
    gid: String,            // Google-issued ID
    deck: [String],         // IDs of the cards in the user's deck
    unlocked: [String],     // IDs of the cards unlocked by the user
}));

export const StatsJoinGame = model("stats-join-game", new Schema({
    gid: String,            // Google-issued ID of the user
    timestamp: {            // When the stat was logged
        type: Date,
        default: Date.now
    },
}));

export const StatsDeath = model("stats-death", new Schema({
    gid: String,            // Google-issued ID of the user
    timestamp: {            // When the stat was logged
        type: Date,
        default: Date.now
    },
}));

export const StatsKill = model("stats-kill", new Schema({
    gid: String,            // Google-issued ID of the user
    timestamp: {            // When the stat was logged
        type: Date,
        default: Date.now
    },
}));