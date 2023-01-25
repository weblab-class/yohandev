import { useCallback, useState } from "preact/hooks"

import { AbilityCollection, AbilityDeck, AbilityIcon } from "./ability";
import { LoginButton } from "./login";
import { POST } from "../utils";
import "../styles/menu.css";

/**
 * Main menu component.
 */
export function Menu({ ...props }) {
    // User progress
    const [deck, setDeck] = useState(Array(4).fill(undefined));
    const [collection, setCollection] = useState([]);

    const onLogin = useCallback(({ id: _id, deck, unlocked }) => {
        setDeck(deck);
        setCollection(unlocked);
    });
    const onPlay = useCallback(() => POST("/api/join-game"), []);

    return (
        <div {...props}>
            {/* Black overlay */}
            <div
                class="w:100vw h:100vh"
                style="background: rgb(0, 0, 0, 0.2)"
            />
            {/* Floating window */}
            <div class="menu centered column">
                <div class="row">
                    <span class="logo">bbox</span>
                    <LoginButton onLogin={onLogin}/>
                </div>
                <div class="row">
                    <AbilityDeck deck={deck}/>
                    <div class="player-preview">

                    </div>
                </div>
                <div class="row">
                    <AbilityCollection collection={collection}/>
                    <div class="column">
                        <div class="player-stats">

                        </div>
                        <button class="play" onclick={onPlay}>
                            Play
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
}